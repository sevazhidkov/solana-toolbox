use std::collections::HashMap;
use std::str::FromStr;

use clap::Args;
use serde_json::from_str;
use serde_json::json;
use serde_json::Value;
use solana_cli_config::Config;
use solana_sdk::bs58;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::Transaction;
use solana_toolbox_idl::ToolboxIdlResolver;

use crate::toolbox_cli_error::ToolboxCliError;
use crate::toolbox_cli_utils::ToolboxCliUtils;

#[derive(Debug, Clone, Args)]
pub struct ToolboxCliCommandIdlResolveInstructionBase58Args {
    program_id: String, // TODO - could take IDL as param also ?
    name: String,
    payload: String,
    #[arg(value_delimiter(','))]
    addresses: Vec<String>,
}

impl ToolboxCliCommandIdlResolveInstructionBase58Args {
    pub async fn process(
        &self,
        config: &Config,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = ToolboxCliUtils::new_endpoint(config)?;
        let mut idl_resolver = ToolboxIdlResolver::new();
        let program_id = Pubkey::from_str(&self.program_id).unwrap();
        let instruction_name = &self.name;
        let mut instruction_addresses = HashMap::new();
        for account in &self.addresses {
            let parts = account.split(":").collect::<Vec<_>>();
            if let [key, value] = parts[..] {
                instruction_addresses
                    .insert(key.to_string(), Pubkey::from_str(value)?);
            } else {
                return Err(ToolboxCliError::Custom(
                    "Invalid account key-value".to_string(),
                ));
            }
        }
        let instruction_payload = from_str::<Value>(&self.payload)?;
        let instruction = idl_resolver
            .resolve_instruction(
                &mut endpoint,
                &program_id,
                instruction_name,
                &instruction_addresses,
                &instruction_payload,
            )
            .await?;
        let transaction = Transaction::new_with_payer(&[instruction], None);
        let transaction_message_base58 =
            bs58::encode(transaction.message().serialize()).into_vec();
        println!(
            "{}",
            serde_json::to_string(&json!({
                "message_base58": transaction_message_base58,
            }))?
        );
        Ok(())
    }
}
