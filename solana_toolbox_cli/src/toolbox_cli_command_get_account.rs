use std::str::FromStr;

use clap::Args;
use serde_json::json;
use solana_cli_config::Config;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_cli_error::ToolboxCliError;
use crate::toolbox_cli_utils::ToolboxCliUtils;

#[derive(Debug, Clone, Args)]
pub struct ToolboxCliCommandGetAccountArgs {
    address: String,
}

impl ToolboxCliCommandGetAccountArgs {
    pub async fn process(
        &self,
        config: &Config,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = ToolboxCliUtils::new_endpoint(config)?;
        let address = Pubkey::from_str(&self.address)?;
        let account = endpoint.get_account_or_default(&address).await?;
        let json = json!({
            "address": address.to_string(),
            "owner": account.owner.to_string(),
            "lamports": account.lamports,
            "data": account.data,
            "executable": account.executable,
        });
        println!("{}", serde_json::to_string(&json)?);
        Ok(())
    }
}
