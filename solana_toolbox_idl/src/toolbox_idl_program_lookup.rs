use std::collections::HashMap;

use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program::ToolboxIdlProgram;

impl ToolboxIdlProgram {
    pub fn from_lib(program_id: &Pubkey) -> Option<ToolboxIdlProgram> {
        let mut known_programs = HashMap::new();
        known_programs.insert(
            ToolboxEndpoint::SYSTEM_PROGRAM_ID,
            include_str!("lib/native_system.json"),
        );
        known_programs.insert(
            ToolboxEndpoint::ADDRESS_LOOKUP_TABLE_PROGRAM_ID,
            include_str!("lib/native_address_lookup_table.json"),
        );
        known_programs.insert(
            ToolboxEndpoint::COMPUTE_BUDGET_PROGRAM_ID,
            include_str!("lib/native_compute_budget.json"),
        );
        known_programs.insert(
            ToolboxEndpoint::NATIVE_LOADER_PROGRAM_ID,
            include_str!("lib/native_loader.json"),
        );
        known_programs.insert(
            ToolboxEndpoint::BPF_LOADER_2_PROGRAM_ID,
            include_str!("lib/native_bpf_loader_2.json"),
        );
        known_programs.insert(
            ToolboxEndpoint::BPF_LOADER_UPGRADEABLE_PROGRAM_ID,
            include_str!("lib/native_bpf_loader_upgradeable.json"),
        );
        known_programs.insert(
            ToolboxEndpoint::SPL_TOKEN_PROGRAM_ID,
            include_str!("lib/spl_token.json"),
        );
        known_programs.insert(
            ToolboxEndpoint::SPL_ASSOCIATED_TOKEN_PROGRAM_ID,
            include_str!("lib/spl_associated_token.json"),
        );
        known_programs.insert(
            pubkey!("L2TExMFKdjpN9kozasaurPirfHy9P8sbXoAN1qA3S95"),
            include_str!("lib/misc_lighthouse.json"),
        );
        known_programs.get(program_id).map(|content| {
            ToolboxIdlProgram::try_parse_from_str(content).unwrap()
        })
    }

    pub fn find_anchor(program_id: &Pubkey) -> Result<Pubkey, ToolboxIdlError> {
        let base = Pubkey::find_program_address(&[], program_id).0;
        Pubkey::create_with_seed(&base, "anchor:idl", program_id)
            .map_err(ToolboxIdlError::Pubkey)
    }

    pub fn find_shank(program_id: &Pubkey) -> Result<Pubkey, ToolboxIdlError> {
        let base = Pubkey::find_program_address(&[], program_id).0;
        Pubkey::create_with_seed(&base, "shank:idl", program_id)
            .map_err(ToolboxIdlError::Pubkey)
    }
}
