use clap::Args;
use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_cli_context::ToolboxCliContext;
use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Args)]
#[command(
    about = "Parse the outcome of a signature's execution using the involved programs IDL"
)]
pub struct ToolboxCliCommandExecutionArgs {
    #[arg(
        value_name = "SIGNATURE_BASE58",
        help = "The transaction's execution signature"
    )]
    signature: String,
}

impl ToolboxCliCommandExecutionArgs {
    pub async fn process(
        &self,
        context: &ToolboxCliContext,
    ) -> Result<Value, ToolboxCliError> {
        let mut endpoint = context.create_endpoint().await?;
        let mut idl_service = context.create_service().await?;
        let signature = context.parse_signature(&self.signature)?;
        let execution = endpoint.get_execution(&signature).await?;
        let mut json_instructions = vec![];
        for instruction in execution.instructions {
            let instruction_decoded = idl_service
                .decode_instruction(&mut endpoint, &instruction)
                .await?;
            let mut json_addresses = Map::new();
            for (name, address) in instruction_decoded.addresses {
                let instruction_account_decoded = idl_service
                    .get_and_decode_account(&mut endpoint, &address)
                    .await?;
                json_addresses.insert(
                    name,
                    json!({
                        "kind": context.compute_account_kind(
                            &instruction_account_decoded.owner,
                            &instruction_account_decoded.program,
                            &instruction_account_decoded.account
                        ),
                        "address": address.to_string(),
                    }),
                );
            }
            json_instructions.push(json!({
                "kind": context.compute_instruction_kind(
                    &instruction.program_id,
                    &instruction_decoded.program,
                    &instruction_decoded.instruction
                ),
                "addresses": json_addresses,
                "payload": instruction_decoded.payload,
            }));
        }
        Ok(json!({
            "payer": execution.payer.to_string(),
            "instructions": json_instructions,
            "logs": execution.logs,
            "error": execution.error, // TODO (MEDIUM) - could parse the error using the code
            "return_data": execution.return_data,
            "units_consumed": execution.units_consumed,
        }))
    }
}
