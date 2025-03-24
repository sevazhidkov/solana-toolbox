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
        let mut idl_resolver = config.create_resolver().await?;
        let signature = Signature::from_str(&self.signature).unwrap();
        let execution = endpoint.get_execution(&signature).await?;
        let mut json_instructions = vec![];
        for instruction in execution.instructions {
            let idl_program = idl_resolver
                .resolve_program(&mut endpoint, &instruction.program_id)
                .await?
                .unwrap_or_default();
            let idl_instruction = idl_program
                .guess_instruction(&instruction.data)
                .unwrap_or_default();
            let (program_id, instruction_payload, instruction_addresses) =
                idl_instruction.decompile(&instruction)?;
            let mut json_addresses = Map::new();
            for (name, address) in instruction_addresses {
                let account =
                    endpoint.get_account(&address).await?.unwrap_or_default();
                let idl_program = idl_resolver
                    .resolve_program(&mut endpoint, &account.owner)
                    .await?
                    .unwrap_or_default();
                let idl_account = idl_program
                    .guess_account(&account.data)
                    .unwrap_or_default();
                json_addresses.insert(
                    name,
                    json!(format!(
                        "{} ({}.{})",
                        address.to_string(),
                        idl_program
                            .name
                            .clone()
                            .unwrap_or(account.owner.to_string()),
                        idl_account.name,
                    )),
                );
            }
            json_instructions.push(json!({
                "program": idl_program.name.clone().unwrap_or(program_id.to_string()),
                "name": idl_instruction.name,
                "addresses": json_addresses,
                "payload": instruction_payload,
            }));
        }
        println!(
            "{}",
            serde_json::to_string(&json!({
                "payer": execution.payer.to_string(),
                "instructions": json_instructions,
                "logs": execution.logs,
                "error": execution.error, // TODO - could parse the error using the code
                "return_data": execution.return_data,
                "units_consumed": execution.units_consumed,
            }))?
        );
        Ok(())
    }
}
