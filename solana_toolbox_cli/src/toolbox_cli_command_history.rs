use anyhow::Result;
use clap::Args;
use serde_json::json;
use serde_json::Value;

use crate::toolbox_cli_context::ToolboxCliContext;

#[derive(Debug, Clone, Args)]
#[command(about = "Search signatures that involve a specific account")]
pub struct ToolboxCliCommandHistoryArgs {
    #[arg(
        value_name = "PUBKEY",
        help = "The account pubkey that is involved in transactions"
    )]
    address: String,
    #[arg(
        value_name = "COUNT",
        help = "How many signature we'll read for before stopping"
    )]
    limit: usize,
    #[arg(value_name = "SIGNATURE")]
    start_before_signature: Option<String>,
    #[arg(value_name = "SIGNATURE")]
    rewind_until_signature: Option<String>,
    #[arg(
        display_order = 11,
        long = "name",
        value_name = "INSTRUCTION_NAME",
        help = "Expect matching parsed IDL instruction name"
    )]
    name: Option<String>,
}

impl ToolboxCliCommandHistoryArgs {
    pub async fn process(&self, context: &ToolboxCliContext) -> Result<Value> {
        let mut endpoint = context.create_endpoint().await?;
        let mut idl_service = context.create_service().await?;
        let address = context.parse_key(&self.address)?.address();
        let start_before = self
            .start_before_signature
            .as_ref()
            .map(|signature| context.parse_signature(signature))
            .transpose()?;
        let rewind_until = self
            .rewind_until_signature
            .as_ref()
            .map(|signature| context.parse_signature(signature))
            .transpose()?;
        let signatures = endpoint
            .search_signatures(&address, self.limit, start_before, rewind_until)
            .await?;
        let mut json_history = vec![];
        for signature in signatures {
            let mut json_instructions = vec![];
            let execution = endpoint.get_execution(&signature).await?;
            let mut filtered_out = self.name.is_some();
            for instruction in execution.instructions {
                match idl_service
                    .infer_and_decode_instruction(&mut endpoint, &instruction)
                    .await
                {
                    Ok(instruction_info) => {
                        let instruction_name = context
                            .compute_instruction_name(
                                &instruction_info.program,
                                &instruction_info.instruction,
                            );
                        if let Some(name) = &self.name {
                            if instruction_name.contains(name) {
                                filtered_out = false;
                            }
                        }
                        json_instructions.push(json!({
                            "program_id": instruction.program_id.to_string(),
                            "name": instruction_name,
                            "payload": instruction_info.payload,
                        }))
                    },
                    Err(error) => {
                        json_instructions.push(json!({
                            "program_id": instruction.program_id.to_string(),
                            "decode_error": context.compute_error_json(error),
                        }));
                    },
                };
            }
            if !filtered_out {
                json_history.push(json!({
                    "signature": signature.to_string(),
                    "instructions": json_instructions,
                    "explorer_url": context.compute_explorer_signature_url(&signature),
                }));
            }
        }
        Ok(json!(json_history))
    }
}
