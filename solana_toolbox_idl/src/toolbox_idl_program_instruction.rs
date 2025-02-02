use std::collections::HashMap;

use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_instruction_account::ToolboxIdlProgramInstructionAccount;
use crate::toolbox_idl_program_type::ToolboxIdlProgramType;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFields;
use crate::toolbox_idl_utils::idl_array_get_scoped_named_object_array_or_else;
use crate::toolbox_idl_utils::idl_as_bytes_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_array_or_else;
use crate::ToolboxIdlTypeFlat;
use crate::ToolboxIdlTypeFull;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgramInstruction {
    pub name: String,
    pub discriminator: Vec<u8>,
    pub accounts: Vec<ToolboxIdlProgramInstructionAccount>,
    pub data_type_flat: ToolboxIdlTypeFlat,
    pub data_type_full: ToolboxIdlTypeFull,
}

impl ToolboxIdlProgramInstruction {
    pub fn print(&self) {
        println!("----");
        println!("instruction.name: {}", self.name);
        println!("instruction.discriminator: {:?}", self.discriminator);
        for (index, instruction_account) in self.accounts.iter().enumerate() {
            println!(
                "instruction.accounts: #{:03}: {}{} {}{}",
                index + 1,
                if instruction_account.is_writable { "W" } else { "R" },
                if instruction_account.is_signer { "S" } else { "-" },
                instruction_account.name,
                if instruction_account.address.is_some()
                    || instruction_account.pda.is_some()
                {
                    " (RESOLVABLE)"
                } else {
                    ""
                }
            );
        }
        println!("instruction.data_type: {}", self.data_type_flat.describe());
    }

    pub(crate) fn try_parse(
        program_types: &HashMap<String, ToolboxIdlProgramType>,
        idl_instruction_name: &str,
        idl_instruction: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramInstruction, ToolboxIdlError> {
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
        let program_instruction_data_type_flat =
            ToolboxIdlProgramInstruction::try_parse_data_type_flat(
                idl_instruction,
                breadcrumbs,
            )?;
        let program_instruction_data_type_full =
            ToolboxIdlProgramInstruction::try_parse_data_type_full(
                program_types,
                &program_instruction_data_type_flat,
                breadcrumbs,
            )?;
        Ok(ToolboxIdlProgramInstruction {
            name: idl_instruction_name.to_string(),
            discriminator: program_instruction_discriminator,
            accounts: program_instruction_accounts,
            data_type_flat: program_instruction_data_type_flat,
            data_type_full: program_instruction_data_type_full,
        })
    }

    fn try_parse_discriminator(
        idl_instruction_name: &str,
        idl_instruction: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Vec<u8>, ToolboxIdlError> {
        Ok(
            if let Some(idl_instruction_discriminator) =
                idl_instruction.get("discriminator")
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
            idl_instruction_account_name,
            idl_instruction_account_object,
            breadcrumbs,
        ) in idl_array_get_scoped_named_object_array_or_else(
            idl_instruction_accounts_array,
            breadcrumbs,
        )? {
            instruction_accounts.push(
                ToolboxIdlProgramInstructionAccount::try_parse(
                    idl_instruction_account_name,
                    idl_instruction_account_object,
                    &breadcrumbs,
                )?,
            );
        }
        Ok(instruction_accounts)
    }

    fn try_parse_data_type_flat(
        idl_instruction: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFlat, ToolboxIdlError> {
        if let Some(idl_instruction_args) = idl_instruction.get("args") {
            let idl_instruction_args = Value::Object(Map::from_iter(vec![(
                "fields".to_string(),
                idl_instruction_args.clone(),
            )]));
            return ToolboxIdlTypeFlat::try_parse(
                &idl_instruction_args,
                &breadcrumbs,
            );
        }
        Ok(ToolboxIdlTypeFlat::Struct {
            fields: ToolboxIdlTypeFlatFields::None,
        })
    }

    fn try_parse_data_type_full(
        program_types: &HashMap<String, ToolboxIdlProgramType>,
        data_type_flat: &ToolboxIdlTypeFlat,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFull, ToolboxIdlError> {
        ToolboxIdlTypeFull::try_hydrate(
            program_types,
            &HashMap::new(),
            data_type_flat,
            breadcrumbs,
        )
    }
}
