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
use crate::toolbox_idl_program_typedef::ToolboxIdlProgramTypedef;
use crate::toolbox_idl_utils::idl_array_get_scoped_named_object_array_or_else;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_object;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;
use crate::toolbox_idl_utils::idl_object_get_scoped_key_value_array;
use crate::toolbox_idl_utils::idl_pubkey_from_bytes_at;
use crate::toolbox_idl_utils::idl_slice_from_bytes;
use crate::toolbox_idl_utils::idl_u32_from_bytes_at;

// TODO - support docs on accounts/instructions/typedefs/etc ?
#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdl {
    pub program_types: HashMap<String, ToolboxIdlProgramType>,
    pub program_accounts: HashMap<String, ToolboxIdlProgramAccount>,
    pub program_instructions: HashMap<String, ToolboxIdlProgramInstruction>,
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
            .map(|account_data| ToolboxIdl::try_from_bytes(&account_data))
            .transpose()
    }

    pub fn find_for_program_id(
        program_id: &Pubkey
    ) -> Result<Pubkey, ToolboxIdlError> {
        let base = Pubkey::find_program_address(&[], program_id).0;
        Pubkey::create_with_seed(&base, "anchor:idl", program_id)
            .map_err(ToolboxIdlError::Pubkey)
    }

    pub fn try_from_bytes(data: &[u8]) -> Result<ToolboxIdl, ToolboxIdlError> {
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
            usize::try_from(length).map_err(|err| {
                ToolboxIdlError::InvalidInteger {
                    conversion: err,
                    context: breadcrumbs.as_val("length"),
                }
            })?,
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
        let idl_root_object =
            idl_as_object_or_else(value, &breadcrumbs.as_idl("$"))?;
        Ok(ToolboxIdl {
            program_accounts: ToolboxIdl::try_parse_program_accounts(
                idl_root_object,
                breadcrumbs,
            )?,
            program_instructions: ToolboxIdl::try_parse_program_instructions(
                idl_root_object,
                breadcrumbs,
            )?,
            program_types: ToolboxIdl::try_parse_program_types(
                idl_root_object,
                breadcrumbs,
            )?,
            program_errors: ToolboxIdl::try_parse_program_errors(
                idl_root_object,
                breadcrumbs,
            )?,
        })
    }

    fn try_parse_program_types(
        idl_root_object: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<HashMap<String, ToolboxIdlProgramType>, ToolboxIdlError> {
        // TODO - generic way for object/array scoped machanism
        let mut program_types = HashMap::new();
        if let Some(idl_types_object) =
            idl_object_get_key_as_object(idl_root_object, "types")
        {
            for (idl_type_name, idl_type_value, breadcrumbs) in
                idl_object_get_scoped_key_value_array(
                    idl_types_object,
                    breadcrumbs,
                )?
            {
                // TODO - support generics
                if let Some(idl_type_object) = idl_type_value.as_object() {
                    if let Some(idl_type_typedef) = idl_type_object.get("type")
                    {
                    }
                }
                program_types.insert(
                    idl_type_name.to_string(),
                    ToolboxIdlProgramType {
                        name: idl_type_name.to_string(),
                        typedef: ToolboxIdlProgramTypedef::try_parse(
                            idl_type_value,
                            &breadcrumbs,
                        )?,
                    },
                );
            }
        }
        if let Some(idl_types_array) =
            idl_object_get_key_as_array(idl_root_object, "types")
        {
            for (idl_type_name, idl_type_object, breadcrumbs) in
                idl_array_get_scoped_named_object_array_or_else(
                    idl_types_array,
                    breadcrumbs,
                )?
            {
                let idl_type_typedef = idl_object_get_key_or_else(
                    idl_type_object,
                    "type",
                    &breadcrumbs.idl(),
                )?;
                let program_typedef = ToolboxIdlProgramTypedef::try_parse(
                    idl_type_typedef,
                    &breadcrumbs,
                )?;
                program_types.insert(
                    idl_type_name.to_string(),
                    ToolboxIdlProgramType {
                        name: idl_type_name.to_string(),
                        typedef: program_typedef,
                    },
                );
            }
        }
        Ok(program_types)
    }

    fn try_parse_program_accounts(
        idl_root_object: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<HashMap<String, ToolboxIdlProgramAccount>, ToolboxIdlError>
    {
        let mut program_accounts = HashMap::new();
        if let Some(idl_accounts_object) =
            idl_object_get_key_as_object(idl_root_object, "accounts")
        {
            for (idl_account_name, idl_account_value, breadcrumbs) in
                idl_object_get_scoped_key_value_array(
                    idl_accounts_object,
                    breadcrumbs,
                )?
            {
                let idl_account_object = idl_as_object_or_else(
                    idl_account_value,
                    &breadcrumbs.idl(),
                )?;
                let program_account = ToolboxIdlProgramAccount::try_parse(
                    idl_account_name,
                    idl_account_object,
                    &breadcrumbs,
                )?;
                program_accounts
                    .insert(program_account.name.to_string(), program_account);
            }
        }
        if let Some(idl_accounts_array) =
            idl_object_get_key_as_array(idl_root_object, "accounts")
        {
            for (idl_account_name, idl_account_object, breadcrumbs) in
                idl_array_get_scoped_named_object_array_or_else(
                    idl_accounts_array,
                    breadcrumbs,
                )?
            {
                let program_account = ToolboxIdlProgramAccount::try_parse(
                    idl_account_name,
                    idl_account_object,
                    &breadcrumbs,
                )?;
                program_accounts
                    .insert(program_account.name.to_string(), program_account);
            }
        }
        Ok(program_accounts)
    }

    fn try_parse_program_instructions(
        idl_root_object: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<HashMap<String, ToolboxIdlProgramInstruction>, ToolboxIdlError>
    {
        let mut program_instructions = HashMap::new();
        if let Some(idl_instructions_object) =
            idl_object_get_key_as_object(idl_root_object, "instructions")
        {
            for (idl_instruction_name, idl_instruction_value, breadcrumbs) in
                idl_object_get_scoped_key_value_array(
                    idl_instructions_object,
                    breadcrumbs,
                )?
            {
                let idl_instruction_object = idl_as_object_or_else(
                    idl_instruction_value,
                    &breadcrumbs.idl(),
                )?;
                let program_instruction =
                    ToolboxIdlProgramInstruction::try_parse(
                        idl_instruction_name,
                        idl_instruction_object,
                        &breadcrumbs,
                    )?;
                program_instructions.insert(
                    program_instruction.name.to_string(),
                    program_instruction,
                );
            }
        }
        if let Some(idl_instructions_array) =
            idl_object_get_key_as_array(idl_root_object, "instructions")
        {
            for (idl_instruction_name, idl_instruction_object, breadcrumbs) in
                idl_array_get_scoped_named_object_array_or_else(
                    idl_instructions_array,
                    breadcrumbs,
                )?
            {
                let program_instruction =
                    ToolboxIdlProgramInstruction::try_parse(
                        idl_instruction_name,
                        idl_instruction_object,
                        &breadcrumbs,
                    )?;
                program_instructions.insert(
                    program_instruction.name.to_string(),
                    program_instruction,
                );
            }
        }
        Ok(program_instructions)
    }

    fn try_parse_program_errors(
        idl_root_object: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<HashMap<u64, ToolboxIdlProgramError>, ToolboxIdlError> {
        let mut program_errors = HashMap::new();
        if let Some(idl_errors_object) =
            idl_object_get_key_as_object(idl_root_object, "errors")
        {
            for (idl_error_name, idl_error_value, breadcrumbs) in
                idl_object_get_scoped_key_value_array(
                    idl_errors_object,
                    breadcrumbs,
                )?
            {
                if let Some(idl_error_code) = idl_error_value.as_u64() {
                    program_errors.insert(
                        idl_error_code,
                        ToolboxIdlProgramError {
                            code: idl_error_code,
                            name: idl_error_name.to_string(),
                            msg: "".to_string(),
                        },
                    );
                } else {
                    let idl_error_object = idl_as_object_or_else(
                        idl_error_value,
                        &breadcrumbs.idl(),
                    )?;
                    let program_error = ToolboxIdlProgramError::try_parse(
                        idl_error_name,
                        idl_error_object,
                        &breadcrumbs,
                    )?;
                    program_errors.insert(program_error.code, program_error);
                }
            }
        }
        if let Some(idl_errors_array) =
            idl_object_get_key_as_array(idl_root_object, "errors")
        {
            for (idl_error_name, idl_error_object, breadcrumbs) in
                idl_array_get_scoped_named_object_array_or_else(
                    idl_errors_array,
                    breadcrumbs,
                )?
            {
                let program_error = ToolboxIdlProgramError::try_parse(
                    idl_error_name,
                    idl_error_object,
                    &breadcrumbs,
                )?;
                program_errors.insert(program_error.code, program_error);
            }
        }
        Ok(program_errors)
    }
}
