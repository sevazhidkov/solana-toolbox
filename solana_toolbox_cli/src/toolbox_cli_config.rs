use std::fs::read_to_string;
use std::str::FromStr;

use serde_json::Value;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::signature::Keypair;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_idl::ToolboxIdlProgram;
use solana_toolbox_idl::ToolboxIdlService;

use crate::toolbox_cli_error::ToolboxCliError;
use crate::toolbox_cli_key::ToolboxCliKey;

pub struct ToolboxCliConfig {
    json_rpc_url: String,
    commitment: String,
    keypair_path: String,
    custom_idls: Vec<String>,
}

impl ToolboxCliConfig {
    pub fn new(
        json_rpc_url: String,
        commitment: String,
        keypair_path: String,
        custom_idls: Vec<String>,
    ) -> ToolboxCliConfig {
        ToolboxCliConfig {
            json_rpc_url,
            commitment,
            keypair_path,
            custom_idls,
        }
    }

    pub async fn create_endpoint(
        &self,
    ) -> Result<ToolboxEndpoint, ToolboxCliError> {
        Ok(ToolboxEndpoint::new_rpc_with_url_or_moniker_and_commitment(
            &self.json_rpc_url,
            CommitmentConfig::from_str(&self.commitment)?,
        ))
    }

    pub async fn create_idl_service(
        &self,
    ) -> Result<ToolboxIdlService, ToolboxCliError> {
        let mut idl_service = ToolboxIdlService::new();
        for custom_idl in &self.custom_idls {
            let parts = custom_idl.split(":").collect::<Vec<_>>();
            if let [program_id, idl_file] = parts[..] {
                idl_service.preload_program(
                    &self.parse_key(program_id)?.address(),
                    Some(
                        ToolboxIdlProgram::try_parse_from_str(
                            &read_to_string(idl_file)?,
                        )?
                        .into(),
                    ),
                );
            } else {
                return Err(ToolboxCliError::Custom(
                    "Invalid idl, expected format: [ProgramId:IdlFile]"
                        .to_string(),
                ));
            }
        }
        Ok(idl_service)
    }

    pub fn parse_account(
        &self,
        account: &str,
    ) -> Result<(String, ToolboxCliKey), ToolboxCliError> {
        let parts = account.split(":").collect::<Vec<_>>();
        if let [name, key] = parts[..] {
            Ok((name.to_string(), self.parse_key(key)?))
        } else {
            Err(ToolboxCliError::Custom(
                "Invalid account, expected format: [Name:[Pubkey|KeypairFile|'KEYPAIR']]".to_string(),
            ))
        }
    }

    pub fn parse_key(
        &self,
        key: &str,
    ) -> Result<ToolboxCliKey, ToolboxCliError> {
        if key.to_ascii_uppercase() == "KEYPAIR"
            || key.to_ascii_uppercase() == "WALLET"
        {
            return Ok(ToolboxCliKey::Keypair(self.get_keypair()));
        }
        Ok(if let Ok(keypair) = read_keypair_file(key) {
            ToolboxCliKey::Keypair(keypair)
        } else {
            ToolboxCliKey::Pubkey(Pubkey::from_str(
                key.trim_matches(|c| !char::is_alphanumeric(c)),
            )?)
        })
    }

    pub fn parse_json(&self, value: &str) -> Result<Value, ToolboxCliError> {
        Ok(serde_hjson::from_str::<Value>(value)?)
    }

    pub fn get_keypair(&self) -> Keypair {
        read_keypair_file(self.keypair_path.clone()).unwrap_or(Keypair::new())
    }
}
