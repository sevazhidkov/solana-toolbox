use anyhow::Result;
use clap::Args;
use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_cli_context::ToolboxCliContext;

#[derive(Debug, Clone, Args)]
#[command(
    about = "Parse the outcome of a signature's execution using the involved programs IDL"
)]
pub struct ToolboxCliCommandExecutionArgs {
    #[arg(
        value_name = "SIGNATURE",
        help = "The transaction's execution signature"
    )]
    signature: String,
}

impl ToolboxCliCommandExecutionArgs {
    pub async fn process(&self, context: &ToolboxCliContext) -> Result<Value> {
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
                        "address": address.to_string(),
                        "owner": instruction_account_decoded.owner.to_string(),
                        "name": context.compute_account_name(
                            &instruction_account_decoded.program,
                            &instruction_account_decoded.account
                        ),
                    }),
                );
            }
            json_instructions.push(json!({
                "program_id": instruction.program_id.to_string(),
                "name": context.compute_instruction_name(
                    &instruction_decoded.program,
                    &instruction_decoded.instruction
                ),
                "payload": instruction_decoded.payload,
                "addresses": json_addresses,
            }));
        }
        Ok(json!({
            "payer": execution.payer.to_string(),
            "instructions": json_instructions,
            "logs": execution.logs, // TODO (MEDIUM) - parse events from logs (39VGeYvhykSQnAFyqARriRrU1DLjGj1jLstRoDShxK4EG5SNQtPM6NQM8dLBwt5kbn7poRn3cJj4xFPL1uKABz3h)
            "error": execution.error, // TODO (MEDIUM) - could parse the error using the code
            "return_data": execution.return_data,
            "units_consumed": execution.units_consumed,
        }))
    }
}
