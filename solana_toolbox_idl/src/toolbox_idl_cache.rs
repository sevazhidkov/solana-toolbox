use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

// TODO - use path import ?
use crate::ToolboxIdlAccount;
use crate::ToolboxIdlBreadcrumbs;
use crate::ToolboxIdlError;
use crate::ToolboxIdlInstruction;
use crate::ToolboxIdlProgram;

pub struct ToolboxIdlCache {
    programs: HashMap<Pubkey, ToolboxIdlProgram>,
}

impl ToolboxIdlCache {
    pub fn new() -> ToolboxIdlCache {
        ToolboxIdlCache {
            programs: Default::default(),
        }
    }

    pub async fn get_program(
        &mut self,
        endpoint: &mut ToolboxEndpoint,
        program_id: &Pubkey,
    ) -> Result<&ToolboxIdlProgram, ToolboxIdlError> {
        // TODO - provide standard implementation for basic contracts such as spl_token and system, and compute_budget ?
        if !self.programs.contains_key(program_id) {
            let program = endpoint
                .get_account(&ToolboxIdlProgram::find(program_id)?)
                .await?
                .map(|account| ToolboxIdlProgram::try_from_account(&account))
                .transpose()?
                .ok_or_else(|| ToolboxIdlError::CouldNotFindIdl {
                    program_id: *program_id,
                })?;
            self.programs.insert(*program_id, program);
        }
        Ok(self.programs.get(program_id).unwrap())
    }

    // TODO - I'm not a huge fan of those tuples
    pub async fn get_account_snapshot(
        &mut self,
        endpoint: &mut ToolboxEndpoint,
        address: &Pubkey,
    ) -> Result<Option<(&ToolboxIdlAccount, Value)>, ToolboxIdlError> {
        let account = match endpoint.get_account(address).await? {
            Some(account) => account,
            None => return Ok(None),
        };
        let account_def = self
            .get_program(endpoint, &account.owner)
            .await?
            .guess_account(&account.data)?;
        Ok(Some((account_def, account_def.decompile(&account.data)?)))
    }

    pub async fn resolve_instruction_accounts_addresses(
        &mut self,
        endpoint: &mut ToolboxEndpoint,
        program_id: &Pubkey,
        instruction: &ToolboxIdlInstruction,
        accounts_addresses: HashMap<String, Pubkey>,
        args: &Value,
    ) -> Result<HashMap<String, Pubkey>, ToolboxIdlError> {
        let mut accounts_addresses = accounts_addresses.clone();
        let mut accounts_snapshots = HashMap::new();
        for (account_name, account_addresses) in &accounts_addresses {
            if let Ok(Some(account_snapshot)) =
                self.get_account_snapshot(endpoint, account_addresses).await
            {
                accounts_snapshots
                    .insert(account_name.to_string(), account_snapshot);
            }
        }
        loop {
            let breadcrumbs = ToolboxIdlBreadcrumbs::default();
            let mut made_progress = false;
            for instruction_account in &instruction.accounts {
                if accounts_addresses.contains_key(&instruction_account.name) {
                    continue;
                }
                if let Ok(account_address) = instruction_account.try_compute(
                    program_id,
                    &accounts_addresses,
                    &accounts_snapshots,
                    &(&instruction.args_type_full_fields, &args),
                    &breadcrumbs.with_idl(&instruction.name),
                ) {
                    made_progress = true;
                    accounts_addresses.insert(
                        instruction_account.name.to_string(),
                        account_address,
                    );
                    if let Ok(Some(account_snapshot)) = self
                        .get_account_snapshot(endpoint, &account_address)
                        .await
                    {
                        accounts_snapshots.insert(
                            instruction_account.name.to_string(),
                            account_snapshot,
                        );
                    }
                }
            }
            if !made_progress {
                break;
            }
        }
        Ok(accounts_addresses)
    }
}
