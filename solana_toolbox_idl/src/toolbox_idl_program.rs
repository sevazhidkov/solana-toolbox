use std::collections::HashMap;

use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_account::ToolboxIdlAccount;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_instruction::ToolboxIdlInstruction;
use crate::toolbox_idl_transaction_error::ToolboxIdlTransactionError;
use crate::toolbox_idl_typedef::ToolboxIdlTypedef;

// TODO - i don't like this "root" postfix
#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgram {
    pub typedefs: HashMap<String, ToolboxIdlTypedef>,
    pub instructions: HashMap<String, ToolboxIdlInstruction>,
    pub accounts: HashMap<String, ToolboxIdlAccount>,
    pub errors: HashMap<String, ToolboxIdlTransactionError>,
}

impl ToolboxIdlProgram {
    pub const DISCRIMINATOR: &[u8] =
        &[0x18, 0x46, 0x62, 0xBF, 0x3A, 0x90, 0x7B, 0x9E];

    pub fn find(program_id: &Pubkey) -> Result<Pubkey, ToolboxIdlError> {
        let base = Pubkey::find_program_address(&[], program_id).0;
        Pubkey::create_with_seed(&base, "anchor:idl", program_id)
            .map_err(ToolboxIdlError::Pubkey)
    }

    pub fn guess_instruction(
        &self,
        instruction_data: &[u8],
    ) -> Result<&ToolboxIdlInstruction, ToolboxIdlError> {
        for instruction in self.instructions.values() {
            if instruction_data.starts_with(&instruction.discriminator) {
                return Ok(instruction);
            }
        }
        Err(ToolboxIdlError::CouldNotGuessAccount {})
    }

    pub fn guess_account(
        &self,
        account_data: &[u8],
    ) -> Result<&ToolboxIdlAccount, ToolboxIdlError> {
        for account in self.accounts.values() {
            if account_data.starts_with(&account.discriminator) {
                return Ok(account);
            }
        }
        Err(ToolboxIdlError::CouldNotGuessAccount {})
    }
}
