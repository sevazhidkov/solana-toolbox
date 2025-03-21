use std::str::FromStr;

use clap::Args;
use serde_json::json;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_cli_config::ToolboxCliConfig;
use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Args)]
#[command(about = "Get an account's information")]
pub struct ToolboxCliCommandRawGetAccountArgs {
    #[arg(help = "The account's pubkey")]
    address: String,
}

// TODO - this should probably be included in idl_account ??
// TODO - naming probably should be json-account ??
impl ToolboxCliCommandRawGetAccountArgs {
    pub async fn process(
        &self,
        config: &ToolboxCliConfig,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = config.create_endpoint()?;
        let address = Pubkey::from_str(&self.address)?;
        let account = endpoint.get_account_or_default(&address).await?;
        println!(
            "{}",
            serde_json::to_string(&json!({
                "address": address.to_string(),
                "owner": account.owner.to_string(),
                "lamports": account.lamports,
                "data": account.data,
                "executable": account.executable,
            }))?
        );
        Ok(())
    }
}
