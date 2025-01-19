use std::collections::HashMap;

use serde_json::Map;
use serde_json::Value;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_error::ToolboxIdlError;

impl ToolboxIdl {
    pub fn generate_instruction(
        &self,
        program_id: &Pubkey,
        instruction_name: &str,
        instruction_accounts_addresses: &HashMap<String, Pubkey>,
        instruction_args: &Map<String, Value>,
    ) -> Result<Instruction, ToolboxIdlError> {
        let instruction_accounts = self.generate_instruction_accounts(
            instruction_name,
            instruction_accounts_addresses,
        )?;
        let instruction_data =
            self.compile_instruction_data(instruction_name, instruction_args)?;
        Ok(Instruction {
            program_id: *program_id,
            accounts: instruction_accounts,
            data: instruction_data,
        })
    }
}
