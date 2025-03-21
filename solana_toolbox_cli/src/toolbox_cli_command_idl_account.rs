use std::str::FromStr;

use clap::Args;
use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdlResolver;

use crate::toolbox_cli_config::ToolboxCliConfig;
use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Args)]
pub struct ToolboxCliCommandIdlAccountArgs {
    address: String,
    // idls: Vec<String>, // TODO - implement ?
}

// TODO - could this be merged with execution by checking if its a valid signature or not ?
impl ToolboxCliCommandIdlAccountArgs {
    pub async fn process(
        &self,
        config: &ToolboxCliConfig,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = config.create_endpoint()?;
        let address = Pubkey::from_str(&self.address).unwrap();
        let account_details = ToolboxIdlResolver::new()
            .resolve_account_details(&mut endpoint, &address)
            .await?
            .unwrap(); // TODO - unwrap error
        println!(
            "{}",
            serde_json::to_string(&json!({
                "name": account_details.0.name,
                "state": account_details.1,
            }))?
        );
        Ok(())
    }
}
