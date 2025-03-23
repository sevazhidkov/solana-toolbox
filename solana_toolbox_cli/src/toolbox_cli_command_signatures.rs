use std::str::FromStr;

use clap::Args;
use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;

use crate::toolbox_cli_config::ToolboxCliConfig;
use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Args)]
#[command(about = "Search signatures that involve a specific account")]
pub struct ToolboxCliCommandSignaturesArgs {
    #[arg(help = "The account pubkey that is involved in transactions")]
    with_address: String,
    #[arg(help = "How much signature we'll search for before stopping")]
    limit: Option<usize>,
    #[arg()]
    start_before_signature: Option<String>,
    #[arg()]
    rewind_until_signature: Option<String>,
}

impl ToolboxCliCommandSignaturesArgs {
    pub async fn process(
        &self,
        config: &ToolboxCliConfig,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = config.create_endpoint().await?;
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
                self.limit.unwrap_or(100),
            )
            .await?;
        // TODO - add IDL analysis
        println!(
            "{}",
            serde_json::to_string(&json!(signatures
                .iter()
                .map(|signature| signature.to_string())
                .collect::<Vec<_>>()))?
        );
        Ok(())
    }
}
