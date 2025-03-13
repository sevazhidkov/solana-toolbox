use std::collections::HashMap;

use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_account::ToolboxIdlProgramAccount;
use crate::toolbox_idl_program_error::ToolboxIdlProgramError;
use crate::toolbox_idl_program_instruction::ToolboxIdlProgramInstruction;
use crate::toolbox_idl_program_typedef::ToolboxIdlProgramTypedef;

// TODO - i don't like this "root" postfix
#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgramRoot {
    pub typedefs: HashMap<String, ToolboxIdlProgramTypedef>,
    pub instructions: HashMap<String, ToolboxIdlProgramInstruction>,
    pub accounts: HashMap<String, ToolboxIdlProgramAccount>,
    pub errors: HashMap<String, ToolboxIdlProgramError>,
}

impl ToolboxIdlProgramRoot {
    pub const DISCRIMINATOR: &[u8] =
        &[0x18, 0x46, 0x62, 0xBF, 0x3A, 0x90, 0x7B, 0x9E];

    // TODO - provide standard implementation for basic contracts such as spl_token and system, and compute_budget ?
    pub async fn get_for_program_id(
        endpoint: &mut ToolboxEndpoint,
        program_id: &Pubkey,
    ) -> Result<Option<ToolboxIdlProgramRoot>, ToolboxIdlError> {
        endpoint
            .get_account(&ToolboxIdlProgramRoot::find(program_id)?)
            .await?
            .map(|account| ToolboxIdlProgramRoot::try_from_account(&account))
            .transpose()
    }

    pub fn find(program_id: &Pubkey) -> Result<Pubkey, ToolboxIdlError> {
        let base = Pubkey::find_program_address(&[], program_id).0;
        Pubkey::create_with_seed(&base, "anchor:idl", program_id)
            .map_err(ToolboxIdlError::Pubkey)
    }

    pub fn guess_program_instruction(
        &self,
        instruction_data: &[u8],
    ) -> Option<&ToolboxIdlProgramInstruction> {
        for program_instruction in self.instructions.values() {
            if instruction_data.starts_with(&program_instruction.discriminator)
            {
                return Some(program_instruction);
            }
        }
        None
    }

    pub fn guess_program_account(
        &self,
        account_data: &[u8],
    ) -> Option<&ToolboxIdlProgramAccount> {
        for program_account in self.accounts.values() {
            if account_data.starts_with(&program_account.discriminator) {
                return Some(program_account);
            }
        }
        None
    }
}
