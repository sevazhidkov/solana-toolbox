use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccount;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFields;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlInstruction {
    pub name: String,
    pub discriminator: Vec<u8>,
    pub accounts: Vec<ToolboxIdlInstructionAccount>,
    pub args_type_flat_fields: ToolboxIdlTypeFlatFields,
    pub args_type_full_fields: Arc<ToolboxIdlTypeFullFields>,
}

impl ToolboxIdlInstruction {
    pub fn compile(
        &self,
        program_id: &Pubkey,
        instruction_addresses: &HashMap<String, Pubkey>,
        instruction_payload: &Value,
    ) -> Result<Instruction, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        Ok(Instruction {
            program_id: *program_id,
            accounts: self.compile_addresses(
                instruction_addresses,
                &breadcrumbs.with_idl("addresses"),
            )?,
            data: self.compile_payload(
                instruction_payload,
                &breadcrumbs.with_idl("payload"),
            )?,
        })
    }

    pub fn decompile(
        &self,
        instruction: &Instruction,
    ) -> Result<(Pubkey, HashMap<String, Pubkey>, Value), ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        Ok((
            instruction.program_id,
            self.decompile_addresses(
                &instruction.accounts,
                &breadcrumbs.with_idl("addresses"),
            )?,
            self.decompile_payload(
                &instruction.data,
                &breadcrumbs.with_idl("payload"),
            )?,
        ))
    }
}
