use std::collections::HashMap;

use inflate::inflate_bytes_zlib;
use solana_sdk::account::Account;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_account::ToolboxIdlProgramAccount;
use crate::toolbox_idl_program_error::ToolboxIdlProgramError;
use crate::toolbox_idl_program_instruction::ToolboxIdlProgramInstruction;
use crate::toolbox_idl_program_typedef::ToolboxIdlProgramTypedef;
use crate::toolbox_idl_utils::idl_map_err_invalid_integer;
use crate::toolbox_idl_utils::idl_pubkey_from_bytes_at;
use crate::toolbox_idl_utils::idl_slice_from_bytes;
use crate::toolbox_idl_utils::idl_u32_from_bytes_at;

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
            .get_account(&ToolboxIdlProgramRoot::find_for_program_id(
                program_id,
            )?)
            .await?
            .map(|account| ToolboxIdlProgramRoot::try_from_account(&account))
            .transpose()
    }

    pub fn find_for_program_id(
        program_id: &Pubkey,
    ) -> Result<Pubkey, ToolboxIdlError> {
        let base = Pubkey::find_program_address(&[], program_id).0;
        Pubkey::create_with_seed(&base, "anchor:idl", program_id)
            .map_err(ToolboxIdlError::Pubkey)
    }

    pub fn try_from_account(
        account: &Account,
    ) -> Result<ToolboxIdlProgramRoot, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let discriminator = ToolboxIdlProgramRoot::DISCRIMINATOR;
        if !account.data.starts_with(discriminator) {
            return Err(ToolboxIdlError::InvalidDiscriminator {
                expected: discriminator.to_vec(),
                found: account.data.to_vec(),
            });
        }
        let authority_offset = discriminator.len();
        let authority = idl_pubkey_from_bytes_at(
            &account.data,
            authority_offset,
            &breadcrumbs.as_val("authority"),
        )?;
        let length_offset =
            authority_offset + std::mem::size_of_val(&authority);
        let length = idl_u32_from_bytes_at(
            &account.data,
            length_offset,
            &breadcrumbs.as_val("length"),
        )?;
        let content_offset = length_offset + std::mem::size_of_val(&length);
        let content = idl_slice_from_bytes(
            &account.data,
            content_offset,
            idl_map_err_invalid_integer(
                usize::try_from(length),
                &breadcrumbs.as_val("length"),
            )?,
            &breadcrumbs.as_val("content"),
        )?;
        let content_encoded =
            inflate_bytes_zlib(content).map_err(ToolboxIdlError::Inflate)?;
        let content_decoded =
            String::from_utf8(content_encoded).map_err(|err| {
                ToolboxIdlError::InvalidString {
                    parsing: err,
                    context: breadcrumbs.as_val("content"),
                }
            })?;
        ToolboxIdlProgramRoot::try_parse_from_str(&content_decoded)
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
