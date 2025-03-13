use std::collections::HashMap;

use serde_json::Value;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

use crate::{
    ToolboxIdlBreadcrumbs, ToolboxIdlError,
    ToolboxIdlProgramInstructionAccount, ToolboxIdlProgramTypeFlatFields,
    ToolboxIdlProgramTypeFullFields,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgramInstruction {
    pub name: String,
    pub discriminator: Vec<u8>,
    pub accounts: Vec<ToolboxIdlProgramInstructionAccount>,
    pub args_type_flat_fields: ToolboxIdlProgramTypeFlatFields,
    pub args_type_full_fields: ToolboxIdlProgramTypeFullFields,
}

impl ToolboxIdlProgramInstruction {
    pub fn compile(
        &self,
        program_id: &Pubkey,
        accounts_addresses: &HashMap<String, Pubkey>,
        args: &Value,
    ) -> Result<Instruction, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        Ok(Instruction {
            program_id: *program_id,
            accounts: self.compile_accounts_addresses(
                accounts_addresses,
                &breadcrumbs.with_idl("accounts_addresses"),
            )?,
            data: self.compile_args(args, &breadcrumbs.with_idl("args"))?,
        })
    }

    pub fn decompile(
        &self,
        instruction: &Instruction,
    ) -> Result<(Pubkey, HashMap<String, Pubkey>, Value), ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        Ok((
            instruction.program_id,
            self.decompile_accounts_addresses(
                &instruction.accounts,
                &breadcrumbs.with_idl("accounts_addresses"),
            )?,
            self.decompile_args(
                &instruction.data,
                &breadcrumbs.with_idl("args"),
            )?,
        ))
    }
}
