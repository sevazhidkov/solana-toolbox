use std::collections::HashMap;

use solana_sdk::address_lookup_table::AddressLookupTableAccount;
use solana_sdk::hash::Hash;
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
                instructions,
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
        let mut resolved_address_lookup_tables_addresses = HashMap::new();
        for resolved_address_lookup_table in resolved_address_lookup_tables {
            resolved_address_lookup_tables_addresses.insert(
                resolved_address_lookup_table.0,
                &resolved_address_lookup_table.1[..],
            );
        }
        let mut loaded_writable_addresses = vec![];
        let mut loaded_readonly_addresses = vec![];
        if let Some(message_address_table_lookups) =
            versioned_transaction.message.address_table_lookups()
        {
            for message_address_table_lookup in message_address_table_lookups {
                let resolved_address_lookup_table_addresses =
                    resolved_address_lookup_tables_addresses
                        .get(&message_address_table_lookup.account_key)
                        .ok_or(CompileError::AddressTableLookupIndexOverflow)?;
                for message_address_table_lookup_writable_index in
                    &message_address_table_lookup.writable_indexes
                {
                    loaded_writable_addresses.push(
                        *resolved_address_lookup_table_addresses
                            .get(usize::from(
                                *message_address_table_lookup_writable_index,
                            ))
                            .ok_or(
                                CompileError::AddressTableLookupIndexOverflow,
                            )?,
                    );
                }
                for message_address_table_lookup_readonly_index in
                    &message_address_table_lookup.readonly_indexes
                {
                    loaded_readonly_addresses.push(
                        *resolved_address_lookup_table_addresses
                            .get(usize::from(
                                *message_address_table_lookup_readonly_index,
                            ))
                            .ok_or(
                                CompileError::AddressTableLookupIndexOverflow,
                            )?,
                    );
                }
            }
        }
        ToolboxEndpoint::decompile_versioned_transaction_with_loaded_addresses(
            versioned_transaction,
            &loaded_writable_addresses,
            &loaded_readonly_addresses,
        )
    }

    pub fn decompile_versioned_transaction_with_loaded_addresses(
        versioned_transaction: &VersionedTransaction,
        loaded_writable_addresses: &[Pubkey],
        loaded_readonly_addresses: &[Pubkey],
    ) -> Result<(Pubkey, Vec<Instruction>), ToolboxEndpointError> {
        let header = versioned_transaction.message.header();
        let static_addresses =
            versioned_transaction.message.static_account_keys();
        let payer =
            ToolboxEndpoint::decompile_transaction_payer(static_addresses)?;
        let instructions = ToolboxEndpoint::decompile_transaction_instructions(
            header.num_required_signatures,
            header.num_readonly_signed_accounts,
            header.num_readonly_unsigned_accounts,
            static_addresses,
            loaded_writable_addresses,
            loaded_readonly_addresses,
            versioned_transaction.message.instructions(),
        )?;
        Ok((payer, instructions))
    }
}
