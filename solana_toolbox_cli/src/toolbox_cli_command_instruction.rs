use std::collections::HashMap;

use clap::Args;
use serde_json::json;
use serde_json::Map;
use serde_json::Value;
use solana_sdk::transaction::Transaction;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_cli_context::ToolboxCliContext;
use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Args)]
#[command(about = "Prepare an instruction using its program's IDL")]
pub struct ToolboxCliCommandInstructionArgs {
    #[arg(
        value_name = "PROGRAM_ID",
        help = "The instruction's Program_ID's pubkey"
    )]
    program_id: String,
    #[arg(
        value_name = "INSTRUCTION_NAME",
        help = "The instruction's name from the IDL"
    )]
    name: Option<String>,
    #[arg(
        value_name = "ACCOUNT_NAME:PUBKEY",
        help = "List the instruction's named accounts"
    )]
    accounts: Vec<String>,
    #[arg(
        long = "arg",
        value_name = "JSON_PATH:JSON_VALUE",
        help = "Add a JSON value to the instruction payload (data)"
    )]
    args: Vec<String>,
    #[arg(
        long = "signer",
        value_name = "KEYPAIR_FILE_PATH",
        help = "Specify an extra instruction signer keypair file"
    )]
    signers: Vec<String>,
    #[arg(
        long,
        action,
        help = "Execute generated instruction instead of simulating it"
    )]
    execute: bool,
    // TODO (SHORT) - set compute budget / price
}

impl ToolboxCliCommandInstructionArgs {
    pub async fn process(
        &self,
        context: &ToolboxCliContext,
    ) -> Result<Value, ToolboxCliError> {
        let mut endpoint = context.create_endpoint().await?;
        let mut idl_service = context.create_service().await?;

        let instruction_program_id =
            context.parse_key(&self.program_id)?.address();
        let instruction_name = self.name.clone().unwrap_or_default();

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

        let idl_instruction = match idl_program
            .instructions
            .get(&instruction_name)
            .cloned()
        {
            Some(idl_instruction) => idl_instruction,
            None => {
                return Ok(json!({
                    "outcome": {
                        "error": "Unknown instruction",
                    },
                    "instructions": idl_program.instructions.keys().collect::<Vec<_>>(),
                }))
            },
        };

        let mut instruction_payload_object = Map::new();
        for arg in &self.args {
            if let Some((path, json)) = arg.split_once(":") {
                object_set_value_at_path(
                    &mut instruction_payload_object,
                    path,
                    context.parse_hjson(json)?,
                );
            }
        }
        let instruction_payload = json!(instruction_payload_object);

        let mut instruction_keys = HashMap::new();
        for account in &self.accounts {
            let (name, key) = context.parse_account(account)?;
            instruction_keys.insert(name, key);
        }
        let mut instruction_addresses = HashMap::new();
        for (name, key) in &instruction_keys {
            instruction_addresses.insert(name.to_string(), key.address());
        }

        let mut instruction_signers = vec![];
        for signer in &self.signers {
            instruction_signers.push(context.load_keypair(signer));
        }

        let (instruction_specs_payload, instruction_specs_addresses) =
            idl_instruction.get_specs();
        let json_specs_payload = instruction_specs_payload.clone();
        let mut json_specs_addresses = Map::new();
        for (instruction_specs_address_name, instruction_specs_address_value) in
            &instruction_specs_addresses
        {
            if instruction_addresses
                .contains_key(instruction_specs_address_name)
            {
                continue;
            }
            json_specs_addresses.insert(
                instruction_specs_address_name.to_string(),
                json!(instruction_specs_address_value),
            );
        }
        let json_specs = json!({
            "payload": json_specs_payload,
            "addresses": json_specs_addresses,
        });

        let instruction_addresses = idl_service
            .resolve_instruction_addresses(
                &mut endpoint,
                &idl_instruction,
                &instruction_program_id,
                &instruction_payload,
                &instruction_addresses,
            )
            .await?;

        let json_resolved_payload = instruction_payload.clone();
        let mut json_resolved_addresses = Map::new();
        for instruction_address in &instruction_addresses {
            json_resolved_addresses.insert(
                instruction_address.0.to_string(),
                json!(instruction_address.1.to_string()),
            );
        }
        let json_resolved = json!({
            "payload": json_resolved_payload,
            "addresses": json_resolved_addresses,
        });

        let mut json_outcome = Map::new();
        match idl_instruction.encode(
            &instruction_program_id,
            &instruction_payload,
            &instruction_addresses,
        ) {
            Ok(instruction) => {
                json_outcome.insert(
                    "message_base58".to_string(),
                    json!(ToolboxEndpoint::encode_base58(
                        &Transaction::new_with_payer(
                            &[instruction.clone()],
                            None,
                        )
                        .message
                        .serialize(),
                    )),
                );
                let mut signers = vec![];
                for key in instruction_keys.values() {
                    if let Some(signer) = key.signer() {
                        signers.push(signer);
                    }
                }
                for instruction_signer in &instruction_signers {
                    signers.push(instruction_signer);
                }
                match ToolboxEndpoint::compile_versioned_transaction(
                    &context.get_keypair(),
                    &[instruction.clone()],
                    &signers,
                    &[],
                    endpoint.get_latest_blockhash().await?,
                ) {
                    Ok(versioned_transaction) => {
                        if self.execute {
                            let (signature, _) = endpoint
                                .process_versioned_transaction(
                                    versioned_transaction.clone(),
                                    true,
                                )
                                .await?;
                            json_outcome.insert(
                                "signature".to_string(),
                                json!(signature.to_string()),
                            );
                            json_outcome.insert(
                                "explorer".to_string(),
                                json!(context.compute_explorer_signature_link(
                                    &signature
                                )),
                            );
                        } else {
                            json_outcome.insert(
                                "explorer".to_string(),
                                json!(context
                                    .compute_explorer_simulation_link(
                                        &versioned_transaction.signatures,
                                        &versioned_transaction
                                            .message
                                            .serialize()
                                    )),
                            );
                            match endpoint
                                .simulate_versioned_transaction(
                                    versioned_transaction.clone(),
                                )
                                .await
                            {
                                Ok(simulation) => {
                                    json_outcome.insert(
                                        "simulation".to_string(),
                                        json!({
                                            "error": simulation.error,
                                            "logs": simulation.logs,
                                            "return_data": simulation.return_data,
                                            "units_consumed": simulation.units_consumed,
                                        }),
                                    );
                                },
                                Err(error) => {
                                    json_outcome.insert(
                                        "simulation_error".to_string(),
                                        json!(format!("{:?}", error)),
                                    );
                                },
                            }
                        }
                    },
                    Err(error) => {
                        json_outcome.insert(
                            "transaction_compile_error".to_string(),
                            json!(format!("{:?}", error)),
                        );
                    },
                }
            },
            Err(error) => {
                json_outcome.insert(
                    "instruction_compile_error".to_string(),
                    json!(format!("{:?}", error)),
                );
            },
        };

        Ok(json!({
            "specs": json_specs,
            "resolved": json_resolved,
            "outcome": json_outcome,
        }))
    }
}

fn object_set_value_at_path(
    object: &mut Map<String, Value>,
    path: &str,
    value: Value,
) {
    // TODO - support unamed append (index array)
    if let Some((key, path_child)) = path.split_once(".") {
        if let Some(object_value) = object.get_mut(key) {
            if let Some(object_child) = object_value.as_object_mut() {
                object_set_value_at_path(object_child, path_child, value);
                return;
            }
        }
        let mut object_child = Map::new();
        object_set_value_at_path(&mut object_child, path_child, value);
        object.insert(key.to_string(), json!(object_child));
        return;
    }
    object.insert(path.to_string(), value);
}
