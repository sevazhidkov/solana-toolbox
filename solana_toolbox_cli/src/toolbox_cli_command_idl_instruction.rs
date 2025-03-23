use std::collections::HashMap;
use std::str::FromStr;

use clap::Args;
use serde_json::from_str;
use serde_json::json;
use serde_json::Map;
use serde_json::Value;
use solana_sdk::bs58;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::Transaction;
use solana_toolbox_idl::ToolboxIdlResolver;

use crate::toolbox_cli_config::ToolboxCliConfig;
use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Args)]
#[command(about = "Prepare an instruction using its program's IDL")]
pub struct ToolboxCliCommandIdlInstructionArgs {
    #[arg(
        value_name = "PROGRAM_ID",
        help = "The instruction's ProgramID pubkey"
    )]
    program_id: String,
    #[arg(
        value_name = "INSTRUCTION_NAME",
        help = "The instruction's IDL name"
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
        help = "The instruction's accounts, format: [Name:[Pubkey|KeypairFile|'WALLET']]"
    )]
    accounts: Vec<String>,
    #[arg(
        long,
        help = "When specified, this flag actually tries to execute the generated instruction"
    )]
    execute: Option<()>,
}

impl ToolboxCliCommandIdlInstructionArgs {
    pub async fn process(
        &self,
        config: &ToolboxCliConfig,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = config.create_endpoint()?;
        let mut idl_resolver = ToolboxIdlResolver::new();
        let program_id = Pubkey::from_str(&self.program_id).unwrap();
        let instruction_name = &self.name;
        let idl_program = idl_resolver
            .resolve_program(&mut endpoint, &program_id)
            .await?;
        let idl_instruction =
            idl_program.instructions.get(instruction_name).unwrap();
        let mut instruction_addresses = HashMap::new();
        for account in &self.accounts {
            let (name, key) = config.parse_account(account)?;
            instruction_addresses.insert(name, key.address());
        }
        let instruction_payload = from_str::<Value>(&self.payload)?;
        let instruction_addresses = idl_resolver
            .resolve_instruction_addresses(
                &mut endpoint,
                &program_id,
                instruction_name,
                &instruction_addresses,
                &instruction_payload,
            )
            .await?;
        let instruction_compile_result = idl_instruction.compile(
            &program_id,
            &instruction_addresses,
            &instruction_payload,
        );
        let instruction_addresses_dependencies =
            idl_instruction.get_addresses_dependencies();
        let mut json_addresses = Map::new();
        for instruction_address in &instruction_addresses {
            json_addresses.insert(
                instruction_address.0.to_string(),
                json!(instruction_address.1.to_string()),
            );
        }
        let mut json_dependencies_missing = Map::new();
        for instruction_address_dependency in instruction_addresses_dependencies
        {
            if instruction_addresses
                .contains_key(&instruction_address_dependency.0)
            {
                continue;
            }
            json_dependencies_missing.insert(
                instruction_address_dependency.0,
                json!(instruction_address_dependency.1),
            );
        }
        let mut json_compile = Map::new();
        match instruction_compile_result {
            Ok(instruction) => {
                let mut json_compile_content = Map::new();
                json_compile_content.insert(
                    "program_id".to_string(),
                    json!(instruction.program_id.to_string()),
                );
                let mut json_compile_content_accounts = vec![];
                for instruction_account in &instruction.accounts {
                    json_compile_content_accounts.push(json!({
                        "address": instruction_account.pubkey.to_string(),
                        "is_writable": instruction_account.is_writable,
                        "is_signer": instruction_account.is_signer,
                    }));
                }
                json_compile_content.insert(
                    "accounts".to_string(),
                    json!(json_compile_content_accounts),
                );
                json_compile_content
                    .insert("data".to_string(), json!(instruction.data));
                json_compile
                    .insert("content".to_string(), json!(json_compile_content));
                json_compile.insert(
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
                let simulation = endpoint
                    .simulate_instruction(
                        config.get_wallet(),
                        instruction.clone(),
                    )
                    .await?;
                json_compile.insert(
                    "simulation".to_string(),
                    json!({
                        "error": simulation.error,
                        "logs": simulation.logs,
                        "return_data": simulation.return_data,
                    }),
                );
            },
            Err(error) => {
                json_compile
                    .insert("error".to_string(), json!(format!("{:?}", error)));
            },
        };
        println!(
            "{}",
            serde_json::to_string(&json!({
                "dependencies_missing": json_dependencies_missing,
                "addresses": json_addresses,
                "compile": json_compile,
            }))?
        );
        Ok(())
    }
}
