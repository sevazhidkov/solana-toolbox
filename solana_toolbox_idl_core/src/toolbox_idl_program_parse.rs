use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use inflate::inflate_bytes_zlib;
use serde_json::from_str;
use serde_json::Map;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_account::ToolboxIdlAccount;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_event::ToolboxIdlEvent;
use crate::toolbox_idl_instruction::ToolboxIdlInstruction;
use crate::toolbox_idl_program::ToolboxIdlProgram;
use crate::toolbox_idl_program::ToolboxIdlProgramMetadata;
use crate::toolbox_idl_typedef::ToolboxIdlTypedef;
use crate::toolbox_idl_utils::idl_convert_to_snake_case;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_object;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_slice_from_bytes;
use crate::toolbox_idl_utils::idl_u32_from_bytes_at;
use crate::toolbox_idl_utils::idl_value_as_object_or_else;

impl ToolboxIdlProgram {
    pub const DISCRIMINATOR: &[u8] =
        &[0x18, 0x46, 0x62, 0xBF, 0x3A, 0x90, 0x7B, 0x9E];

    pub fn try_parse_from_account_data(
        account_data: &[u8],
    ) -> Result<ToolboxIdlProgram> {
        let discriminator = ToolboxIdlProgram::DISCRIMINATOR;
        if !account_data.starts_with(discriminator) {
            return Err(anyhow!(
                "Invalid IDL account discriminator: expected: {:?}, found: {:?}",
                discriminator,
                account_data
            ));
        }
        let length =
            idl_u32_from_bytes_at(account_data, 40).context("Read Length")?;
        let content =
            idl_slice_from_bytes(account_data, 44, usize::try_from(length)?)
                .context("Read Content")?;
        let content_encoded = inflate_bytes_zlib(content).map_err(|error| {
            anyhow!("Could not decompress idl data: {}", error)
        })?;
        let content_decoded =
            String::from_utf8(content_encoded).context("Decode Content")?;
        ToolboxIdlProgram::try_parse_from_str(&content_decoded)
            .context("Parse Content")
    }

    pub fn try_parse_from_str(content: &str) -> Result<ToolboxIdlProgram> {
        ToolboxIdlProgram::try_parse(
            &from_str::<Value>(content).context("Parse JSON")?,
        )
    }

    pub fn try_parse(value: &Value) -> Result<ToolboxIdlProgram> {
        let idl_root = idl_value_as_object_or_else(value).context("Root")?;
        let metadata = ToolboxIdlProgram::try_parse_metadata(idl_root)?;
        let typedefs = ToolboxIdlProgram::try_parse_typedefs(idl_root)
            .context("Parse Types")?;
        let accounts =
            ToolboxIdlProgram::try_parse_accounts(idl_root, &typedefs)
                .context("Parse Accounts")?;
        let instructions = ToolboxIdlProgram::try_parse_instructions(
            idl_root, &accounts, &typedefs,
        )
        .context("Parse Instructions")?;
        let events = ToolboxIdlProgram::try_parse_events(idl_root, &typedefs)
            .context("Parse Events")?;
        let errors = ToolboxIdlProgram::try_parse_errors(idl_root)
            .context("Parse Errors")?;
        Ok(ToolboxIdlProgram {
            metadata,
            typedefs,
            accounts,
            instructions,
            errors,
            events,
        })
    }

    fn try_parse_metadata(
        idl_root: &Map<String, Value>,
    ) -> Result<ToolboxIdlProgramMetadata> {
        let mut metadata =
            ToolboxIdlProgram::try_parse_metadata_object(idl_root)?;
        if let Some(idl_metadata) =
            idl_object_get_key_as_object(idl_root, "metadata")
        {
            let metadata_inner =
                ToolboxIdlProgram::try_parse_metadata_object(idl_metadata)?;
            metadata.address = metadata_inner.address.or(metadata.address);
            metadata.name = metadata_inner.name.or(metadata.name);
            metadata.description =
                metadata_inner.description.or(metadata.description);
            metadata.docs = metadata_inner.docs.or(metadata.docs);
            metadata.version = metadata_inner.version.or(metadata.version);
            metadata.spec = metadata_inner.spec.or(metadata.spec);
        }
        Ok(metadata)
    }

    fn try_parse_metadata_object(
        idl_object: &Map<String, Value>,
    ) -> Result<ToolboxIdlProgramMetadata> {
        Ok(ToolboxIdlProgramMetadata {
            address: idl_object_get_key_as_str(idl_object, "address")
                .map(Pubkey::from_str)
                .transpose()?,
            name: idl_object_get_key_as_str(idl_object, "name")
                .map(String::from),
            description: idl_object_get_key_as_str(idl_object, "description")
                .map(String::from),
            docs: idl_object.get("docs").cloned(),
            version: idl_object_get_key_as_str(idl_object, "version")
                .map(String::from),
            spec: idl_object_get_key_as_str(idl_object, "spec")
                .map(String::from),
        })
    }

    fn try_parse_typedefs(
        idl_root: &Map<String, Value>,
    ) -> Result<HashMap<String, Arc<ToolboxIdlTypedef>>> {
        let mut typedefs = HashMap::new();
        for (idl_typedef_name, idl_typedef) in
            ToolboxIdlProgram::root_collection_scoped_named_values(
                idl_root, "types",
            )?
        {
            typedefs.insert(
                idl_typedef_name.to_string(),
                ToolboxIdlTypedef::try_parse(idl_typedef_name, idl_typedef)
                    .with_context(|| {
                        format!("Parse Typedef: {}", idl_typedef_name)
                    })?
                    .into(),
            );
        }
        Ok(typedefs)
    }

    fn try_parse_accounts(
        idl_root: &Map<String, Value>,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<HashMap<String, Arc<ToolboxIdlAccount>>> {
        let mut accounts = HashMap::new();
        for (idl_account_name, idl_account) in
            ToolboxIdlProgram::root_collection_scoped_named_values(
                idl_root, "accounts",
            )?
        {
            accounts.insert(
                idl_account_name.to_string(),
                ToolboxIdlAccount::try_parse(
                    idl_account_name,
                    idl_account,
                    typedefs,
                )
                .with_context(|| {
                    format!("Parse Account: {}", idl_account_name)
                })?
                .into(),
            );
        } // TODO - support for extracting typedefs from accounts for re-export to new IDL
        Ok(accounts)
    }

    fn try_parse_instructions(
        idl_root: &Map<String, Value>,
        accounts: &HashMap<String, Arc<ToolboxIdlAccount>>,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<HashMap<String, Arc<ToolboxIdlInstruction>>> {
        let mut instructions = HashMap::new();
        for (idl_instruction_name, idl_instruction) in
            ToolboxIdlProgram::root_collection_scoped_named_values(
                idl_root,
                "instructions",
            )?
        {
            let idl_instruction_name =
                idl_convert_to_snake_case(idl_instruction_name);
            instructions.insert(
                idl_instruction_name.to_string(),
                ToolboxIdlInstruction::try_parse(
                    &idl_instruction_name,
                    idl_instruction,
                    accounts,
                    typedefs,
                )
                .with_context(|| {
                    format!("Parse Instruction: {}", idl_instruction_name)
                })?
                .into(),
            );
        }
        Ok(instructions)
    }

    fn try_parse_events(
        idl_root: &Map<String, Value>,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<HashMap<String, Arc<ToolboxIdlEvent>>> {
        let mut events = HashMap::new();
        for (idl_event_name, idl_event) in
            ToolboxIdlProgram::root_collection_scoped_named_values(
                idl_root, "events",
            )?
        {
            events.insert(
                idl_event_name.to_string(),
                ToolboxIdlEvent::try_parse(idl_event_name, idl_event, typedefs)
                    .with_context(|| {
                        format!("Parse Event: {}", idl_event_name)
                    })?
                    .into(),
            );
        }
        Ok(events)
    }

    fn try_parse_errors(
        idl_root: &Map<String, Value>,
    ) -> Result<HashMap<String, Arc<ToolboxIdlError>>> {
        let mut errors = HashMap::new();
        for (idl_error_name, idl_error) in
            ToolboxIdlProgram::root_collection_scoped_named_values(
                idl_root, "errors",
            )?
        {
            errors.insert(
                idl_error_name.to_string(),
                ToolboxIdlError::try_parse(idl_error_name, idl_error)
                    .with_context(|| {
                        format!("Parse Error: {}", idl_error_name)
                    })?
                    .into(),
            );
        }
        Ok(errors)
    }

    fn root_collection_scoped_named_values<'a>(
        idl_root: &'a Map<String, Value>,
        collection_key: &str,
    ) -> Result<Vec<(&'a str, &'a Value)>> {
        let mut collection_scoped_named_objects = vec![];
        if let Some(idl_collection) =
            idl_object_get_key_as_object(idl_root, collection_key)
        {
            for (idl_collection_item_key, idl_collection_item) in idl_collection
            {
                collection_scoped_named_objects.push((
                    idl_collection_item_key.as_str(),
                    idl_collection_item,
                ));
            }
        }
        if let Some(idl_collection) =
            idl_object_get_key_as_array(idl_root, collection_key)
        {
            for (index, idl_collection_item) in
                idl_collection.iter().enumerate()
            {
                let idl_collection_item_name = idl_object_get_key_as_str(
                    idl_value_as_object_or_else(idl_collection_item)
                        .context(index)?,
                    "name",
                )
                .context(index)?;
                collection_scoped_named_objects
                    .push((idl_collection_item_name, idl_collection_item));
            }
        }
        Ok(collection_scoped_named_objects)
    }
}
