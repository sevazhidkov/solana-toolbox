use std::collections::HashMap;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccount;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccountPda;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccountPdaBlob;
use crate::toolbox_idl_utils::idl_iter_get_scoped_values;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;

impl ToolboxIdlInstructionAccount {
    pub fn try_find(
        &self,
        instruction_program_id: &Pubkey,
        instruction_payload: &Value,
        instruction_addresses: &HashMap<String, Pubkey>,
        instruction_accounts_states: &HashMap<String, Value>,
    ) -> Result<Pubkey> {
        if let Some(address) = instruction_addresses.get(&self.name) {
            return Ok(*address);
        }
        if let Some(instruction_account_address) = &self.address {
            return Ok(*instruction_account_address);
        }
        if let Some(instruction_account_pda) = &self.pda {
            return instruction_account_pda.try_find(
                instruction_program_id,
                instruction_payload,
                instruction_addresses,
                instruction_accounts_states,
            );
        }
        Err(anyhow!("Could not find account (unresolvable)"))
    }
}

impl ToolboxIdlInstructionAccountPda {
    pub fn try_find(
        &self,
        instruction_program_id: &Pubkey,
        instruction_payload: &Value,
        instruction_addresses: &HashMap<String, Pubkey>,
        instruction_accounts_states: &HashMap<String, Value>,
    ) -> Result<Pubkey> {
        let mut pda_seeds_bytes = vec![];
        for (_, pda_seed_blob, context) in
            idl_iter_get_scoped_values(&self.seeds)
        {
            pda_seeds_bytes.push(
                pda_seed_blob
                    .try_compute(
                        instruction_payload,
                        instruction_addresses,
                        instruction_accounts_states,
                    )
                    .context(context)?,
            );
        }
        let pda_program_id = if let Some(pda_program_blob) = &self.program {
            let pda_program_id_bytes = pda_program_blob
                .try_compute(
                    instruction_payload,
                    instruction_addresses,
                    instruction_accounts_states,
                )
                .context("Program blob compute")?;
            Pubkey::new_from_array(
                pda_program_id_bytes
                    .try_into()
                    .map_err(|error| {
                        anyhow!("Invalid Pubkey bytes: {:?}", error)
                    })
                    .context("Program")?,
            )
        } else {
            *instruction_program_id
        };
        let mut pda_seeds_slices = vec![];
        for pda_seed_bytes in pda_seeds_bytes.iter() {
            pda_seeds_slices.push(&pda_seed_bytes[..]);
        }
        Ok(Pubkey::find_program_address(&pda_seeds_slices, &pda_program_id).0)
    }
}

impl ToolboxIdlInstructionAccountPdaBlob {
    pub fn try_compute(
        &self,
        instruction_payload: &Value,
        instruction_addresses: &HashMap<String, Pubkey>,
        instruction_accounts_states: &HashMap<String, Value>,
    ) -> Result<Vec<u8>> {
        let (type_full, value) = match self {
            ToolboxIdlInstructionAccountPdaBlob::Const {
                value,
                type_full,
                ..
            } => (type_full, value),
            ToolboxIdlInstructionAccountPdaBlob::Arg {
                path,
                type_full,
                ..
            } => {
                let value = path
                    .try_get_json_value(instruction_payload)
                    .context("Arg extract value")?;
                (type_full, value)
            },
            ToolboxIdlInstructionAccountPdaBlob::Account {
                instruction_account_name,
                account_content_path,
                type_full,
                ..
            } => {
                if account_content_path.is_empty() {
                    return idl_map_get_key_or_else(
                        instruction_addresses,
                        instruction_account_name,
                    )
                    .context("Instruction addresses")
                    .map(|address| address.to_bytes().to_vec());
                }
                let instruction_account_state = idl_map_get_key_or_else(
                    instruction_accounts_states,
                    instruction_account_name,
                )
                .context("Instruction accounts states")?;
                let value = account_content_path
                    .try_get_json_value(instruction_account_state)
                    .context("Account extract value")?;
                (type_full, value)
            },
        };
        let mut bytes = vec![];
        type_full
            .try_serialize(value, &mut bytes, false)
            .context("Serialize")?;
        Ok(bytes)
    }
}
