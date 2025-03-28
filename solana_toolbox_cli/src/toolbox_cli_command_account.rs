use clap::Args;
use serde_json::json;
use serde_json::Value;

use crate::toolbox_cli_context::ToolboxCliContext;
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
        context: &ToolboxCliContext,
    ) -> Result<Value, ToolboxCliError> {
        let mut endpoint = context.create_endpoint().await?;
        let mut idl_service = context.create_service().await?;
        let address = context.parse_key(&self.address)?.address();
        let account_decoded = idl_service
            .get_and_decode_account(&mut endpoint, &address)
            .await?;
        Ok(json!({
            "address": address.to_string(),
            "owner": account_decoded.owner.to_string(),
            "lamports": account_decoded.lamports,
            "kind": context.compute_account_kind(
                &account_decoded.owner,
                &account_decoded.program,
                &account_decoded.account,
            ),
            "state": account_decoded.state,
            "explorer": context.compute_explorer_address_link(&address),
        }))
    }
}
