use std::str::FromStr;

use clap::Args;
use serde_json::json;
use solana_cli_config::Config;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;

use crate::toolbox_cli_error::ToolboxCliError;
use crate::toolbox_cli_utils::ToolboxCliUtils;

#[derive(Debug, Clone, Args)]
pub struct ToolboxCliCommandRawSearchSignaturesArgs {
    with_address: String,
    start_before_signature: Option<String>,
    rewind_until_signature: Option<String>,
    limit: Option<usize>,
}

impl ToolboxCliCommandRawSearchSignaturesArgs {
    pub async fn process(
        &self,
        config: &Config,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = ToolboxCliUtils::new_endpoint(config)?;
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
                self.limit.unwrap_or(10),
            )
            .await?;
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
