use anyhow::anyhow;
use anyhow::Result;
use solana_sdk::hash::Hash;
use solana_sdk::hash::Hasher;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::SeedDerivable;

use crate::toolbox_endpoint::ToolboxEndpoint;

impl ToolboxEndpoint {
    pub fn hash_bytes(value: &[u8]) -> Hash {
        let mut hasher = Hasher::default();
        hasher.hash(value);
        hasher.result()
    }

    pub fn hash_string(value: &str) -> Hash {
        ToolboxEndpoint::hash_bytes(value.as_bytes())
    }

    pub fn keypair_from_seed_string_hash(value: &str) -> Result<Keypair> {
        Keypair::from_seed(&ToolboxEndpoint::hash_string(value).to_bytes())
            .map_err(|err| {
                anyhow!("Failed to create keypair from seed: {}", err)
            })
    }
}
