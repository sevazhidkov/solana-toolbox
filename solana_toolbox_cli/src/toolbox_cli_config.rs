use std::str::FromStr;

use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_cli_error::ToolboxCliError;

pub struct ToolboxCliConfig {
    json_rpc_url: String,
    commitment: String,
    keypair_path: String,
}

// TODO - should this have its own name+file ?
pub enum ToolboxCliConfigKeypairOrPubkey {
    Keypair(Keypair),
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
    ) -> ToolboxCliConfig {
        ToolboxCliConfig {
            json_rpc_url,
            commitment,
            keypair_path,
        }
    }

    pub fn create_endpoint(&self) -> Result<ToolboxEndpoint, ToolboxCliError> {
        Ok(ToolboxEndpoint::new_rpc_with_url_or_moniker_and_commitment(
            &self.json_rpc_url,
            CommitmentConfig::from_str(&self.commitment)?,
        ))
    }

    pub fn parse_account(
        &self,
        account: &str,
    ) -> Result<(String, ToolboxCliConfigKeypairOrPubkey), ToolboxCliError>
    {
        let parts = account.split(":").collect::<Vec<_>>();
        if let [name, key] = parts[..] {
            return Ok((name.to_string(), self.parse_key(key)?));
        } else {
            return Err(ToolboxCliError::Custom(
                "Invalid account, expected format: [name:[Pubkey|KeypairFile|'WALLET']]".to_string(),
            ));
        }
    }

    pub fn parse_key(
        &self,
        key: &str,
    ) -> Result<ToolboxCliConfigKeypairOrPubkey, ToolboxCliError> {
        if key == "WALLET" {
            return Ok(ToolboxCliConfigKeypairOrPubkey::Keypair(
                read_keypair_file(&self.keypair_path).unwrap(),
            ));
        }
        // TODO - smarter parsing using maybe URIs?
        Ok(if let Ok(keypair) = read_keypair_file(key) {
            ToolboxCliConfigKeypairOrPubkey::Keypair(keypair)
        } else {
            ToolboxCliConfigKeypairOrPubkey::Pubkey(Pubkey::from_str(key)?)
        })
    }

    // TODO - what's our standard naming for this?
    pub fn get_wallet(&self) -> Result<Keypair, ToolboxCliError> {
        read_keypair_file(&self.keypair_path).ok().ok_or_else(|| {
            ToolboxCliError::Custom("Could not read config keypair".to_string())
        })
    }
}
