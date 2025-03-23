use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};

pub enum ToolboxCliKey {
    Keypair(Keypair),
    Pubkey(Pubkey),
}

impl ToolboxCliKey {
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
