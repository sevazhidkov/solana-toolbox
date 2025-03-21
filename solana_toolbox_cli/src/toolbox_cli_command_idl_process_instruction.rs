use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

use clap::Args;
use serde_json::from_str;
use serde_json::json;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use solana_toolbox_idl::ToolboxIdlResolver;

use crate::toolbox_cli_config::ToolboxCliConfig;
use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Args)]
pub struct ToolboxCliCommandIdlProcessInstructionArgs {
    program_id: String,
    name: String,
    payload: String,
    #[arg(value_delimiter(','))]
    accounts: Vec<String>,
}

impl ToolboxCliCommandIdlProcessInstructionArgs {
    pub async fn process(
        &self,
        config: &ToolboxCliConfig,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = config.create_endpoint()?;
        let program_id = Pubkey::from_str(&self.program_id).unwrap();
        let instruction_name = &self.name;
        let instruction_payload = from_str::<Value>(&self.payload)?;
        let mut instruction_keys = HashMap::new();
        for account in &self.accounts {
            let (name, key) = config.parse_account(account)?;
            instruction_keys.insert(name, key);
        }
        let mut instruction_addresses = HashMap::new();
        for instruction_key in &instruction_keys {
            instruction_addresses.insert(
                instruction_key.0.to_string(),
                instruction_key.1.address(),
            );
        }
        let instruction = ToolboxIdlResolver::new()
            .resolve_instruction(
                &mut endpoint,
                &program_id,
                instruction_name,
                &instruction_addresses,
                &instruction_payload,
            )
            .await?;
        let mut instruction_signing_keys = HashSet::new();
        for instruction_account in &instruction.accounts {
            if instruction_account.is_signer {
                instruction_signing_keys.insert(instruction_account.pubkey);
            }
        }

        let mut signers_pubkeys = HashSet::new();
        let mut signers_keypairs = vec![];

        signers_pubkeys.insert(config.get_keypair()?.pubkey());

        for instruction_key in instruction_keys.values() {
            let instruction_key_address = instruction_key.address();
            if instruction_signing_keys.contains(&instruction_key_address) {
                if let Some(signer_keypair) = instruction_key.signer() {
                    if !signers_pubkeys.contains(&instruction_key_address) {
                        signers_pubkeys.insert(instruction_key_address);
                        signers_keypairs.push(signer_keypair);
                    }
                }
            }
        }
        let (signature, _execution) = endpoint
            .process_instruction_with_signers(
                &config.get_keypair()?,
                instruction,
                &signers_keypairs,
            )
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
