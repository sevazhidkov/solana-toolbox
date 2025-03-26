use std::str::FromStr;

use clap::Args;
use serde_json::json;
use serde_json::Map;
use solana_sdk::signature::Signature;

use crate::toolbox_cli_config::ToolboxCliConfig;
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
        config: &ToolboxCliConfig,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = config.create_endpoint().await?;
        let mut idl_service = config.create_idl_service().await?;
        let signature = Signature::from_str(&self.signature).unwrap();
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
                    json!(format!(
                        "{} ({}.{})",
                        address.to_string(),
                        instruction_account_decoded
                            .program
                            .metadata
                            .name
                            .clone()
                            .unwrap_or(
                                instruction_account_decoded.owner.to_string()
                            ),
                        instruction_account_decoded.account.name,
                    )),
                );
            }
            json_instructions.push(json!({
                "program": instruction_decoded.program.metadata.name.clone().unwrap_or(instruction.program_id.to_string()),
                "name": instruction_decoded.instruction.name,
                "addresses": json_addresses,
                "payload": instruction_decoded.payload,
            }));
        }
        println!(
            "{}",
            serde_json::to_string(&json!({
                "payer": execution.payer.to_string(),
                "instructions": json_instructions,
                "logs": execution.logs,
                "error": execution.error, // TODO (MEDIUM) - could parse the error using the code
                "return_data": execution.return_data,
                "units_consumed": execution.units_consumed,
            }))?
        );
        Ok(())
    }
}
