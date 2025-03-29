use std::str::FromStr;

use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{read_keypair_file, Keypair};
use solana_sdk::signer::Signer;

use crate::toolbox_cli_error::ToolboxCliError;

pub enum ToolboxCliKey {
    Keypair(Keypair),
    Pubkey(Pubkey),
}

impl ToolboxCliKey {
    pub fn try_parse(value: &str) -> Result<ToolboxCliKey, ToolboxCliError> {
        Ok(if let Ok(keypair) = read_keypair_file(value) {
            ToolboxCliKey::Keypair(keypair)
        } else {
            ToolboxCliKey::Pubkey(Pubkey::from_str(
                value.trim_matches(|c| !char::is_alphanumeric(c)),
            )?)
        })
    }

    pub fn address(&self) -> Pubkey {
        match self {
            ToolboxCliKey::Keypair(keypair) => keypair.pubkey(),
            ToolboxCliKey::Pubkey(pubkey) => *pubkey,
        }
    }

    pub fn signer(&self) -> Option<&Keypair> {
        match self {
            ToolboxCliKey::Keypair(keypair) => Some(keypair),
            ToolboxCliKey::Pubkey(_) => None,
        }
    }
}
