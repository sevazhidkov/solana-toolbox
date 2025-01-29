use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_instruction_account::ToolboxIdlProgramInstructionAccount;
use crate::toolbox_idl_program_typedef::ToolboxIdlProgramTypedef;
use crate::toolbox_idl_utils::idl_as_bytes_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_scoped_named_content_array_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_scoped_named_object_array_or_else;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgramInstruction {
    pub name: String,
    pub discriminator: Vec<u8>,
    pub accounts: Vec<ToolboxIdlProgramInstructionAccount>,
    pub args: Vec<(String, ToolboxIdlProgramTypedef)>,
}

impl ToolboxIdlProgramInstruction {
    pub(crate) fn try_parse(
        idl_instruction_name: &str,
        idl_instruction_object: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramInstruction, ToolboxIdlError> {
        Ok(ToolboxIdlProgramInstruction {
            name: idl_instruction_name.to_string(),
            discriminator:
                ToolboxIdlProgramInstruction::try_parse_discriminator(
                    idl_instruction_name,
                    idl_instruction_object,
                    breadcrumbs,
                )?,
            accounts: ToolboxIdlProgramInstruction::try_parse_accounts(
                idl_instruction_object,
                breadcrumbs,
            )?,
            args: ToolboxIdlProgramInstruction::try_parse_args(
                idl_instruction_object,
                breadcrumbs,
            )?,
        })
    }

    fn try_parse_discriminator(
        idl_instruction_name: &str,
        idl_instruction_object: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Vec<u8>, ToolboxIdlError> {
        Ok(
            if let Some(idl_instruction_discriminator) =
                idl_instruction_object.get("discriminator")
            {
                idl_as_bytes_or_else(
                    idl_instruction_discriminator,
                    &breadcrumbs.as_val("discriminator"),
                )?
            } else {
                ToolboxIdl::compute_instruction_discriminator(
                    idl_instruction_name,
                )
            },
        )
    }

    fn try_parse_accounts(
        idl_instruction_object: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Vec<ToolboxIdlProgramInstructionAccount>, ToolboxIdlError> {
        let mut instruction_accounts = vec![];
        for (idl_instruction_account_name, idl_instruction_account_object, _) in
            idl_object_get_key_as_scoped_named_object_array_or_else(
                idl_instruction_object,
                "accounts",
                breadcrumbs,
            )?
        {
            instruction_accounts.push(
                ToolboxIdlProgramInstructionAccount::try_parse(
                    idl_instruction_account_name,
                    idl_instruction_account_object,
                    breadcrumbs,
                )?,
            );
        }
        Ok(instruction_accounts)
    }

    fn try_parse_args(
        idl_instruction_object: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Vec<(String, ToolboxIdlProgramTypedef)>, ToolboxIdlError> {
        let mut instruction_args = vec![];
        if idl_instruction_object.contains_key("args") {
            for (
                idl_instruction_arg_name,
                idl_instruction_arg_typedef,
                breadcrumbs,
            ) in idl_object_get_key_as_scoped_named_content_array_or_else(
                idl_instruction_object,
                "args",
                "type",
                &breadcrumbs,
            )? {
                instruction_args.push((
                    idl_instruction_arg_name.to_string(),
                    ToolboxIdlProgramTypedef::try_parse(
                        idl_instruction_arg_typedef,
                        &breadcrumbs,
                    )?,
                ));
            }
        }
        Ok(instruction_args)
    }
}
