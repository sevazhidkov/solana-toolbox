use std::collections::HashMap;
use std::sync::Arc;

use inflate::inflate_bytes_zlib;
use serde_json::from_str;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_account::ToolboxIdlAccount;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_instruction::ToolboxIdlInstruction;
use crate::toolbox_idl_program::ToolboxIdlProgram;
use crate::toolbox_idl_program::ToolboxIdlProgramMetadata;
use crate::toolbox_idl_transaction_error::ToolboxIdlTransactionError;
use crate::toolbox_idl_typedef::ToolboxIdlTypedef;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_convert_to_type_name;
use crate::toolbox_idl_utils::idl_iter_get_scoped_values;
use crate::toolbox_idl_utils::idl_map_err_invalid_integer;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_object;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_pubkey_from_bytes_at;
use crate::toolbox_idl_utils::idl_slice_from_bytes;
use crate::toolbox_idl_utils::idl_u32_from_bytes_at;
use crate::toolbox_idl_utils::idl_value_as_str_or_object_with_name_as_str_or_else;

impl ToolboxIdlProgram {
    pub const DISCRIMINATOR: &[u8] =
        &[0x18, 0x46, 0x62, 0xBF, 0x3A, 0x90, 0x7B, 0x9E];

    pub fn try_parse_from_account_data(
        account_data: &[u8],
    ) -> Result<ToolboxIdlProgram, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let discriminator = ToolboxIdlProgram::DISCRIMINATOR;
        if !account_data.starts_with(discriminator) {
            return Err(ToolboxIdlError::InvalidDiscriminator {
                expected: discriminator.to_vec(),
                found: account_data.to_vec(),
            });
        }
        // TODO (FAR) - this could use a struct deserialization ?
        let authority_offset = discriminator.len();
        let authority = idl_pubkey_from_bytes_at(
            account_data,
            authority_offset,
            &breadcrumbs.as_val("authority"),
        )?;
        let length_offset =
            authority_offset + std::mem::size_of_val(&authority);
        let length = idl_u32_from_bytes_at(
            account_data,
            length_offset,
            &breadcrumbs.as_val("length"),
        )?;
        let content_offset = length_offset + std::mem::size_of_val(&length);
        let content = idl_slice_from_bytes(
            account_data,
            content_offset,
            idl_map_err_invalid_integer(
                usize::try_from(length),
                &breadcrumbs.as_val("length"),
            )?,
            &breadcrumbs.as_val("content"),
        )?;
        let content_encoded =
            inflate_bytes_zlib(content).map_err(ToolboxIdlError::Inflate)?;
        let content_decoded =
            String::from_utf8(content_encoded).map_err(|err| {
                ToolboxIdlError::InvalidString {
                    parsing: err,
                    context: breadcrumbs.as_val("content"),
                }
            })?;
        ToolboxIdlProgram::try_parse_from_str(&content_decoded)
    }

    pub fn try_parse_from_str(
        content: &str,
    ) -> Result<ToolboxIdlProgram, ToolboxIdlError> {
        ToolboxIdlProgram::try_parse_from_value(
            &from_str::<Value>(content).map_err(ToolboxIdlError::SerdeJson)?,
        )
    }

    pub fn try_parse_from_value(
        value: &Value,
    ) -> Result<ToolboxIdlProgram, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let idl_root = idl_as_object_or_else(value, &breadcrumbs.as_idl("$"))?;
        let metadata = ToolboxIdlProgram::try_parse_metadata(idl_root);
        let typedefs =
            ToolboxIdlProgram::try_parse_typedefs(idl_root, breadcrumbs)?;
        let instructions = ToolboxIdlProgram::try_parse_instructions(
            &typedefs,
            idl_root,
            breadcrumbs,
        )?;
        let accounts = ToolboxIdlProgram::try_parse_accounts(
            &typedefs,
            idl_root,
            breadcrumbs,
        )?;
        let errors =
            ToolboxIdlProgram::try_parse_errors(idl_root, breadcrumbs)?;
        Ok(ToolboxIdlProgram {
            metadata,
            instructions,
            accounts,
            errors,
            typedefs,
        })
    }

    fn try_parse_metadata(
        idl_root: &Map<String, Value>,
    ) -> ToolboxIdlProgramMetadata {
        let mut metadata =
            ToolboxIdlProgram::try_parse_metadata_object(idl_root);
        if let Some(idl_metadata) =
            idl_object_get_key_as_object(idl_root, "metadata")
        {
            let metadata_inner =
                ToolboxIdlProgram::try_parse_metadata_object(idl_metadata);
            metadata.name = metadata_inner.name.or(metadata.name);
            metadata.version = metadata_inner.version.or(metadata.version);
            metadata.description =
                metadata_inner.description.or(metadata.description);
            metadata.docs = metadata_inner.docs.or(metadata.docs);
        }
        metadata
    }

    fn try_parse_metadata_object(
        idl_object: &Map<String, Value>,
    ) -> ToolboxIdlProgramMetadata {
        ToolboxIdlProgramMetadata {
            name: idl_object_get_key_as_str(idl_object, "name")
                .map(ToolboxIdlProgram::sanitize_name),
            version: idl_object_get_key_as_str(idl_object, "version")
                .map(String::from),
            description: idl_object_get_key_as_str(idl_object, "description")
                .map(String::from),
            docs: idl_object.get("docs").cloned(),
        }
    }

    fn try_parse_typedefs(
        idl_root: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<HashMap<String, Arc<ToolboxIdlTypedef>>, ToolboxIdlError> {
        let mut typedefs = HashMap::new();
        for (idl_typedef_name, idl_typedef, breadcrumbs) in
            ToolboxIdlProgram::root_collection_scoped_named_values(
                idl_root,
                "types",
                breadcrumbs,
            )?
        {
            typedefs.insert(
                idl_typedef_name.to_string(),
                ToolboxIdlTypedef::try_parse(
                    idl_typedef_name,
                    idl_typedef,
                    &breadcrumbs,
                )?
                .into(),
            );
        }
        Ok(typedefs)
    }

    fn try_parse_instructions(
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
        idl_root: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<HashMap<String, Arc<ToolboxIdlInstruction>>, ToolboxIdlError>
    {
        let mut instructions = HashMap::new();
        for (idl_instruction_name, idl_instruction, breadcrumbs) in
            ToolboxIdlProgram::root_collection_scoped_named_values(
                idl_root,
                "instructions",
                breadcrumbs,
            )?
        {
            // TODO (FAR) - support instructions with no name (use index ?)
            let idl_instruction_name =
                ToolboxIdlInstruction::sanitize_name(idl_instruction_name);
            instructions.insert(
                idl_instruction_name.to_string(),
                ToolboxIdlInstruction::try_parse(
                    &idl_instruction_name,
                    idl_instruction,
                    typedefs,
                    &breadcrumbs,
                )?
                .into(),
            );
        }
        Ok(instructions)
    }

    fn try_parse_accounts(
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
        idl_root: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<HashMap<String, Arc<ToolboxIdlAccount>>, ToolboxIdlError> {
        let mut accounts = HashMap::new();
        for (idl_account_name, idl_account, breadcrumbs) in
            ToolboxIdlProgram::root_collection_scoped_named_values(
                idl_root,
                "accounts",
                breadcrumbs,
            )?
        {
            let idl_account_name =
                ToolboxIdlAccount::sanitize_name(idl_account_name);
            accounts.insert(
                idl_account_name.to_string(),
                ToolboxIdlAccount::try_parse(
                    &idl_account_name,
                    idl_account,
                    typedefs,
                    &breadcrumbs,
                )?
                .into(),
            );
        }
        Ok(accounts)
    }

    fn try_parse_errors(
        idl_root: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<HashMap<String, Arc<ToolboxIdlTransactionError>>, ToolboxIdlError>
    {
        let mut errors = HashMap::new();
        for (idl_error_name, idl_error, breadcrumbs) in
            ToolboxIdlProgram::root_collection_scoped_named_values(
                idl_root,
                "errors",
                breadcrumbs,
            )?
        {
            let idl_error_name = idl_convert_to_type_name(idl_error_name);
            errors.insert(
                idl_error_name.to_string(),
                ToolboxIdlTransactionError::try_parse(
                    &idl_error_name,
                    idl_error,
                    &breadcrumbs,
                )?
                .into(),
            );
        }
        Ok(errors)
    }

    fn root_collection_scoped_named_values<'a>(
        idl_root: &'a Map<String, Value>,
        collection_key: &str,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Vec<(&'a str, &'a Value, ToolboxIdlBreadcrumbs)>, ToolboxIdlError>
    {
        let mut collection_scoped_named_objects = vec![];
        if let Some(idl_collection) =
            idl_object_get_key_as_object(idl_root, collection_key)
        {
            for (idl_collection_item_key, idl_collection_item) in idl_collection
            {
                collection_scoped_named_objects.push((
                    idl_collection_item_key.as_str(),
                    idl_collection_item,
                    breadcrumbs.with_idl(idl_collection_item_key),
                ));
            }
        }
        if let Some(idl_collection) =
            idl_object_get_key_as_array(idl_root, collection_key)
        {
            for (_, idl_collection_item, breadcrumbs) in
                idl_iter_get_scoped_values(idl_collection, breadcrumbs)?
            {
                let idl_collection_item_name =
                    idl_value_as_str_or_object_with_name_as_str_or_else(
                        idl_collection_item,
                        &breadcrumbs.idl(),
                    )?;
                collection_scoped_named_objects.push((
                    idl_collection_item_name,
                    idl_collection_item,
                    breadcrumbs.with_idl(idl_collection_item_name),
                ));
            }
        }
        Ok(collection_scoped_named_objects)
    }
}
