use std::str::FromStr;

use clap::Args;
use serde_json::json;
use serde_json::Map;
use solana_sdk::signature::Signature;

use crate::toolbox_cli_config::ToolboxCliConfig;
use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Args)]
#[command(about = "Search signatures that involve a specific account")]
pub struct ToolboxCliCommandHistoryArgs {
    #[arg(help = "The account pubkey that is involved in transactions")]
    address: String,
    #[arg(help = "How much signature we'll search for before stopping")]
    limit: Option<usize>,
    #[arg()]
    start_before_signature: Option<String>,
    #[arg()]
    rewind_until_signature: Option<String>,
}

impl ToolboxCliCommandHistoryArgs {
    pub async fn process(
        &self,
        config: &ToolboxCliConfig,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = config.create_endpoint().await?;
        let mut idl_resolver = config.create_resolver().await?;
        let address = config.parse_key(&self.address)?.address();
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
                &address,
                start_before,
                rewind_until,
                self.limit.unwrap_or(5),
            )
            .await?;
        let mut json_history = vec![];
        for signature in signatures {
            let mut json_instructions = vec![];
            let execution = endpoint.get_execution(&signature).await?;
            for instruction in execution.instructions {
                let idl_program = idl_resolver
                    .resolve_program(&mut endpoint, &instruction.program_id)
                    .await?
                    .unwrap_or_default();
                let idl_instruction = idl_program
                    .guess_instruction(&instruction.data)
                    .unwrap_or_default();
                json_instructions.push(format!(
                    "{}.{}",
                    idl_program
                        .name
                        .clone()
                        .unwrap_or(instruction.program_id.to_string()),
                    idl_instruction.name
                ));
            }
            json_history.push(json!({
                "signature": signature.to_string(),
                "instructions": json_instructions,
            }));
        }
        println!("{}", serde_json::to_string(&json!(json_history))?);
        Ok(())
    }
}
