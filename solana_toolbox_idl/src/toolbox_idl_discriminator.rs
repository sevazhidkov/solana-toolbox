use sha2::Digest;
use sha2::Sha256;

use crate::toolbox_idl::ToolboxIdl;

impl ToolboxIdl {
    pub fn compute_account_discriminator(account_type: &str) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("account:{}", account_type));
        let hash = hasher.finalize();
        u64::from_le_bytes(hash[..8].try_into().unwrap())
    }

    pub fn compute_instruction_discriminator(instruction_name: &str) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("global:{}", instruction_name));
        let hash = hasher.finalize();
        u64::from_le_bytes(hash[..8].try_into().unwrap())
    }
}
