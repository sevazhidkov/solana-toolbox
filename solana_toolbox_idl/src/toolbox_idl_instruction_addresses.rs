use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_instruction::ToolboxIdlInstruction;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;

impl ToolboxIdlInstruction {
    pub fn encode_addresses(
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

    pub fn decode_addresses(
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
        Ok(instruction_addresses)
    }

    pub fn find_addresses(
        &self,
        instruction_program_id: &Pubkey,
        instruction_payload: &Value,
        instruction_addresses: &HashMap<String, Pubkey>,
    ) -> HashMap<String, Pubkey> {
        self.find_addresses_with_accounts_content_types_and_states(
            instruction_program_id,
            instruction_payload,
            instruction_addresses,
            &HashMap::new(),
        )
    }

    pub fn find_addresses_with_accounts_content_types_and_states(
        &self,
        instruction_program_id: &Pubkey,
        instruction_payload: &Value,
        instruction_addresses: &HashMap<String, Pubkey>,
        accounts_content_types_and_states: &HashMap<
            String,
            (Arc<ToolboxIdlTypeFull>, Value),
        >,
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
                if let Ok(instruction_address) = instruction_account.try_find(
                    instruction_program_id,
                    &self.args_type_full_fields,
                    instruction_payload,
                    &instruction_addresses,
                    accounts_content_types_and_states,
                    &breadcrumbs.with_idl(&instruction_account.name),
                ) {
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
