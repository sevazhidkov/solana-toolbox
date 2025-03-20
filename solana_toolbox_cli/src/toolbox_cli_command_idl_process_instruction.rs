use std::collections::HashMap;
use std::str::FromStr;

use clap::Args;
use clap::ValueHint;
use serde_json::from_str;
use serde_json::json;
use serde_json::Value;
use solana_cli_config::Config;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_idl::ToolboxIdlResolver;

use crate::toolbox_cli_error::ToolboxCliError;
use crate::toolbox_cli_utils::ToolboxCliUtils;

#[derive(Debug, Clone, Args)]
pub struct ToolboxCliCommandIdlProcessInstructionArgs {
    program_id: String,
    name: String,
    payload: String,
    #[arg(value_delimiter(','))]
    keys: Vec<String>,
    #[arg(short, long, value_hint(ValueHint::FilePath))]
    payer: Option<String>,
}

impl ToolboxCliCommandIdlProcessInstructionArgs {
    pub async fn process(
        &self,
        config: &Config,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = ToolboxCliUtils::new_endpoint(config)?;
        let payer = ToolboxCliUtils::load_keypair(
            self.payer.as_ref().unwrap_or(&config.keypair_path),
        )?;

        let program_id = Pubkey::from_str(&self.program_id).unwrap();

        let instruction_name = &self.name;

        let mut instruction_keys = HashMap::new();
        instruction_keys.insert(
            "payer".to_string(),
            KeypairOrPubkey::Keypair(payer.insecure_clone()),
        );
        for key in &self.keys {
            let parts = key.split(":").collect::<Vec<_>>();
            if let [name, value] = parts[..] {
                instruction_keys.insert(name.to_string(), parse_key(value)?);
            } else {
                return Err(ToolboxCliError::Custom(
                    "Invalid account key-value".to_string(),
                ));
            }
        }
        let mut instruction_addresses = HashMap::new();
        for instruction_key in &instruction_keys {
            instruction_addresses.insert(
                instruction_key.0.to_string(),
                match instruction_key.1 {
                    KeypairOrPubkey::Keypair(keypair) => keypair.pubkey(),
                    KeypairOrPubkey::Pubkey(pubkey) => *pubkey,
                },
            );
        }

        let instruction_payload = from_str::<Value>(&self.payload)?;

        let instruction = ToolboxIdlResolver::new()
            .resolve_instruction(
                &mut endpoint,
                &program_id,
                instruction_name,
                &instruction_addresses,
                &instruction_payload,
            )
            .await?;

        let mut signers = vec![];
        for instruction_key in &instruction_keys {
            if let KeypairOrPubkey::Keypair(keypair) = instruction_key.1 {
                signers.push(keypair);
            }
        }

        let (signature, _execution) = endpoint
            .process_instruction_with_signers(&payer, instruction, &signers)
            .await?;

        println!(
            "{}",
            serde_json::to_string(&json!({
                "signature": signature.to_string(),
                // TODO - output execution same as idl_resolve_execution
            }))?
        );
        Ok(())
    }
}

enum KeypairOrPubkey {
    Keypair(Keypair),
    Pubkey(Pubkey),
}

fn parse_key(value: &str) -> Result<KeypairOrPubkey, ToolboxCliError> {
    Ok(if let Ok(keypair) = read_keypair_file(value) {
        KeypairOrPubkey::Keypair(keypair)
    } else {
        KeypairOrPubkey::Pubkey(Pubkey::from_str(value)?)
    })
}
