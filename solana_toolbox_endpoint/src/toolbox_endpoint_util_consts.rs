use solana_sdk::native_loader;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::sysvar;

use crate::toolbox_endpoint::ToolboxEndpoint;

impl ToolboxEndpoint {
    pub const LAMPORTS_PER_SIGNATURE: u64 = 5_000;
    pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;
    pub const NATIVE_LOADER_PROGRAM_ID: Pubkey = native_loader::ID;
    pub const SYSVAR_PROGRAM_ID: Pubkey = sysvar::ID;
    pub const TRANSACTION_DATA_SIZE_LIMIT: usize = 1232;
}
