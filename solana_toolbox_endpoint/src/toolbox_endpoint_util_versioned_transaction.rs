use std::collections::HashMap;
use std::collections::HashSet;

use solana_sdk::address_lookup_table::AddressLookupTableAccount;
use solana_sdk::hash::Hash;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::instruction::Instruction;
use solana_sdk::message::v0::Message;
use solana_sdk::message::CompileError;
use solana_sdk::message::VersionedMessage;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::VersionedTransaction;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_error::ToolboxEndpointError;

impl ToolboxEndpoint {
    pub fn compile_versioned_transaction(
        payer: &Keypair,
        instructions: &[Instruction],
        signers: &[&Keypair],
        resolved_address_lookup_tables: &[(Pubkey, Vec<Pubkey>)],
        recent_blockhash: Hash,
    ) -> Result<VersionedTransaction, ToolboxEndpointError> {
        let mut address_lookup_table_accounts = vec![];
        for resolved_address_lookup_table in resolved_address_lookup_tables {
            address_lookup_table_accounts.push(AddressLookupTableAccount {
                key: resolved_address_lookup_table.0,
                addresses: resolved_address_lookup_table.1.to_vec(),
            });
        }
        let mut keypairs = vec![];
        if !signers.contains(&payer) {
            keypairs.push(payer);
        }
        keypairs.extend_from_slice(signers);
        let versioned_transaction = VersionedTransaction::try_new(
            VersionedMessage::V0(Message::try_compile(
                &payer.pubkey(),
                &instructions,
                &address_lookup_table_accounts,
                recent_blockhash,
            )?),
            &keypairs,
        )?;
        Ok(versioned_transaction)
    }

    pub fn decompile_versioned_transaction(
        versioned_transaction: &VersionedTransaction,
        resolved_address_lookup_tables: &[(Pubkey, Vec<Pubkey>)],
    ) -> Result<(Pubkey, Vec<Instruction>), ToolboxEndpointError> {
        let mut known_address_lookup_tables_addresses = HashMap::new();
        for resolved_address_lookup_table in resolved_address_lookup_tables {
            known_address_lookup_tables_addresses.insert(
                resolved_address_lookup_table.0,
                &resolved_address_lookup_table.1[..],
            );
        }
        let mut loaded_addresses_writable = vec![];
        let mut loaded_addresses_readonly = vec![];
        if let Some(message_address_table_lookups) =
            versioned_transaction.message.address_table_lookups()
        {
            for message_address_table_lookup in message_address_table_lookups {
                let known_address_lookup_table_addresses =
                    known_address_lookup_tables_addresses
                        .get(&message_address_table_lookup.account_key)
                        .ok_or(CompileError::AddressTableLookupIndexOverflow)?;
                for loaded_addresses_writable_index in
                    &message_address_table_lookup.writable_indexes
                {
                    loaded_addresses_writable.push(
                        *known_address_lookup_table_addresses
                            .get(usize::from(*loaded_addresses_writable_index))
                            .ok_or(
                                CompileError::AddressTableLookupIndexOverflow,
                            )?,
                    );
                }
                for loaded_addresses_readonly_index in
                    &message_address_table_lookup.readonly_indexes
                {
                    loaded_addresses_readonly.push(
                        *known_address_lookup_table_addresses
                            .get(usize::from(*loaded_addresses_readonly_index))
                            .ok_or(
                                CompileError::AddressTableLookupIndexOverflow,
                            )?,
                    );
                }
            }
        }
        ToolboxEndpoint::decompile_versioned_transaction_with_loaded_addresses(
            versioned_transaction,
            &loaded_addresses_writable,
            &loaded_addresses_readonly,
        )
    }

    pub fn decompile_versioned_transaction_with_loaded_addresses(
        versioned_transaction: &VersionedTransaction,
        loaded_addresses_writable: &[Pubkey],
        loaded_addresses_readonly: &[Pubkey],
    ) -> Result<(Pubkey, Vec<Instruction>), ToolboxEndpointError> {
        let header = versioned_transaction.message.header();
        let static_signatures_count =
            usize::from(header.num_required_signatures);
        let static_readonly_signed_count =
            usize::from(header.num_readonly_signed_accounts);
        let static_readonly_unsigned_count =
            usize::from(header.num_readonly_unsigned_accounts);
        let static_accounts =
            versioned_transaction.message.static_account_keys();
        let static_accounts_count = static_accounts.len();
        let mut signers = HashSet::new();
        for static_account_index in 0..static_signatures_count {
            signers.insert(
                *static_accounts
                    .get(static_account_index)
                    .ok_or(CompileError::AccountIndexOverflow)?,
            );
        }
        let mut readonly = HashSet::new();
        for static_account_index in (static_signatures_count
            - static_readonly_signed_count)
            ..static_signatures_count
        {
            readonly.insert(
                *static_accounts
                    .get(static_account_index)
                    .ok_or(CompileError::AccountIndexOverflow)?,
            );
        }
        for static_account_index in (static_accounts_count
            - static_readonly_unsigned_count)
            ..static_accounts_count
        {
            readonly.insert(
                *static_accounts
                    .get(static_account_index)
                    .ok_or(CompileError::AccountIndexOverflow)?,
            );
        }
        for loaded_address_readonly in loaded_addresses_readonly {
            readonly.insert(*loaded_address_readonly);
        }
        let mut all_accounts = vec![];
        all_accounts.extend_from_slice(static_accounts);
        all_accounts.extend_from_slice(loaded_addresses_writable);
        all_accounts.extend_from_slice(loaded_addresses_readonly);
        let mut instructions = vec![];
        for instruction in versioned_transaction.message.instructions() {
            let instruction_program_id = *all_accounts
                .get(usize::from(instruction.program_id_index))
                .ok_or(CompileError::AccountIndexOverflow)?;
            let mut instruction_accounts = vec![];
            for account_index in &instruction.accounts {
                let account = all_accounts
                    .get(usize::from(*account_index))
                    .ok_or(CompileError::AccountIndexOverflow)?;
                let account_is_readonly = readonly.contains(&account);
                let account_is_signer = signers.contains(&account);
                instruction_accounts.push(if account_is_readonly {
                    AccountMeta::new_readonly(*account, account_is_signer)
                } else {
                    AccountMeta::new(*account, account_is_signer)
                });
            }
            instructions.push(Instruction {
                program_id: instruction_program_id,
                accounts: instruction_accounts,
                data: instruction.data.clone(),
            });
        }
        Ok((
            *all_accounts
                .first()
                .ok_or(CompileError::AccountIndexOverflow)?,
            instructions,
        ))
    }
}
