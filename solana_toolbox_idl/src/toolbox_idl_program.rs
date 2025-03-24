use std::collections::HashMap;
use std::sync::Arc;

use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_idl_account::ToolboxIdlAccount;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_instruction::ToolboxIdlInstruction;
use crate::toolbox_idl_transaction_error::ToolboxIdlTransactionError;
use crate::toolbox_idl_typedef::ToolboxIdlTypedef;
use crate::toolbox_idl_utils::idl_convert_to_value_name;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ToolboxIdlProgram {
    // TODO - parse metadata from all versions
    pub name: Option<String>,
    pub instructions: HashMap<String, Arc<ToolboxIdlInstruction>>,
    pub accounts: HashMap<String, Arc<ToolboxIdlAccount>>,
    pub errors: HashMap<String, Arc<ToolboxIdlTransactionError>>,
    pub typedefs: HashMap<String, Arc<ToolboxIdlTypedef>>,
}

impl ToolboxIdlProgram {
    pub fn sanitize_name(name: &str) -> String {
        idl_convert_to_value_name(name)
    }

    pub fn from_lib(program_id: &Pubkey) -> Option<ToolboxIdlProgram> {
        // TODO - provide cached standard implementation for basic contracts such as spl_token and system, and compute_budget ?
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

    pub fn guess_account(
        &self,
        account_data: &[u8],
    ) -> Option<Arc<ToolboxIdlAccount>> {
        for account in self.accounts.values() {
            if !account_data.starts_with(&account.discriminator) {
                continue;
            }
            if let Some(account_space) = account.space {
                if account_data.len() != account_space {
                    continue;
                }
            }
            return Some(account.clone());
        }
        None
    }

    pub fn guess_instruction(
        &self,
        instruction_data: &[u8],
    ) -> Option<Arc<ToolboxIdlInstruction>> {
        for instruction in self.instructions.values() {
            if !instruction_data.starts_with(&instruction.discriminator) {
                continue;
            }
            return Some(instruction.clone());
        }
        None
    }

    pub fn guess_error(
        &self,
        error_code: u64,
    ) -> Option<Arc<ToolboxIdlTransactionError>> {
        for error in self.errors.values() {
            if error_code != error.code {
                continue;
            }
            return Some(error.clone());
        }
        None
    }
}
