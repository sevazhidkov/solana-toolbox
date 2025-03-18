use std::str::FromStr;

use clap::Args;
use serde_json::json;
use serde_json::Map;
use serde_json::Value;
use solana_cli_config::Config;
use solana_sdk::signature::Signature;
use solana_toolbox_idl::ToolboxIdlResolver;

use crate::toolbox_cli_error::ToolboxCliError;
use crate::toolbox_cli_utils::ToolboxCliUtils;

#[derive(Debug, Clone, Args)]
pub struct ToolboxCliCommandIdlResolveExecutionArgs {
    signature: String,
    // TODO - allow custom IDLs
}

impl ToolboxCliCommandIdlResolveExecutionArgs {
    pub async fn process(
        &self,
        config: &Config,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = ToolboxCliUtils::new_endpoint(config)?;
        let mut idl_resolver = ToolboxIdlResolver::new();
        let signature = Signature::from_str(&self.signature).unwrap();
        let execution = endpoint.get_execution(&signature).await?;
        let mut json_instructions = vec![];
        for instruction in execution.instructions {
            let idl_program = idl_resolver
                .resolve_idl_program(&mut endpoint, &instruction.program_id)
                .await?;
            let (program_id, instruction_addresses, instruction_payload) =
                idl_program.guess_idl_instruction(&instruction.data)
                .unwrap() // TODO - handle unwrap
                .decompile(&instruction)?;
            let mut json_addresses = vec![];
            for instruction_address in instruction_addresses {
                json_addresses.push(json!([
                    instruction_address.0,
                    instruction_address.1.to_string()
                ]));
            }
            json_instructions.push(json!({
                "program_id": program_id.to_string(),
                "addresses": json_addresses,
                "payload": instruction_payload,
            }));
        }
        let json = json!({
            "payer": execution.payer.to_string(),
            "instructions": json_instructions,
            "logs": execution.logs,
            "error": execution.error, // TODO - could parse the error using the code
            "return_data": execution.return_data,
            "units_consumed": execution.units_consumed,
        });
        println!("{}", serde_json::to_string(&json)?);
        Ok(())
    }
}
