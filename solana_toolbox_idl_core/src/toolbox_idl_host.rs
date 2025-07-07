use solana_sdk::address_lookup_table::program as address_lookup_table_program;
use solana_sdk::bpf_loader;
use solana_sdk::bpf_loader_upgradeable;
use solana_sdk::compute_budget;
use solana_sdk::native_loader;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::system_program;
use spl_associated_token_account::{
    self, get_associated_token_address,
    ID as SPL_ASSOCIATED_TOKEN_PROGRAM_ID_CONST,
};
use spl_token::ID as SPL_TOKEN_PROGRAM_ID_CONST;

pub const SYSTEM_PROGRAM_ID: Pubkey = system_program::ID;
pub const ADDRESS_LOOKUP_TABLE_PROGRAM_ID: Pubkey =
    address_lookup_table_program::ID;
pub const COMPUTE_BUDGET_PROGRAM_ID: Pubkey = compute_budget::ID;
pub const NATIVE_LOADER_PROGRAM_ID: Pubkey = native_loader::ID;
pub const BPF_LOADER_2_PROGRAM_ID: Pubkey = bpf_loader::ID;
pub const BPF_LOADER_UPGRADEABLE_PROGRAM_ID: Pubkey =
    bpf_loader_upgradeable::ID;
pub const SPL_TOKEN_PROGRAM_ID: Pubkey = SPL_TOKEN_PROGRAM_ID_CONST;
pub const SPL_ASSOCIATED_TOKEN_PROGRAM_ID: Pubkey =
    SPL_ASSOCIATED_TOKEN_PROGRAM_ID_CONST;

pub fn find_spl_associated_token_account(
    owner: &Pubkey,
    mint: &Pubkey,
) -> Pubkey {
    get_associated_token_address(owner, mint)
}
