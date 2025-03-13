use std::collections::HashMap;

use inflate::inflate_bytes_zlib;
use serde_json::from_str;
use serde_json::Map;
use serde_json::Value;
use solana_sdk::account::Account;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_account::ToolboxIdlProgramAccount;
use crate::toolbox_idl_program_error::ToolboxIdlProgramError;
use crate::toolbox_idl_program_instruction::ToolboxIdlProgramInstruction;
use crate::toolbox_idl_program_typedef::ToolboxIdlProgramTypedef;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_iter_get_scoped_values;
use crate::toolbox_idl_utils::idl_map_err_invalid_integer;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_object;
use crate::toolbox_idl_utils::idl_pubkey_from_bytes_at;
use crate::toolbox_idl_utils::idl_slice_from_bytes;
use crate::toolbox_idl_utils::idl_u32_from_bytes_at;
use crate::toolbox_idl_utils::idl_value_as_str_or_object_with_name_as_str_or_else;
use crate::ToolboxIdlProgramRoot;

impl ToolboxIdlProgramRoot {
    pub fn try_from_account(
        account: &Account,
    ) -> Result<ToolboxIdlProgramRoot, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let discriminator = ToolboxIdlProgramRoot::DISCRIMINATOR;
        if !account.data.starts_with(discriminator) {
            return Err(ToolboxIdlError::InvalidDiscriminator {
                expected: discriminator.to_vec(),
                found: account.data.to_vec(),
            });
        }
        let authority_offset = discriminator.len();
        let authority = idl_pubkey_from_bytes_at(
            &account.data,
            authority_offset,
            &breadcrumbs.as_val("authority"),
        )?;
        let length_offset =
            authority_offset + std::mem::size_of_val(&authority);
        let length = idl_u32_from_bytes_at(
            &account.data,
            length_offset,
            &breadcrumbs.as_val("length"),
        )?;
        let content_offset = length_offset + std::mem::size_of_val(&length);
        let content = idl_slice_from_bytes(
            &account.data,
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
        ToolboxIdlProgramRoot::try_parse_from_str(&content_decoded)
    }

    pub fn try_parse_from_str(
        content: &str,
    ) -> Result<ToolboxIdlProgramRoot, ToolboxIdlError> {
        ToolboxIdlProgramRoot::try_parse_from_value(
            &from_str::<Value>(content).map_err(ToolboxIdlError::SerdeJson)?,
        )
    }

    pub fn try_parse_from_value(
        value: &Value,
    ) -> Result<ToolboxIdlProgramRoot, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let idl_root = idl_as_object_or_else(value, &breadcrumbs.as_idl("$"))?;
        let program_typedefs =
            ToolboxIdlProgramRoot::try_parse_program_typedefs(
                idl_root,
                breadcrumbs,
            )?;
        let program_instructions =
            ToolboxIdlProgramRoot::try_parse_program_instructions(
                &program_typedefs,
                idl_root,
                breadcrumbs,
            )?;
        let program_accounts =
            ToolboxIdlProgramRoot::try_parse_program_accounts(
                &program_typedefs,
                idl_root,
                breadcrumbs,
            )?;
        let program_errors = ToolboxIdlProgramRoot::try_parse_program_errors(
            idl_root,
            breadcrumbs,
        )?;
        Ok(ToolboxIdlProgramRoot {
            typedefs: program_typedefs,
            instructions: program_instructions,
            accounts: program_accounts,
            errors: program_errors,
        })
    }

    fn try_parse_program_typedefs(
        idl_root: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<HashMap<String, ToolboxIdlProgramTypedef>, ToolboxIdlError>
    {
        let mut program_typedefs = HashMap::new();
        for (idl_type_name, idl_type, breadcrumbs) in
            ToolboxIdlProgramRoot::root_collection_scoped_named_values(
                idl_root,
                "types",
                breadcrumbs,
            )?
        {
            program_typedefs.insert(
                idl_type_name.to_string(),
                ToolboxIdlProgramTypedef::try_parse(
                    idl_type_name,
                    idl_type,
                    &breadcrumbs,
                )?,
            );
        }
        Ok(program_typedefs)
    }

    fn try_parse_program_instructions(
        program_typedefs: &HashMap<String, ToolboxIdlProgramTypedef>,
        idl_root: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<HashMap<String, ToolboxIdlProgramInstruction>, ToolboxIdlError>
    {
        let mut program_instructions = HashMap::new();
        for (idl_instruction_name, idl_instruction, breadcrumbs) in
            ToolboxIdlProgramRoot::root_collection_scoped_named_values(
                idl_root,
                "instructions",
                breadcrumbs,
            )?
        {
            program_instructions.insert(
                idl_instruction_name.to_string(),
                ToolboxIdlProgramInstruction::try_parse(
                    idl_instruction_name,
                    idl_instruction,
                    program_typedefs,
                    &breadcrumbs,
                )?,
            );
        }
        Ok(program_instructions)
    }

    fn try_parse_program_accounts(
        program_typedefs: &HashMap<String, ToolboxIdlProgramTypedef>,
        idl_root: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<HashMap<String, ToolboxIdlProgramAccount>, ToolboxIdlError>
    {
        let mut program_accounts = HashMap::new();
        for (idl_account_name, idl_account, breadcrumbs) in
            ToolboxIdlProgramRoot::root_collection_scoped_named_values(
                idl_root,
                "accounts",
                breadcrumbs,
            )?
        {
            program_accounts.insert(
                idl_account_name.to_string(),
                ToolboxIdlProgramAccount::try_parse(
                    idl_account_name,
                    idl_account,
                    program_typedefs,
                    &breadcrumbs,
                )?,
            );
        }
        Ok(program_accounts)
    }

    fn try_parse_program_errors(
        idl_root: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<HashMap<String, ToolboxIdlProgramError>, ToolboxIdlError> {
        let mut program_errors = HashMap::new();
        for (idl_error_name, idl_error, breadcrumbs) in
            ToolboxIdlProgramRoot::root_collection_scoped_named_values(
                idl_root,
                "errors",
                breadcrumbs,
            )?
        {
            program_errors.insert(
                idl_error_name.to_string(),
                ToolboxIdlProgramError::try_parse(
                    idl_error_name,
                    idl_error,
                    &breadcrumbs,
                )?,
            );
        }
        Ok(program_errors)
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
