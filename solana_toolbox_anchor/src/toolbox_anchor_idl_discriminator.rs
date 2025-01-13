use sha2::{Digest, Sha256};

use crate::toolbox_anchor_endpoint::ToolboxAnchorEndpoint;

impl ToolboxAnchorEndpoint {
    pub fn compute_anchor_account_discriminator(
        &self,
        account_type: &str,
    ) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("account:{}", account_type));
        let hash = hasher.finalize();
        let mut discriminator = [0u8; 8];
        discriminator.copy_from_slice(&hash[..8]);
        u64::from_le_bytes(discriminator)
    }
}
