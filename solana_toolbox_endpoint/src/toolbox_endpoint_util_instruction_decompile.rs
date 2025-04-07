use std::collections::HashSet;

use anyhow::Result;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::instruction::CompiledInstruction;
use solana_sdk::instruction::Instruction;
use solana_sdk::message::CompileError;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_endpoint::ToolboxEndpoint;

impl ToolboxEndpoint {
    pub fn decompile_transaction_payer(
        static_addresses: &[Pubkey],
    ) -> Result<Pubkey> {
        Ok(*static_addresses
            .first()
            .ok_or(CompileError::AccountIndexOverflow)?)
    }

    pub fn decompile_transaction_instructions(
        header_num_required_signatures: u8,
        header_num_readonly_signed_accounts: u8,
        header_num_readonly_unsigned_accounts: u8,
        static_addresses: &[Pubkey],
        loaded_writable_addresses: &[Pubkey],
        loaded_readonly_addresses: &[Pubkey],
        compiled_instructions: &[CompiledInstruction],
    ) -> Result<Vec<Instruction>> {
        let signer_addresses =
            ToolboxEndpoint::decompile_transaction_signer_addresses(
                header_num_required_signatures,
                static_addresses,
            )?;
        let mut readonly_addresses =
            ToolboxEndpoint::decompile_transaction_static_readonly_addresses(
                header_num_required_signatures,
                header_num_readonly_signed_accounts,
                header_num_readonly_unsigned_accounts,
                static_addresses,
            )?;
        for loaded_readonly_address in loaded_readonly_addresses {
            readonly_addresses.insert(*loaded_readonly_address);
        }
        let mut used_addresses = vec![];
        used_addresses.extend_from_slice(static_addresses);
        used_addresses.extend_from_slice(loaded_writable_addresses);
        used_addresses.extend_from_slice(loaded_readonly_addresses);
        let mut instructions = vec![];
        for compiled_instruction in compiled_instructions {
            let instruction_program_id = *used_addresses
                .get(usize::from(compiled_instruction.program_id_index))
                .ok_or(CompileError::AccountIndexOverflow)?;
            let mut instruction_accounts = vec![];
            for account_index in &compiled_instruction.accounts {
                let account_address = used_addresses
                    .get(usize::from(*account_index))
                    .ok_or(CompileError::AccountIndexOverflow)?;
                let account_is_readonly =
                    readonly_addresses.contains(account_address);
                let account_is_signer =
                    signer_addresses.contains(account_address);
                instruction_accounts.push(if account_is_readonly {
                    AccountMeta::new_readonly(
                        *account_address,
                        account_is_signer,
                    )
                } else {
                    AccountMeta::new(*account_address, account_is_signer)
                });
            }
            instructions.push(Instruction {
                program_id: instruction_program_id,
                accounts: instruction_accounts,
                data: compiled_instruction.data.clone(),
            });
        }
        Ok(instructions)
    }

    fn decompile_transaction_signer_addresses(
        header_num_required_signatures: u8,
        static_addresses: &[Pubkey],
    ) -> Result<HashSet<Pubkey>> {
        let mut signers = HashSet::new();
        for index in 0..usize::from(header_num_required_signatures) {
            signers.insert(
                *static_addresses
                    .get(index)
                    .ok_or(CompileError::AccountIndexOverflow)?,
            );
        }
        Ok(signers)
    }

    fn decompile_transaction_static_readonly_addresses(
        header_num_required_signatures: u8,
        header_num_readonly_signed_accounts: u8,
        header_num_readonly_unsigned_accounts: u8,
        static_addresses: &[Pubkey],
    ) -> Result<HashSet<Pubkey>> {
        let mut readonly = HashSet::new();
        for index in (usize::from(header_num_required_signatures)
            - usize::from(header_num_readonly_signed_accounts))
            ..usize::from(header_num_required_signatures)
        {
            readonly.insert(
                *static_addresses
                    .get(index)
                    .ok_or(CompileError::AccountIndexOverflow)?,
            );
        }
        for index in (static_addresses.len()
            - usize::from(header_num_readonly_unsigned_accounts))
            ..static_addresses.len()
        {
            readonly.insert(
                *static_addresses
                    .get(index)
                    .ok_or(CompileError::AccountIndexOverflow)?,
            );
        }
        Ok(readonly)
    }
}
