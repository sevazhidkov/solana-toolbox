use std::str::FromStr;

use clap::Args;
use serde_json::json;
use solana_cli_config::Config;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdlResolver;

use crate::toolbox_cli_error::ToolboxCliError;
use crate::toolbox_cli_utils::ToolboxCliUtils;

#[derive(Debug, Clone, Args)]
pub struct ToolboxCliCommandIdlResolveAccountArgs {
    address: String,
    // TODO - should support loading a custom IDL ?
}

// TODO - could this be merged with execution by checking if its a valid signature or not ?
impl ToolboxCliCommandIdlResolveAccountArgs {
    pub async fn process(
        &self,
        config: &Config,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = ToolboxCliUtils::new_endpoint(config)?;
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
