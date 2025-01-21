use sha2::Digest;
use sha2::Sha256;

use crate::toolbox_idl::ToolboxIdl;

impl ToolboxIdl {
    pub const DISCRIMINATOR: &[u8] =
        &[0x18, 0x46, 0x62, 0xBF, 0x3A, 0x90, 0x7B, 0x9E];

    pub fn compute_account_discriminator(account_name: &str) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(format!("account:{}", account_name));
        let hash = hasher.finalize();
        hash[..8].to_vec()
    }

    pub fn compute_instruction_discriminator(
        instruction_name: &str
    ) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(format!("global:{}", instruction_name));
        let hash = hasher.finalize();
        hash[..8].to_vec()
    }
}
