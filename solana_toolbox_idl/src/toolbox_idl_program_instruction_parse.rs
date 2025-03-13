use std::collections::HashMap;

use serde_json::Map;
use serde_json::Value;
use sha2::Digest;
use sha2::Sha256;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_instruction::ToolboxIdlProgramInstruction;
use crate::toolbox_idl_program_instruction_account::ToolboxIdlProgramInstructionAccount;
use crate::toolbox_idl_program_type_flat::ToolboxIdlProgramTypeFlatFields;
use crate::toolbox_idl_program_typedef::ToolboxIdlProgramTypedef;
use crate::toolbox_idl_utils::idl_as_bytes_or_else;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_iter_get_scoped_values;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_array_or_else;
use crate::ToolboxIdlProgramTypeFullFields;

impl ToolboxIdlProgramInstruction {
    pub fn try_parse(
        idl_instruction_name: &str,
        idl_instruction: &Value,
        program_typedefs: &HashMap<String, ToolboxIdlProgramTypedef>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramInstruction, ToolboxIdlError> {
        let idl_instruction =
            idl_as_object_or_else(idl_instruction, &breadcrumbs.idl())?;
        let program_instruction_discriminator =
            ToolboxIdlProgramInstruction::try_parse_discriminator(
                idl_instruction_name,
                idl_instruction,
                breadcrumbs,
            )?;
        let program_instruction_accounts =
            ToolboxIdlProgramInstruction::try_parse_accounts(
                idl_instruction,
                breadcrumbs,
            )?;
        let program_instruction_args_type_flat_fields =
            ToolboxIdlProgramInstruction::try_parse_args_type_flat_fields(
                idl_instruction,
                breadcrumbs,
            )?;
        let program_instruction_args_type_full_fields =
            ToolboxIdlProgramInstruction::try_parse_args_type_full_fields(
                &program_instruction_args_type_flat_fields,
                program_typedefs,
                breadcrumbs,
            )?;
        Ok(ToolboxIdlProgramInstruction {
            name: idl_instruction_name.to_string(),
            discriminator: program_instruction_discriminator,
            accounts: program_instruction_accounts,
            args_type_flat_fields: program_instruction_args_type_flat_fields,
            args_type_full_fields: program_instruction_args_type_full_fields,
        })
    }

    fn try_parse_discriminator(
        idl_instruction_name: &str,
        idl_instruction: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Vec<u8>, ToolboxIdlError> {
        Ok(
            if let Some(idl_instruction_discriminator) =
                idl_object_get_key_as_array(idl_instruction, "discriminator")
            {
                idl_as_bytes_or_else(
                    idl_instruction_discriminator,
                    &breadcrumbs.as_val("discriminator"),
                )?
            } else {
                let mut hasher = Sha256::new();
                hasher.update(format!("global:{}", idl_instruction_name));
                hasher.finalize()[..8].to_vec()
            },
        )
    }

    fn try_parse_accounts(
        idl_instruction: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Vec<ToolboxIdlProgramInstructionAccount>, ToolboxIdlError> {
        let idl_instruction_accounts_array =
            idl_object_get_key_as_array_or_else(
                idl_instruction,
                "accounts",
                &breadcrumbs.idl(),
            )?;
        let mut instruction_accounts = vec![];
        for (
            idl_instruction_account_index,
            idl_instruction_account,
            breadcrumbs,
        ) in idl_iter_get_scoped_values(
            idl_instruction_accounts_array,
            breadcrumbs,
        )? {
            instruction_accounts.push(
                ToolboxIdlProgramInstructionAccount::try_parse(
                    idl_instruction_account_index,
                    idl_instruction_account,
                    &breadcrumbs,
                )?,
            );
        }
        Ok(instruction_accounts)
    }

    fn try_parse_args_type_flat_fields(
        idl_instruction: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramTypeFlatFields, ToolboxIdlError> {
        if let Some(idl_instruction_args) =
            idl_object_get_key_as_array(idl_instruction, "args")
        {
            return ToolboxIdlProgramTypeFlatFields::try_parse(
                &idl_instruction_args,
                breadcrumbs,
            );
        }
        Ok(ToolboxIdlProgramTypeFlatFields::None)
    }

    fn try_parse_args_type_full_fields(
        args_type_flat_fields: &ToolboxIdlProgramTypeFlatFields,
        program_typedefs: &HashMap<String, ToolboxIdlProgramTypedef>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramTypeFullFields, ToolboxIdlError> {
        args_type_flat_fields.try_hydrate(
            &HashMap::new(),
            program_typedefs,
            breadcrumbs,
        )
    }
}
