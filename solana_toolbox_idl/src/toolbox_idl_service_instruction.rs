use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use serde_json::Value;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_idl_instruction::ToolboxIdlInstruction;
use crate::toolbox_idl_program::ToolboxIdlProgram;
use crate::toolbox_idl_service::ToolboxIdlService;

pub struct ToolboxIdlServiceInstructionDecoded {
    pub program_id: Pubkey,
    pub program: Arc<ToolboxIdlProgram>,
    pub instruction: Arc<ToolboxIdlInstruction>,
    pub payload: Value,
    pub addresses: HashMap<String, Pubkey>,
}

impl ToolboxIdlService {
    pub async fn decode_instruction(
        &mut self,
        endpoint: &mut ToolboxEndpoint,
        instruction: &Instruction,
    ) -> Result<ToolboxIdlServiceInstructionDecoded> {
        let idl_program = self
            .resolve_program(endpoint, &instruction.program_id)
            .await?
            .unwrap_or_default();
        let idl_instruction = idl_program
            .guess_instruction(&instruction.data)
            .unwrap_or_default();
        let (_, instruction_payload, instruction_addresses) =
            idl_instruction.decode(instruction)?;
        Ok(ToolboxIdlServiceInstructionDecoded {
            program_id: instruction.program_id,
            program: idl_program,
            instruction: idl_instruction,
            payload: instruction_payload,
            addresses: instruction_addresses,
        })
    }

    pub async fn resolve_and_encode_instruction(
        &mut self,
        endpoint: &mut ToolboxEndpoint,
        idl_instruction: &ToolboxIdlInstruction,
        instruction_program_id: &Pubkey,
        instruction_payload: &Value,
        instruction_addresses: &HashMap<String, Pubkey>,
    ) -> Result<Instruction> {
        idl_instruction.encode(
            instruction_program_id,
            instruction_payload,
            &self
                .resolve_instruction_addresses(
                    endpoint,
                    idl_instruction,
                    instruction_program_id,
                    instruction_payload,
                    instruction_addresses,
                )
                .await?,
        )
    }

    pub async fn resolve_instruction_addresses(
        &mut self,
        endpoint: &mut ToolboxEndpoint,
        idl_instruction: &ToolboxIdlInstruction,
        instruction_program_id: &Pubkey,
        instruction_payload: &Value,
        instruction_addresses: &HashMap<String, Pubkey>,
    ) -> Result<HashMap<String, Pubkey>> {
        let mut instruction_addresses = instruction_addresses.clone();
        let mut instruction_accounts_states = HashMap::new();
        for (instruction_account_name, instruction_address) in
            &instruction_addresses
        {
            let account_decoded = self
                .get_and_decode_account(endpoint, instruction_address)
                .await?;
            instruction_accounts_states.insert(
                instruction_account_name.to_string(),
                account_decoded.state,
            );
        }
        loop {
            let mut made_progress = false;
            for idl_instruction_account in &idl_instruction.accounts {
                if instruction_addresses
                    .contains_key(&idl_instruction_account.name)
                {
                    continue;
                }
                if let Ok(instruction_address) = idl_instruction_account
                    .try_find(
                        instruction_program_id,
                        instruction_payload,
                        &instruction_addresses,
                        &instruction_accounts_states,
                    )
                {
                    made_progress = true;
                    instruction_addresses.insert(
                        idl_instruction_account.name.to_string(),
                        instruction_address,
                    );
                    let account_decoded = self
                        .get_and_decode_account(endpoint, &instruction_address)
                        .await?;
                    instruction_accounts_states.insert(
                        idl_instruction_account.name.to_string(),
                        account_decoded.state,
                    );
                }
            }
            if !made_progress {
                break;
            }
        }
        Ok(instruction_addresses)
    }
}
