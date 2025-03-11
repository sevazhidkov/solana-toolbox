use std::collections::HashMap;
use std::str::FromStr;

use clap::Args;
use serde_json::from_str;
use serde_json::Map;
use serde_json::Value;
use solana_cli_config::Config;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdl;
use solana_toolbox_idl::ToolboxIdlTransactionInstruction;

use crate::toolbox_cli_error::ToolboxCliError;
use crate::toolbox_cli_utils::ToolboxCliUtils;

// TODO - resolve a single account instead ?
#[derive(Debug, Clone, Args)]
pub struct ToolboxCliCommandIdlResolveInstructionAccountsArgs {
    program_address: String,
    name: String,
    args: String,
    #[arg(value_delimiter(','))]
    accounts: Vec<String>,
}

impl ToolboxCliCommandIdlResolveInstructionAccountsArgs {
    pub async fn process(
        &self,
        config: &Config,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = ToolboxCliUtils::new_endpoint(config)?;

        let program_address = Pubkey::from_str(&self.program_address).unwrap();
        let idl =
            ToolboxIdl::get_for_program_id(&mut endpoint, &program_address)
                .await?
                .unwrap(); // TODO - handle unwrap

        let args = from_str::<Value>(&self.args)?;

        let mut accounts_addresses = HashMap::new();
        for account in &self.accounts {
            let parts = account.split(":").collect::<Vec<_>>();
            if let [key, value] = parts[..] {
                accounts_addresses
                    .insert(key.to_string(), Pubkey::from_str(value)?);
            } else {
                return Err(ToolboxCliError::Custom(
                    "Invalid account key-value".to_string(),
                ));
            }
        }

        let instruction_accounts_addresses = idl
            .resolve_instruction_accounts_addresses(
                &mut endpoint,
                &ToolboxIdlTransactionInstruction {
                    program_id: program_address,
                    name: self.name.to_string(),
                    accounts_addresses,
                    args,
                },
            )
            .await?;

        let json = Value::Object(Map::from_iter(
            instruction_accounts_addresses.iter().map(|entry| {
                (entry.0.to_string(), Value::String(entry.1.to_string()))
            }),
        ));
        println!("{}", serde_json::to_string(&json)?);
        Ok(())
    }
}
