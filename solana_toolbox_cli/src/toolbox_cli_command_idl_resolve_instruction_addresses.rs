use std::collections::HashMap;
use std::str::FromStr;

use clap::Args;
use serde_json::from_str;
use serde_json::Map;
use serde_json::Value;
use solana_cli_config::Config;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdlResolver;

use crate::toolbox_cli_error::ToolboxCliError;
use crate::toolbox_cli_utils::ToolboxCliUtils;

#[derive(Debug, Clone, Args)]
pub struct ToolboxCliCommandIdlResolveInstructionAddressesArgs {
    program_id: String, // TODO - could take IDL as param also ?
    name: String,
    #[arg(value_delimiter(','))]
    addresses: Vec<String>,
    #[arg(long)]
    payload: Option<String>,
}

impl ToolboxCliCommandIdlResolveInstructionAddressesArgs {
    pub async fn process(
        &self,
        config: &Config,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = ToolboxCliUtils::new_endpoint(config)?;
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
        let instruction_payload = from_str::<Value>(
            &self.payload.clone().unwrap_or("{}".to_string()),
        )?;
        let instruction_addresses = ToolboxIdlResolver::new()
            .resolve_instruction_addresses(
                &mut endpoint,
                &program_id,
                instruction_name,
                &instruction_addresses,
                &instruction_payload,
            )
            .await?;
        let json = Value::Object(Map::from_iter(
            instruction_addresses.iter().map(|entry| {
                (entry.0.to_string(), Value::String(entry.1.to_string()))
            }),
        ));
        println!("{}", serde_json::to_string(&json)?);
        Ok(())
    }
}
