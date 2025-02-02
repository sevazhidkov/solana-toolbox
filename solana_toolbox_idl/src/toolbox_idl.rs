use std::collections::HashMap;

use inflate::inflate_bytes_zlib;
use serde_json::from_str;
use serde_json::Map;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_account::ToolboxIdlProgramAccount;
use crate::toolbox_idl_program_error::ToolboxIdlProgramError;
use crate::toolbox_idl_program_instruction::ToolboxIdlProgramInstruction;
use crate::toolbox_idl_program_type::ToolboxIdlProgramType;
use crate::toolbox_idl_utils::idl_array_get_scoped_named_object_array_or_else;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_map_err_invalid_integer;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_object;
use crate::toolbox_idl_utils::idl_object_get_scoped_key_value_array;
use crate::toolbox_idl_utils::idl_pubkey_from_bytes_at;
use crate::toolbox_idl_utils::idl_slice_from_bytes;
use crate::toolbox_idl_utils::idl_u32_from_bytes_at;

// TODO - support docs on accounts/instructions/defs/etc ?
#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdl {
    pub program_types: HashMap<String, ToolboxIdlProgramType>,
    pub program_instructions: HashMap<String, ToolboxIdlProgramInstruction>,
    pub program_accounts: HashMap<String, ToolboxIdlProgramAccount>,
    pub program_errors: HashMap<u64, ToolboxIdlProgramError>,
}

impl ToolboxIdl {
    pub async fn get_for_program_id(
        endpoint: &mut ToolboxEndpoint,
        program_id: &Pubkey,
    ) -> Result<Option<ToolboxIdl>, ToolboxIdlError> {
        endpoint
            .get_account_data(&ToolboxIdl::find_for_program_id(program_id)?)
            .await?
            .map(|account_data| ToolboxIdl::try_from_data(&account_data))
            .transpose()
    }

    pub fn find_for_program_id(
        program_id: &Pubkey
    ) -> Result<Pubkey, ToolboxIdlError> {
        let base = Pubkey::find_program_address(&[], program_id).0;
        Pubkey::create_with_seed(&base, "anchor:idl", program_id)
            .map_err(ToolboxIdlError::Pubkey)
    }

    pub fn try_from_data(data: &[u8]) -> Result<ToolboxIdl, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let discriminator = ToolboxIdl::DISCRIMINATOR;
        if !data.starts_with(discriminator) {
            return Err(ToolboxIdlError::InvalidDiscriminator {
                expected: discriminator.to_vec(),
                found: data.to_vec(),
            });
        }
        let authority_offset = discriminator.len();
        let authority = idl_pubkey_from_bytes_at(
            data,
            authority_offset,
            &breadcrumbs.as_val("authority"),
        )?;
        let length_offset =
            authority_offset + std::mem::size_of_val(&authority);
        let length = idl_u32_from_bytes_at(
            data,
            length_offset,
            &breadcrumbs.as_val("length"),
        )?;
        let content_offset = length_offset + std::mem::size_of_val(&length);
        let content = idl_slice_from_bytes(
            data,
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
        ToolboxIdl::try_from_str(&content_decoded)
    }

    pub fn try_from_str(content: &str) -> Result<ToolboxIdl, ToolboxIdlError> {
        ToolboxIdl::try_from_value(
            &from_str::<Value>(content).map_err(ToolboxIdlError::SerdeJson)?,
        )
    }

    pub fn try_from_value(
        value: &Value
    ) -> Result<ToolboxIdl, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let idl_root = idl_as_object_or_else(value, &breadcrumbs.as_idl("$"))?;
        let program_types =
            ToolboxIdl::try_parse_program_types(idl_root, breadcrumbs)?;
        let program_instructions = ToolboxIdl::try_parse_program_instructions(
            &program_types,
            idl_root,
            breadcrumbs,
        )?;
        let program_accounts = ToolboxIdl::try_parse_program_accounts(
            &program_types,
            idl_root,
            breadcrumbs,
        )?;
        let program_errors =
            ToolboxIdl::try_parse_program_errors(idl_root, breadcrumbs)?;
        Ok(ToolboxIdl {
            program_types,
            program_instructions,
            program_accounts,
            program_errors,
        })
    }

    fn try_parse_program_types(
        idl_root: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<HashMap<String, ToolboxIdlProgramType>, ToolboxIdlError> {
        // TODO - generic way for object/array scoped machanism
        let mut program_types = HashMap::new();
        if let Some(idl_types) = idl_object_get_key_as_object(idl_root, "types")
        {
            for (idl_type_name, idl_type, breadcrumbs) in
                idl_object_get_scoped_key_value_array(idl_types, breadcrumbs)?
            {
                let idl_type_object =
                    idl_as_object_or_else(idl_type, &breadcrumbs.idl())?;
                program_types.insert(
                    idl_type_name.to_string(),
                    ToolboxIdlProgramType::try_parse(
                        idl_type_name,
                        idl_type_object,
                        &breadcrumbs,
                    )?,
                );
            }
        }
        if let Some(idl_types) = idl_object_get_key_as_array(idl_root, "types")
        {
            for (idl_type_name, idl_type_object, breadcrumbs) in
                idl_array_get_scoped_named_object_array_or_else(
                    idl_types,
                    breadcrumbs,
                )?
            {
                program_types.insert(
                    idl_type_name.to_string(),
                    ToolboxIdlProgramType::try_parse(
                        idl_type_name,
                        idl_type_object,
                        &breadcrumbs,
                    )?,
                );
            }
        }
        Ok(program_types)
    }

    fn try_parse_program_instructions(
        program_types: &HashMap<String, ToolboxIdlProgramType>,
        idl_root: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<HashMap<String, ToolboxIdlProgramInstruction>, ToolboxIdlError>
    {
        let mut program_instructions = HashMap::new();
        if let Some(idl_instructions) =
            idl_object_get_key_as_object(idl_root, "instructions")
        {
            for (idl_instruction_name, idl_instruction_value, breadcrumbs) in
                idl_object_get_scoped_key_value_array(
                    idl_instructions,
                    breadcrumbs,
                )?
            {
                let idl_instruction_object = idl_as_object_or_else(
                    idl_instruction_value,
                    &breadcrumbs.idl(),
                )?;
                program_instructions.insert(
                    idl_instruction_name.to_string(),
                    ToolboxIdlProgramInstruction::try_parse(
                        program_types,
                        idl_instruction_name,
                        idl_instruction_object,
                        &breadcrumbs,
                    )?,
                );
            }
        }
        if let Some(idl_instructions) =
            idl_object_get_key_as_array(idl_root, "instructions")
        {
            for (idl_instruction_name, idl_instruction_object, breadcrumbs) in
                idl_array_get_scoped_named_object_array_or_else(
                    idl_instructions,
                    breadcrumbs,
                )?
            {
                program_instructions.insert(
                    idl_instruction_name.to_string(),
                    ToolboxIdlProgramInstruction::try_parse(
                        program_types,
                        idl_instruction_name,
                        idl_instruction_object,
                        &breadcrumbs,
                    )?,
                );
            }
        }
        Ok(program_instructions)
    }

    fn try_parse_program_accounts(
        program_types: &HashMap<String, ToolboxIdlProgramType>,
        idl_root: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<HashMap<String, ToolboxIdlProgramAccount>, ToolboxIdlError>
    {
        let mut program_accounts = HashMap::new();
        if let Some(idl_accounts) =
            idl_object_get_key_as_object(idl_root, "accounts")
        {
            for (idl_account_name, idl_account, breadcrumbs) in
                idl_object_get_scoped_key_value_array(
                    idl_accounts,
                    breadcrumbs,
                )?
            {
                let idl_account =
                    idl_as_object_or_else(idl_account, &breadcrumbs.idl())?;
                program_accounts.insert(
                    idl_account_name.to_string(),
                    ToolboxIdlProgramAccount::try_parse(
                        program_types,
                        idl_account_name,
                        idl_account,
                        &breadcrumbs,
                    )?,
                );
            }
        }
        if let Some(idl_accounts) =
            idl_object_get_key_as_array(idl_root, "accounts")
        {
            for (idl_account_name, idl_account, breadcrumbs) in
                idl_array_get_scoped_named_object_array_or_else(
                    idl_accounts,
                    breadcrumbs,
                )?
            {
                program_accounts.insert(
                    idl_account_name.to_string(),
                    ToolboxIdlProgramAccount::try_parse(
                        program_types,
                        idl_account_name,
                        idl_account,
                        &breadcrumbs,
                    )?,
                );
            }
        }
        Ok(program_accounts)
    }

    fn try_parse_program_errors(
        idl_root: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<HashMap<u64, ToolboxIdlProgramError>, ToolboxIdlError> {
        let mut program_errors = HashMap::new();
        if let Some(idl_errors) =
            idl_object_get_key_as_object(idl_root, "errors")
        {
            for (idl_error_name, idl_error, breadcrumbs) in
                idl_object_get_scoped_key_value_array(idl_errors, breadcrumbs)?
            {
                if let Some(idl_error_code) = idl_error.as_u64() {
                    program_errors.insert(
                        idl_error_code,
                        ToolboxIdlProgramError {
                            code: idl_error_code,
                            name: idl_error_name.to_string(),
                            msg: "".to_string(),
                        },
                    );
                } else {
                    let idl_error =
                        idl_as_object_or_else(idl_error, &breadcrumbs.idl())?;
                    let program_error = ToolboxIdlProgramError::try_parse(
                        idl_error_name,
                        idl_error,
                        &breadcrumbs,
                    )?;
                    program_errors.insert(program_error.code, program_error);
                }
            }
        }
        if let Some(idl_errors_array) =
            idl_object_get_key_as_array(idl_root, "errors")
        {
            for (idl_error_name, idl_error, breadcrumbs) in
                idl_array_get_scoped_named_object_array_or_else(
                    idl_errors_array,
                    breadcrumbs,
                )?
            {
                let program_error = ToolboxIdlProgramError::try_parse(
                    idl_error_name,
                    idl_error,
                    &breadcrumbs,
                )?;
                program_errors.insert(program_error.code, program_error);
            }
        }
        Ok(program_errors)
    }
}
