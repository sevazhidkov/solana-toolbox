use clap::Args;
use serde_json::json;

use crate::toolbox_cli_config::ToolboxCliConfig;
use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Args)]
#[command(about = "Parse the content of an account using its program's IDL")]
pub struct ToolboxCliCommandAccountArgs {
    #[arg(value_name = "PUBKEY_BASE58", help = "Any account's address Pubkey")]
    address: String,
}

impl ToolboxCliCommandAccountArgs {
    pub async fn process(
        &self,
        config: &ToolboxCliConfig,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = config.create_endpoint().await?;
        let mut idl_service = config.create_idl_service().await?;
        let address = config.parse_key(&self.address)?.address();
        let account_decoded = idl_service
            .get_and_decode_account(&mut endpoint, &address)
            .await?;
        println!(
            "{}",
            serde_json::to_string(&json!({
                "address": address.to_string(),
                "owner": account_decoded.owner.to_string(),
                "lamports": account_decoded.lamports,
                "kind": format!(
                    "{}.{}",
                    account_decoded.program.metadata.name.clone().unwrap_or(account_decoded.owner.to_string()),
                    account_decoded.account.name
                ),
                "state": account_decoded.state,
            }))?
        );
        Ok(())
    }
}
