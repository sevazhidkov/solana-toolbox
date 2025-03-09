use std::str::FromStr;

use clap::Args;
use serde_json::json;
use serde_json::Map;
use serde_json::Value;
use solana_cli_config::Config;
use solana_sdk::signature::Signature;
use solana_toolbox_idl::ToolboxIdl;

use crate::toolbox_cli_error::ToolboxCliError;
use crate::toolbox_cli_utils::ToolboxCliUtils;

#[derive(Debug, Clone, Args)]
pub struct ToolboxCliCommandIdlDecompileExecutionArgs {
    signature: String,
}

impl ToolboxCliCommandIdlDecompileExecutionArgs {
    pub async fn process(
        &self,
        config: &Config,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = ToolboxCliUtils::new_endpoint(config)?;
        let signature = Signature::from_str(&self.signature).unwrap();
        let execution = endpoint.get_execution(&signature).await?;
        let mut decompiled_instructions = vec![];
        for instruction in execution.instructions {
            let idl = ToolboxIdl::get_for_program_id(
                &mut endpoint,
                &instruction.program_id,
            )
            .await?
            .unwrap(); // TODO - handle unwrap
            decompiled_instructions
                .push(idl.decompile_instruction(&instruction)?);
        }
        let json = json!({
            "payer": execution.payer.to_string(),
            "instructions": decompiled_instructions.into_iter().map(|decompiled_instruction| {
                json!({
                    "program_id": decompiled_instruction.program_id.to_string(),
                    "name": decompiled_instruction.name,
                    "accounts_addresses": Value::Object(Map::from_iter(
                        decompiled_instruction.accounts_addresses.into_iter().map(|account_address_entry| {
                            (account_address_entry.0, Value::String(account_address_entry.1.to_string()))
                        })
                    )),
                    "args": decompiled_instruction.args,
                })
            }).collect::<Vec<_>>(),
            "logs": execution.logs,
            "error": execution.error, // TODO - could parse the error using the code
            "return_data": execution.return_data,
            "units_consumed": execution.units_consumed,
        });
        println!("{}", serde_json::to_string(&json)?);
        Ok(())
    }
}
