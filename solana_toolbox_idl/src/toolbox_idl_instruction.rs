use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccount;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFields;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
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
        Ok(Instruction {
            program_id: *program_id,
            accounts: self.compile_addresses(instruction_addresses)?,
            data: self.compile_payload(instruction_payload)?,
        })
    }

    pub fn decompile(
        &self,
        instruction: &Instruction,
    ) -> Result<(Pubkey, HashMap<String, Pubkey>, Value), ToolboxIdlError> {
        Ok((
            instruction.program_id,
            self.decompile_addresses(&instruction.accounts)?,
            self.decompile_payload(&instruction.data)?,
        ))
    }

    pub fn find_addresses(
        &self,
        program_id: &Pubkey,
        instruction_addresses: &HashMap<String, Pubkey>,
        instruction_payload: &Value,
    ) -> HashMap<String, Pubkey> {
        self.find_addresses_with_snapshots(
            program_id,
            instruction_addresses,
            instruction_payload,
            &HashMap::new(),
        )
    }

    pub fn find_addresses_with_snapshots(
        &self,
        program_id: &Pubkey,
        instruction_addresses: &HashMap<String, Pubkey>,
        instruction_payload: &Value,
        snapshots: &HashMap<String, (Arc<ToolboxIdlTypeFull>, Value)>,
    ) -> HashMap<String, Pubkey> {
        let mut instruction_addresses = instruction_addresses.clone();
        loop {
            let breadcrumbs = ToolboxIdlBreadcrumbs::default();
            let mut made_progress = false;
            for instruction_account in &self.accounts {
                if instruction_addresses.contains_key(&instruction_account.name)
                {
                    continue;
                }
                if let Ok(instruction_address) = instruction_account
                    .try_compute(
                        program_id,
                        &instruction_addresses,
                        snapshots,
                        &(&self.args_type_full_fields, &instruction_payload),
                        &breadcrumbs.with_idl(&instruction_account.name),
                    )
                {
                    made_progress = true;
                    instruction_addresses.insert(
                        instruction_account.name.to_string(),
                        instruction_address,
                    );
                }
            }
            if !made_progress {
                break;
            }
        }
        instruction_addresses
    }
}
