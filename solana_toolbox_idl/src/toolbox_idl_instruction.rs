use std::collections::HashMap;

use serde_json::Value;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;

use crate::ToolboxIdlBreadcrumbs;
use crate::ToolboxIdlError;
use crate::ToolboxIdlInstructionAccount;
use crate::ToolboxIdlTypeFlatFields;
use crate::ToolboxIdlTypeFullFields;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlInstruction {
    pub name: String,
    pub discriminator: Vec<u8>,
    pub accounts: Vec<ToolboxIdlInstructionAccount>,
    pub args_type_flat_fields: ToolboxIdlTypeFlatFields,
    pub args_type_full_fields: ToolboxIdlTypeFullFields,
}

impl ToolboxIdlInstruction {
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
