use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::str::FromStr;

use clap::Args;
use serde_json::{json, Map};
use solana_cli_config::Config;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;

use crate::toolbox_cli_error::ToolboxCliError;
use crate::toolbox_cli_utils::ToolboxCliUtils;

#[derive(Debug, Clone, Args)]
pub struct ToolboxCliCommandRawSearchOccurancesArgs {
    with_address: String,
    limit: Option<usize>,
    start_before_signature: Option<String>,
    rewind_until_signature: Option<String>,
}

impl ToolboxCliCommandRawSearchOccurancesArgs {
    pub async fn process(
        &self,
        config: &Config,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = ToolboxCliUtils::new_endpoint(config)?;
        let with_address = Pubkey::from_str(&self.with_address).unwrap();
        let start_before = self
            .start_before_signature
            .as_ref()
            .map(|signature| Signature::from_str(signature))
            .transpose()?;
        let rewind_until = self
            .rewind_until_signature
            .as_ref()
            .map(|signature| Signature::from_str(signature))
            .transpose()?;
        let signatures = endpoint
            .search_signatures(
                &with_address,
                start_before,
                rewind_until,
                self.limit.unwrap_or(10),
            )
            .await?;
        let mut executions = vec![];
        for signature in signatures {
            executions.push(endpoint.get_execution(&signature).await?);
        }

        let mut occurances_programs = HashMap::new();
        let mut occurances_accounts = HashMap::new();
        for execution in executions {
            for instruction in execution.instructions {
                match occurances_programs.entry(instruction.program_id) {
                    Entry::Vacant(entry) => {
                        entry.insert(1);
                    },
                    Entry::Occupied(mut entry) => {
                        *entry.get_mut() += 1;
                    },
                };
                for account in instruction.accounts {
                    match occurances_accounts.entry(account.pubkey) {
                        Entry::Vacant(entry) => {
                            entry.insert(1);
                        },
                        Entry::Occupied(mut entry) => {
                            *entry.get_mut() += 1;
                        },
                    };
                }
            }
        }

        let mut json_programs = Map::new();
        for occurances_program in occurances_programs {
            json_programs.insert(
                occurances_program.0.to_string(),
                json!(occurances_program.1),
            );
        }

        let mut json_accounts = Map::new();
        for occurances_account in occurances_accounts {
            json_accounts.insert(
                occurances_account.0.to_string(),
                json!(occurances_account.1),
            );
        }

        println!(
            "{}",
            serde_json::to_string(&json!({
                "programs": json_programs,
                "accounts": json_accounts,
            }))?
        );
        Ok(())
    }
}
