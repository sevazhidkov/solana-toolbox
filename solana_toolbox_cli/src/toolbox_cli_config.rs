use std::str::FromStr;
use std::sync::Arc;

use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_idl::ToolboxIdlProgram;
use solana_toolbox_idl::ToolboxIdlResolver;
use tokio::fs::read_to_string;

use crate::toolbox_cli_error::ToolboxCliError;

pub struct ToolboxCliConfig {
    json_rpc_url: String,
    commitment: String,
    keypair_path: String,
    custom_idls: Vec<String>,
}

// TODO - should this have its own name+file ?
pub enum ToolboxCliConfigKeypairOrPubkey {
    Keypair(Arc<Keypair>),
    Pubkey(Pubkey),
}

impl ToolboxCliConfigKeypairOrPubkey {
    pub fn address(&self) -> Pubkey {
        match self {
            ToolboxCliConfigKeypairOrPubkey::Keypair(keypair) => {
                keypair.pubkey()
            },
            ToolboxCliConfigKeypairOrPubkey::Pubkey(pubkey) => *pubkey,
        }
    }

    pub fn signer(&self) -> Option<&Keypair> {
        match self {
            ToolboxCliConfigKeypairOrPubkey::Keypair(keypair) => Some(keypair),
            ToolboxCliConfigKeypairOrPubkey::Pubkey(_) => None,
        }
    }
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

    pub async fn create_resolver(
        &self,
    ) -> Result<ToolboxIdlResolver, ToolboxCliError> {
        let mut idl_resolver = ToolboxIdlResolver::new();
        for custom_idl in &self.custom_idls {
            let parts = custom_idl.split(":").collect::<Vec<_>>();
            if let [program_id, idl_file] = parts[..] {
                idl_resolver.preload_program(
                    &Pubkey::from_str(program_id)?,
                    ToolboxIdlProgram::try_parse_from_str(
                        &read_to_string(idl_file).await?,
                    )?,
                );
            } else {
                return Err(ToolboxCliError::Custom(
                    "Invalid idl, expected format: [ProgramId:IdlFilePath]"
                        .to_string(),
                ));
            }
        }
        Ok(idl_resolver)
    }

    pub fn parse_account(
        &self,
        account: &str,
    ) -> Result<(String, ToolboxCliConfigKeypairOrPubkey), ToolboxCliError>
    {
        let parts = account.split(":").collect::<Vec<_>>();
        if let [name, key] = parts[..] {
            Ok((name.to_string(), self.parse_key(key)?))
        } else {
            Err(ToolboxCliError::Custom(
                "Invalid account, expected format: [Name:[Pubkey|KeypairFile|'WALLET']]".to_string(),
            ))
        }
    }

    pub fn parse_key(
        &self,
        key: &str,
    ) -> Result<ToolboxCliConfigKeypairOrPubkey, ToolboxCliError> {
        if key == "WALLET" {
            return Ok(ToolboxCliConfigKeypairOrPubkey::Keypair(
                self.wallet.clone(),
            ));
        }
        Ok(if let Ok(keypair) = read_keypair_file(key) {
            ToolboxCliConfigKeypairOrPubkey::Keypair(keypair.into())
        } else {
            ToolboxCliConfigKeypairOrPubkey::Pubkey(Pubkey::from_str(key)?)
        })
    }

    pub fn get_wallet(&self) -> &Keypair {
        &read_keypair_file(self.keypair_path.clone())
            .unwrap_or(Keypair::new())
            .into()
    }
}
