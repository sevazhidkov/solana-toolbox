use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use serde_json::Value;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccount;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFields;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;
use crate::toolbox_idl_utils::idl_convert_to_value_name;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlInstruction {
    pub name: String,
    pub docs: Option<Value>,
    pub discriminator: Vec<u8>,
    pub accounts: Vec<ToolboxIdlInstructionAccount>,
    pub args_type_flat_fields: ToolboxIdlTypeFlatFields,
    pub args_type_full_fields: Arc<ToolboxIdlTypeFullFields>,
    pub return_type_flat: ToolboxIdlTypeFlat,
    pub return_type_full: Arc<ToolboxIdlTypeFull>,
}

impl Default for ToolboxIdlInstruction {
    fn default() -> ToolboxIdlInstruction {
        ToolboxIdlInstruction {
            name: ToolboxIdlInstruction::sanitize_name("unknown_instruction"),
            docs: None,
            discriminator: vec![],
            accounts: vec![],
            args_type_flat_fields: ToolboxIdlTypeFlatFields::nothing(),
            args_type_full_fields: ToolboxIdlTypeFullFields::nothing().into(),
            return_type_flat: ToolboxIdlTypeFlat::nothing(),
            return_type_full: ToolboxIdlTypeFull::nothing().into(),
        }
    }
}

impl ToolboxIdlInstruction {
    pub fn sanitize_name(name: &str) -> String {
        idl_convert_to_value_name(name)
    }

    pub fn encode(
        &self,
        instruction_program_id: &Pubkey,
        instruction_payload: &Value,
        instruction_addresses: &HashMap<String, Pubkey>,
    ) -> Result<Instruction> {
        Ok(Instruction {
            program_id: *instruction_program_id,
            data: self.encode_payload(instruction_payload)?,
            accounts: self.encode_addresses(instruction_addresses)?,
        })
    }

    pub fn decode(
        &self,
        instruction: &Instruction,
    ) -> Result<(Pubkey, Value, HashMap<String, Pubkey>)> {
        Ok((
            instruction.program_id,
            self.decode_payload(&instruction.data)?,
            self.decode_addresses(&instruction.accounts)?,
        ))
    }
}
