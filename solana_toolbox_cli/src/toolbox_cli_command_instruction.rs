use std::collections::HashMap;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use clap::Args;
use serde_json::json;
use serde_json::Map;
use serde_json::Value;
use solana_sdk::transaction::Transaction;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_idl::ToolboxIdlPath;

use crate::toolbox_cli_context::ToolboxCliContext;

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
        display_order = 11,
        long = "arg",
        alias = "args",
        value_name = "JSON_PATH:JSON_VALUE",
        help = "Add a JSON value to the instruction payload (data)"
    )]
    args: Vec<String>,
    #[arg(
        display_order = 12,
        long = "signer",
        alias = "signers",
        value_name = "KEYPAIR_FILE_PATH",
        help = "Specify an extra signer keypair file for the instruction"
    )]
    signers: Vec<String>,
    #[arg(
        display_order = 13,
        long = "compute-budget",
        value_name = "COMPUTE_UNITS",
        help = "Set the compute budget unit limit for the instruction"
    )]
    compute_budget: Option<u32>,
    #[arg(
        display_order = 14,
        long = "compute-price",
        value_name = "MICRO_LAMPORTS",
        help = "Set the price of the compute unit for the instruction"
    )]
    compute_price: Option<u64>,
    #[arg(
        display_order = 15,
        long = "execute",
        action,
        help = "Execute the generated instruction instead of simulating it"
    )]
    execute: bool,
}

impl ToolboxCliCommandInstructionArgs {
    pub async fn process(&self, context: &ToolboxCliContext) -> Result<Value> {
        let mut endpoint = context.create_endpoint().await?;
        let mut idl_service = context.create_service().await?;

        let instruction_program_id =
            context.parse_key(&self.program_id)?.address();
        let instruction_name = self.name.clone().unwrap_or_default();

        let idl_program = match idl_service
            .load_program(&mut endpoint, &instruction_program_id)
            .await?
        {
            Some(idl_program) => idl_program,
            None => Err(anyhow!(
                "Could not resolve program with program_id: {}",
                instruction_program_id.to_string(),
            ))?,
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

        let mut instruction_payload = json!({});
        for arg in &self.args {
            if let Some((path, json)) = arg.split_once(":") {
                let path = ToolboxIdlPath::try_parse(path)
                    .context("Parse Arg JSON path")?;
                instruction_payload = path
                    .try_set_json_value(
                        Some(instruction_payload),
                        context.parse_hjson(json)?,
                    )
                    .context("Set Arg JSON value")?;
            }
        }

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

        let (explained_payload, explained_addresses) =
            idl_instruction.explained();
        let json_explained_payload = explained_payload.clone();
        let mut json_explained_addresses = Map::new();
        for (explained_address_name, explained_address_value) in
            &explained_addresses
        {
            if instruction_addresses.contains_key(explained_address_name) {
                continue;
            }
            json_explained_addresses.insert(
                explained_address_name.to_string(),
                json!(explained_address_value),
            );
        }
        let json_explained = json!({
            "payload": json_explained_payload,
            "addresses": json_explained_addresses,
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
                let instructions =
                    ToolboxEndpoint::generate_instructions_with_compute_budget(
                        &[instruction.clone()],
                        self.compute_budget,
                        self.compute_price,
                    );
                match ToolboxEndpoint::compile_transaction(
                    &context.get_keypair(),
                    &instructions,
                    &signers,
                    endpoint.get_latest_blockhash().await?,
                ) {
                    Ok(transaction) => {
                        if self.execute {
                            let (signature, _) = endpoint
                                .process_transaction(transaction.clone(), true)
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
                                        &transaction.signatures,
                                        &transaction.message.serialize()
                                    )),
                            );
                            match endpoint
                                .simulate_transaction(
                                    transaction.clone(),
                                    true, // TODO - configurable ?
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
                                        context.compute_error_json(error),
                                    );
                                },
                            }
                        }
                    },
                    Err(error) => {
                        json_outcome.insert(
                            "transaction_compile_error".to_string(),
                            context.compute_error_json(error),
                        );
                    },
                }
            },
            Err(error) => {
                json_outcome.insert(
                    "instruction_compile_error".to_string(),
                    context.compute_error_json(error),
                );
            },
        };

        Ok(json!({
            "explained": json_explained,
            "resolved": json_resolved,
            "outcome": json_outcome,
        }))
    }
}
