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
        let mut idl_resolver = config.create_resolver().await?;
        let address = config.parse_key(&self.address)?.address();
        let account = endpoint.get_account(&address).await?.unwrap_or_default();
        let idl_program = idl_resolver
            .resolve_program(&mut endpoint, &account.owner)
            .await?
            .unwrap_or_default();
        let idl_account =
            idl_program.guess_account(&account.data).unwrap_or_default();
        println!(
            "{}",
            serde_json::to_string(&json!({
                "metadata": {
                    "address": address.to_string(),
                    "owner": account.owner.to_string(),
                    "lamports": account.lamports,
                    "space": account.data.len(),
                    "executable": account.executable,
                },
                "idl": {
                    "kind": format!(
                        "{}.{}",
                        idl_program.name.clone().unwrap_or(account.owner.to_string()),
                        idl_account.name
                    ),
                    "state": idl_account.decompile(&account.data)?,
                },
                "data": account.data,
            }))?
        );
        Ok(())
    }
}
