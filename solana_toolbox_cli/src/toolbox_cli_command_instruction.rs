use std::collections::HashMap;

use clap::Args;
use serde_json::json;
use serde_json::Map;
use solana_sdk::bs58;
use solana_sdk::transaction::Transaction;

use crate::toolbox_cli_config::ToolboxCliConfig;
use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Args)]
#[command(about = "Prepare an instruction using its program's IDL")]
pub struct ToolboxCliCommandInstructionArgs {
    #[arg(
        value_name = "PROGRAM_ID",
        help = "The instruction's ProgramID pubkey"
    )]
    program_id: String,
    #[arg(
        value_name = "INSTRUCTION_NAME",
        help = "The instruction's name from the IDL"
    )]
    name: String,
    #[arg(
        value_name = "JSON",
        help = "The instruction's args object in JSON format"
    )]
    payload: String,
    #[arg(
        value_delimiter = ',',
        value_name = "NAME:KEY",
        help = "The instruction's accounts, format: [Name:[Pubkey|KeypairFile|'KEYPAIR']]"
    )]
    accounts: Vec<String>,
    #[arg(long, help = "Execute generated instruction instead of simulate")]
    execute: bool,
    // TODO (SHORT) - set compute budget / price
}

impl ToolboxCliCommandInstructionArgs {
    pub async fn process(
        &self,
        config: &ToolboxCliConfig,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = config.create_endpoint().await?;
        let mut idl_service = config.create_idl_service().await?;
        let instruction_program_id =
            config.parse_key(&self.program_id)?.address();
        let instruction_name = &self.name;
        let idl_program = match idl_service
            .resolve_program(&mut endpoint, &instruction_program_id)
            .await?
        {
            Some(idl_program) => idl_program,
            None => Err(ToolboxCliError::Custom(format!(
                "Could not resolve program with program_id: {}",
                instruction_program_id.to_string(),
            )))?,
        };
        let idl_instruction =
            match idl_program.instructions.get(instruction_name).cloned() {
                Some(idl_instruction) => idl_instruction,
                None => Err(ToolboxCliError::Custom(format!(
                    "Could not find instruction {}, available: {:?}",
                    instruction_name,
                    idl_program.instructions.keys()
                )))?,
            };
        let instruction_payload = config.parse_json(&self.payload)?;
        let mut instruction_keys = HashMap::new();
        for account in &self.accounts {
            let (name, key) = config.parse_account(account)?;
            instruction_keys.insert(name, key);
        }
        let mut instruction_addresses = HashMap::new();
        for (name, key) in &instruction_keys {
            instruction_addresses.insert(name.to_string(), key.address());
        }
        let instruction_addresses = idl_service
            .resolve_instruction_addresses(
                &mut endpoint,
                &idl_instruction,
                &instruction_program_id,
                &instruction_payload,
                &instruction_addresses,
            )
            .await?;
        let instruction_encode_result = idl_instruction.encode(
            &instruction_program_id,
            &instruction_payload,
            &instruction_addresses,
        );
        let mut json_addresses = Map::new();
        for instruction_address in &instruction_addresses {
            json_addresses.insert(
                instruction_address.0.to_string(),
                json!(instruction_address.1.to_string()),
            );
        }
        let (payload_dependencies, addresses_dependencies) =
            idl_instruction.get_dependencies();
        let mut json_dependencies_addresses = Map::new();
        for address_dependency in addresses_dependencies {
            if instruction_addresses.contains_key(&address_dependency.0) {
                continue;
            }
            json_dependencies_addresses
                .insert(address_dependency.0, json!(address_dependency.1));
        }
        let json_dependencies = json!({
            "payload": payload_dependencies,
            "addresses": json_dependencies_addresses,
        });
        let mut json_outcome = Map::new();
        match instruction_encode_result {
            Ok(instruction) => {
                // TODO - provide link to simulation explorer instead of encoded
                json_outcome.insert(
                    "message_base58".to_string(),
                    json!(bs58::encode(
                        Transaction::new_with_payer(
                            &[instruction.clone()],
                            None
                        )
                        .message
                        .serialize(),
                    )
                    .into_string()),
                );
                let mut signers = vec![];
                for key in instruction_keys.values() {
                    if let Some(signer) = key.signer() {
                        signers.push(signer);
                    }
                }
                if self.execute {
                    let (signature, _) = endpoint
                        .process_instruction_with_signers(
                            &config.get_keypair(),
                            instruction.clone(),
                            &signers,
                        )
                        .await?;
                    json_outcome.insert(
                        "signature".to_string(),
                        json!(signature.to_string()),
                    );
                } else {
                    let simulation = endpoint
                        .simulate_instruction_with_signers(
                            &config.get_keypair(),
                            instruction.clone(),
                            &signers,
                        )
                        .await?;
                    json_outcome.insert(
                        "simulation".to_string(),
                        json!({
                            "error": simulation.error,
                            "logs": simulation.logs,
                            "return_data": simulation.return_data,
                            "units_consumed": simulation.units_consumed,
                        }),
                    );
                }
            },
            Err(error) => {
                json_outcome
                    .insert("error".to_string(), json!(format!("{:?}", error)));
            },
        };
        println!(
            "{}",
            serde_json::to_string(&json!({
                "resolved": {
                    "payload": instruction_payload,
                    "addresses": json_addresses,
                },
                "dependencies": json_dependencies,
                "outcome": json_outcome,
            }))?
        );
        Ok(())
    }
}
