use std::collections::HashMap;

use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_instruction::ToolboxIdlInstruction;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;

impl ToolboxIdlInstruction {
    pub fn compile_addresses(
        &self,
        instruction_addresses: &HashMap<String, Pubkey>,
    ) -> Result<Vec<AccountMeta>, ToolboxIdlError> {
        let mut instruction_metas = vec![];
        for instruction_account in &self.accounts {
            if instruction_account.optional
                && !instruction_addresses
                    .contains_key(&instruction_account.name)
            {
                continue;
            }
            let instruction_address = *idl_map_get_key_or_else(
                instruction_addresses,
                &instruction_account.name,
                &ToolboxIdlBreadcrumbs::default()
                    .as_val("instruction_addresses"),
            )?;
            if instruction_account.writable {
                instruction_metas.push(AccountMeta::new(
                    instruction_address,
                    instruction_account.signer,
                ));
            } else {
                instruction_metas.push(AccountMeta::new_readonly(
                    instruction_address,
                    instruction_account.signer,
                ));
            }
        }
        Ok(instruction_metas)
    }

    pub fn decompile_addresses(
        &self,
        instruction_metas: &[AccountMeta],
    ) -> Result<HashMap<String, Pubkey>, ToolboxIdlError> {
        let mut instruction_optionals_possible = 0usize;
        for account in &self.accounts {
            if account.optional {
                instruction_optionals_possible += 1;
            }
        }
        let instruction_optionals_unuseds =
            self.accounts.len().saturating_sub(instruction_metas.len());
        let instruction_optionals_used = instruction_optionals_possible
            .saturating_sub(instruction_optionals_unuseds);
        let mut instruction_addresses = HashMap::new();
        let mut instruction_meta_index = 0;
        let mut instruction_optionals_current = 0;
        for account in &self.accounts {
            if account.optional {
                instruction_optionals_current += 1;
            }
            if instruction_optionals_current > instruction_optionals_used {
                continue;
            }
            if instruction_meta_index >= instruction_metas.len() {
                break;
            }
            instruction_addresses.insert(
                account.name.to_string(),
                instruction_metas[instruction_meta_index].pubkey,
            );
            instruction_meta_index += 1;
        }
        loop {
            if instruction_meta_index >= instruction_metas.len() {
                break;
            }
            instruction_addresses.insert(
                format!("_remaining_{}", instruction_meta_index),
                instruction_metas[instruction_meta_index].pubkey,
            );
            instruction_meta_index += 1;
        }
        Ok(instruction_addresses)
    }
}
