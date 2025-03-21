use std::collections::HashMap;
use std::str::FromStr;

use clap::Args;
use serde_json::from_str;
use serde_json::json;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdlResolver;

use crate::toolbox_cli_config::ToolboxCliConfig;
use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Args)]
#[command(about = "Process an instruction using its program's IDL")]
pub struct ToolboxCliCommandIdlProcessArgs {
    #[arg(help = "The instruction's ProgramID pubkey")]
    program_id: String,
    #[arg(help = "The instruction's name")]
    name: String,
    #[arg(help = "The instruction's args object in JSON format")]
    payload: String,
    #[arg(
        value_delimiter(','),
        help = "The instruction's accounts, format: [name:[Pubkey|KeypairFile|'WALLET']]"
    )]
    accounts: Vec<String>,
    // TODO - allow passing IDLs as parameter
}

impl ToolboxCliCommandIdlProcessArgs {
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
        let mut signers = vec![];
        for instruction_key in instruction_keys.values() {
            if let Some(signer_keypair) = instruction_key.signer() {
                signers.push(signer_keypair);
            }
        }
        let (signature, _execution) = endpoint
            .process_instruction_with_signers(
                &config.get_wallet()?,
                instruction,
                &signers,
            )
            .await?;
        println!(
            "{}",
            serde_json::to_string(
                &json!({ "signature": signature.to_string() })
            )?
        );
        Ok(())
    }
}
