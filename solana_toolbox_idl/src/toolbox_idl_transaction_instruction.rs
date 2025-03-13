use std::collections::HashMap;

use serde_json::Value;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_instruction::ToolboxIdlProgramInstruction;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;
use crate::toolbox_idl_utils::idl_ok_or_else;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlTransactionInstruction {
    // TODO - should this contain the program_instruction ?
    pub program_id: Pubkey,
    pub name: String,
    pub accounts_addresses: HashMap<String, Pubkey>,
    pub args: Value,
}

impl ToolboxIdl {
    pub fn compile_transaction_instruction(
        &self,
        transaction_instruction: &ToolboxIdlTransactionInstruction,
    ) -> Result<Instruction, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let program_instruction = idl_map_get_key_or_else(
            &self.program_instructions,
            &transaction_instruction.name,
            &breadcrumbs.as_idl("$program_instructions"),
        )?;
        Ok(Instruction {
            program_id: transaction_instruction.program_id,
            accounts: ToolboxIdl::compile_transaction_instruction_accounts(
                program_instruction,
                &transaction_instruction.accounts_addresses,
                breadcrumbs,
            )?,
            data: ToolboxIdl::compile_transaction_instruction_data(
                program_instruction,
                &transaction_instruction.args,
                breadcrumbs,
            )?,
        })
    }

    // TODO - should this be on the program_instruction API ?
    pub fn decompile_transaction_instruction(
        &self,
        native_instruction: &Instruction,
    ) -> Result<ToolboxIdlTransactionInstruction, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let program_instruction = idl_ok_or_else(
            self.guess_program_instruction(native_instruction),
            "Could not guess instruction name",
            &breadcrumbs.as_val("instruction_name"),
        )?;
        Ok(ToolboxIdlTransactionInstruction {
            program_id: native_instruction.program_id,
            name: program_instruction.name.to_string(),
            accounts_addresses:
                ToolboxIdl::decompile_transaction_instruction_accounts(
                    program_instruction,
                    &native_instruction.accounts,
                    breadcrumbs,
                )?,
            args: ToolboxIdl::decompile_transaction_instruction_data(
                program_instruction,
                &native_instruction.data,
                breadcrumbs,
            )?,
        })
    }

    pub fn guess_program_instruction(
        &self,
        native_instruction: &Instruction,
    ) -> Option<&ToolboxIdlProgramInstruction> {
        for program_instruction in self.program_instructions.values() {
            if native_instruction
                .data
                .starts_with(&program_instruction.discriminator)
            {
                return Some(program_instruction);
            }
        }
        None
    }
}
