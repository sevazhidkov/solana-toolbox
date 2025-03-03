use std::collections::HashSet;

use solana_sdk::hash::Hash;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::instruction::Instruction;
use solana_sdk::message::CompileError;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_error::ToolboxEndpointError;

impl ToolboxEndpoint {
    pub async fn compile_transaction(
        payer: &Keypair,
        instructions: &[Instruction],
        signers: &[&Keypair],
        recent_blockhash: Hash,
    ) -> Result<Transaction, ToolboxEndpointError> {
        let mut transaction =
            Transaction::new_with_payer(&instructions, Some(&payer.pubkey()));
        let mut keypairs = signers.to_vec();
        keypairs.push(payer);
        transaction.partial_sign(&keypairs, recent_blockhash);
        Ok(transaction)
    }

    pub fn decompile_transaction(
        transaction: &Transaction
    ) -> Result<(Pubkey, Vec<Instruction>), ToolboxEndpointError> {
        let header = transaction.message.header;
        let signatures_count = usize::from(header.num_required_signatures);
        let readonly_signed_count =
            usize::from(header.num_readonly_signed_accounts);
        let readonly_unsigned_count =
            usize::from(header.num_readonly_unsigned_accounts);
        let accounts = &transaction.message.account_keys;
        let accounts_count = accounts.len();
        let mut signers = HashSet::new();
        for account_index in 0..signatures_count {
            signers.insert(
                *accounts
                    .get(account_index)
                    .ok_or(CompileError::AccountIndexOverflow)?,
            );
        }
        let mut readonly = HashSet::new();
        for account_index in
            (signatures_count - readonly_signed_count)..signatures_count
        {
            readonly.insert(
                *accounts
                    .get(account_index)
                    .ok_or(CompileError::AccountIndexOverflow)?,
            );
        }
        for account_index in
            (accounts_count - readonly_unsigned_count)..accounts_count
        {
            readonly.insert(
                *accounts
                    .get(account_index)
                    .ok_or(CompileError::AccountIndexOverflow)?,
            );
        }
        let mut instructions = vec![];
        for instruction in &transaction.message.instructions {
            let instruction_program_id = *accounts
                .get(usize::from(instruction.program_id_index))
                .ok_or(CompileError::AccountIndexOverflow)?;
            let mut instruction_accounts = vec![];
            for account_index in &instruction.accounts {
                let account = accounts
                    .get(usize::from(*account_index))
                    .ok_or(CompileError::AccountIndexOverflow)?;
                let account_is_readonly = readonly.contains(&account);
                let account_is_signer = signers.contains(&account);
                instruction_accounts.push(
                    if account_is_readonly {
                        AccountMeta::new_readonly(*account, account_is_signer)
                    } else {
                        AccountMeta::new(*account, account_is_signer)
                    },
                );
            }
            instructions.push(Instruction {
                program_id: instruction_program_id,
                accounts: instruction_accounts,
                data: instruction.data.clone(),
            });
        }
        Ok((
            *accounts.first().ok_or(CompileError::AccountIndexOverflow)?,
            instructions,
        ))
    }
}
