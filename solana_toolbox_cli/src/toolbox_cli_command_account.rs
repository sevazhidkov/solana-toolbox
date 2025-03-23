use std::str::FromStr;

use clap::Args;
use serde_json::json;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_cli_config::ToolboxCliConfig;
use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Args)]
#[command(about = "Parse the content of an account using its program's IDL")]
pub struct ToolboxCliCommandIdlAccountArgs {
    #[arg(value_name = "PUBKEY_BASE58", help = "Any account's address Pubkey")]
    address: String,
}

// TODO - could this be merged with execution by checking if its a valid signature or not ?
// TODO - this could instead be merged with raw_get_account and dev_account
impl ToolboxCliCommandIdlAccountArgs {
    pub async fn process(
        &self,
        config: &ToolboxCliConfig,
    ) -> Result<(), ToolboxCliError> {
        let address = Pubkey::from_str(&self.address)?;
        let mut endpoint = config.create_endpoint().await?;
        let account = endpoint.get_account_or_default(&address).await?;
        let mut idl_resolver = config.create_resolver().await?;
        let idl_program = idl_resolver
            .resolve_program(&mut endpoint, &account.owner)
            .await?;

        let idl_account = idl_program.guess_account(&account.data);

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
