use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Duration;
use std::time::SystemTime;

use anyhow::anyhow;
use anyhow::Result;
use solana_program_test::ProgramTestBanksClientExt;
use solana_program_test::ProgramTestContext;
use solana_sdk::account::Account;
use solana_sdk::address_lookup_table::state::AddressLookupTable;
use solana_sdk::hash::Hash;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::signer::Signer;
use solana_sdk::slot_hashes::SlotHashes;
use solana_sdk::system_instruction::transfer;
use solana_sdk::sysvar::clock::Clock;
use solana_sdk::transaction::Transaction;
use solana_sdk::transaction::VersionedTransaction;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_execution::ToolboxEndpointExecution;
use crate::toolbox_endpoint_proxy::ToolboxEndpointProxy;

const SLOTS_PER_EPOCH: u64 = 432_000;
const SLOTS_PER_SECOND: u64 = 2;
const SECONDS_PER_EPOCH: u64 = SLOTS_PER_EPOCH / SLOTS_PER_SECOND;

pub struct ToolboxEndpointProxyProgramTestContext {
    program_test_context: ProgramTestContext,
    unix_timestamp_by_slot: HashMap<u64, i64>,
    addresses_by_program_id: HashMap<Pubkey, HashSet<Pubkey>>,
    signatures_by_address: HashMap<Pubkey, Vec<Signature>>,
    execution_by_signature: HashMap<Signature, ToolboxEndpointExecution>,
}

impl ToolboxEndpointProxyProgramTestContext {
    pub fn new(
        program_test_context: ProgramTestContext,
    ) -> ToolboxEndpointProxyProgramTestContext {
        ToolboxEndpointProxyProgramTestContext {
            program_test_context,
            unix_timestamp_by_slot: Default::default(),
            addresses_by_program_id: Default::default(),
            signatures_by_address: Default::default(),
            execution_by_signature: Default::default(),
        }
    }

    pub async fn save_slot_unix_timestamp(&mut self) {
        let clock = self
            .program_test_context
            .banks_client
            .get_sysvar::<Clock>()
            .await
            .unwrap();
        self.unix_timestamp_by_slot
            .insert(clock.slot, clock.unix_timestamp);
    }
}

#[async_trait::async_trait]
impl ToolboxEndpointProxy for ToolboxEndpointProxyProgramTestContext {
    async fn get_latest_blockhash(&mut self) -> Result<Hash> {
        Ok(self.program_test_context.last_blockhash)
    }

    async fn get_slot_unix_timestamp(&mut self, slot: u64) -> Result<i64> {
        self.unix_timestamp_by_slot
            .get(&slot)
            .ok_or_else(|| anyhow!("Could not find slot: {}", slot))
            .cloned()
    }

    async fn get_balance(&mut self, address: &Pubkey) -> Result<u64> {
        Ok(self
            .program_test_context
            .banks_client
            .get_balance(*address)
            .await?)
    }

    async fn get_account(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<Account>> {
        Ok(self
            .program_test_context
            .banks_client
            .get_account(*address)
            .await?)
    }

    async fn get_accounts(
        &mut self,
        addresses: &[Pubkey],
    ) -> Result<Vec<Option<Account>>> {
        let mut accounts = vec![];
        for address in addresses {
            accounts.push(
                self.program_test_context
                    .banks_client
                    .get_account(*address)
                    .await?,
            )
        }
        Ok(accounts)
    }

    async fn simulate_transaction(
        &mut self,
        versioned_transaction: VersionedTransaction,
        verify_signatures: bool,
    ) -> Result<ToolboxEndpointExecution> {
        ToolboxEndpoint::verify_versioned_transaction_length(
            &versioned_transaction,
        )?;
        if verify_signatures {
            ToolboxEndpoint::verify_versioned_transaction_signatures(
                &versioned_transaction,
            )?;
        }
        let clock = self
            .program_test_context
            .banks_client
            .get_sysvar::<Clock>()
            .await?;
        let outcome = self
            .program_test_context
            .banks_client
            .simulate_transaction(versioned_transaction.clone())
            .await?;
        let (payer, instructions) = self
            .resolve_versioned_transaction(&versioned_transaction)
            .await?;
        if let Some(simulation_details) = outcome.simulation_details {
            return Ok(ToolboxEndpointExecution {
                processed_time: None,
                slot: clock.slot,
                payer,
                instructions,
                error: outcome.result.transpose().err(),
                steps: Some(ToolboxEndpointExecution::try_parse_steps(
                    &simulation_details.logs,
                )?),
                logs: Some(simulation_details.logs),
                units_consumed: Some(simulation_details.units_consumed),
            });
        }
        Ok(ToolboxEndpointExecution {
            processed_time: None,
            slot: clock.slot,
            payer,
            instructions,
            error: outcome.result.transpose().err(),
            steps: None,
            logs: None,
            units_consumed: None,
        })
    }

    async fn process_transaction(
        &mut self,
        versioned_transaction: VersionedTransaction,
        process_preflight: bool,
    ) -> Result<(Signature, ToolboxEndpointExecution)> {
        ToolboxEndpoint::verify_versioned_transaction_length(
            &versioned_transaction,
        )?;
        ToolboxEndpoint::verify_versioned_transaction_signatures(
            &versioned_transaction,
        )?;
        if process_preflight {
            if let Some(Err(error)) = self
                .program_test_context
                .banks_client
                .simulate_transaction(versioned_transaction.clone())
                .await?
                .result
            {
                return Err(error.into());
            }
        }
        let outcome = self
            .program_test_context
            .banks_client
            .process_transaction_with_metadata(versioned_transaction.clone())
            .await?;
        let (payer, instructions) = self
            .resolve_versioned_transaction(&versioned_transaction)
            .await?;
        let mut transaction_accounts = HashSet::new();
        transaction_accounts.insert(payer);
        for instruction in &instructions {
            transaction_accounts.insert(instruction.program_id);
            for instruction_account_meta in &instruction.accounts {
                transaction_accounts.insert(instruction_account_meta.pubkey);
                self.insert_address_for_program_id(
                    instruction.program_id,
                    instruction_account_meta.pubkey,
                );
            }
        }
        let clock = self
            .program_test_context
            .banks_client
            .get_sysvar::<Clock>()
            .await?;
        let clock_time = SystemTime::UNIX_EPOCH
            + Duration::from_secs(clock.unix_timestamp as u64);
        let signature = Signature::new_unique();
        for transaction_account in transaction_accounts {
            self.push_signature_for_address(transaction_account, signature);
        }
        let execution = match outcome.metadata {
            Some(metadata) => ToolboxEndpointExecution {
                processed_time: Some(clock_time),
                slot: clock.slot,
                payer,
                instructions,
                error: outcome.result.err(),
                steps: Some(ToolboxEndpointExecution::try_parse_steps(
                    &metadata.log_messages,
                )?),
                logs: Some(metadata.log_messages),
                units_consumed: Some(metadata.compute_units_consumed),
            },
            None => ToolboxEndpointExecution {
                processed_time: Some(clock_time),
                slot: clock.slot,
                payer,
                instructions,
                error: outcome.result.err(),
                steps: None,
                logs: None,
                units_consumed: None,
            },
        };
        self.execution_by_signature
            .insert(signature, execution.clone());
        Ok((signature, execution))
    }

    async fn request_airdrop(
        &mut self,
        to: &Pubkey,
        lamports: u64,
    ) -> Result<(Signature, ToolboxEndpointExecution)> {
        let instruction =
            transfer(&self.program_test_context.payer.pubkey(), to, lamports);
        let latest_blockhash = self.get_latest_blockhash().await?;
        let mut transaction = Transaction::new_with_payer(
            &[instruction.clone()],
            Some(&self.program_test_context.payer.pubkey()),
        );
        transaction.partial_sign(
            &[&self.program_test_context.payer],
            latest_blockhash,
        );
        self.process_transaction(transaction.into(), false).await
    }

    async fn get_execution(
        &mut self,
        signature: &Signature,
    ) -> Result<ToolboxEndpointExecution> {
        self.execution_by_signature
            .get(signature)
            .ok_or_else(|| {
                anyhow!("Could not find execution signature: {}", signature)
            })
            .cloned()
    }

    async fn search_addresses(
        &mut self,
        program_id: &Pubkey,
        data_len: Option<usize>,
        data_chunks: &[(usize, &[u8])],
    ) -> Result<HashSet<Pubkey>> {
        let mut found_addresses = HashSet::new();
        if let Some(addresses) = self.addresses_by_program_id.get(program_id) {
            for address in addresses {
                let account = self
                    .program_test_context
                    .banks_client
                    .get_account(*address)
                    .await?
                    .unwrap_or_default();
                if account.owner != *program_id {
                    continue;
                }
                if let Some(data_len) = data_len {
                    if account.data.len() != data_len {
                        continue;
                    }
                }
                let mut data_match = true;
                for (data_offset, data_slices) in data_chunks {
                    if account.data.len() < *data_offset {
                        data_match = false;
                        continue;
                    }
                    if !account.data[*data_offset..].starts_with(data_slices) {
                        data_match = false;
                        continue;
                    }
                }
                if data_match {
                    found_addresses.insert(*address);
                }
            }
        }
        Ok(found_addresses)
    }

    async fn search_signatures(
        &mut self,
        address: &Pubkey,
        limit: usize,
        start_before: Option<Signature>,
        rewind_until: Option<Signature>,
    ) -> Result<Vec<Signature>> {
        let mut found_signatures = vec![];
        if let Some(signatures) = self.signatures_by_address.get(address) {
            let mut started = start_before.is_none();
            for signature in signatures.iter().rev() {
                if started {
                    found_signatures.push(*signature);
                    if let Some(rewind_until) = rewind_until {
                        if *signature == rewind_until {
                            break;
                        }
                    }
                    if found_signatures.len() >= limit {
                        break;
                    }
                }
                if let Some(start_before) = start_before {
                    if *signature == start_before {
                        started = true;
                    }
                }
            }
        }
        Ok(found_signatures)
    }

    async fn forward_clock_unix_timestamp(
        &mut self,
        unix_timestamp_delta: u64,
    ) -> Result<()> {
        if unix_timestamp_delta == 0 {
            return Ok(());
        }
        let current_clock = self
            .program_test_context
            .banks_client
            .get_sysvar::<Clock>()
            .await?;
        let mut forwarded_clock = current_clock;
        forwarded_clock.slot += unix_timestamp_delta * SLOTS_PER_SECOND;
        forwarded_clock.unix_timestamp += i64::try_from(unix_timestamp_delta)?;
        forwarded_clock.epoch += unix_timestamp_delta / SECONDS_PER_EPOCH;
        self.update_slot(&forwarded_clock).await
    }

    async fn forward_clock_slot(&mut self, slot_delta: u64) -> Result<()> {
        if slot_delta == 0 {
            return Ok(());
        }
        let current_clock = self
            .program_test_context
            .banks_client
            .get_sysvar::<Clock>()
            .await?;
        let mut forwarded_clock = current_clock;
        forwarded_clock.slot += slot_delta;
        forwarded_clock.unix_timestamp +=
            i64::try_from(slot_delta / SLOTS_PER_SECOND)?;
        forwarded_clock.epoch += slot_delta / SLOTS_PER_EPOCH;
        self.update_slot(&forwarded_clock).await
    }

    async fn forward_clock_epoch(&mut self, epoch_delta: u64) -> Result<()> {
        if epoch_delta == 0 {
            return Ok(());
        }
        let current_clock = self
            .program_test_context
            .banks_client
            .get_sysvar::<Clock>()
            .await?;
        let mut forwarded_clock = current_clock;
        forwarded_clock.slot += epoch_delta * SLOTS_PER_EPOCH;
        forwarded_clock.unix_timestamp +=
            i64::try_from(epoch_delta * SECONDS_PER_EPOCH)?;
        forwarded_clock.epoch += epoch_delta;
        self.update_slot(&forwarded_clock).await
    }
}

impl ToolboxEndpointProxyProgramTestContext {
    async fn update_slot(&mut self, new_clock: &Clock) -> Result<()> {
        let old_hash = self.program_test_context.last_blockhash;
        let old_clock = self
            .program_test_context
            .banks_client
            .get_sysvar::<Clock>()
            .await?;
        let new_hash = self
            .program_test_context
            .banks_client
            .get_new_latest_blockhash(&old_hash)
            .await?;
        let mut slot_hashes = self
            .program_test_context
            .banks_client
            .get_sysvar::<SlotHashes>()
            .await?;
        slot_hashes.add(old_clock.slot, old_hash);
        self.program_test_context.set_sysvar(&slot_hashes);
        self.program_test_context.set_sysvar(new_clock);
        self.program_test_context.last_blockhash = new_hash;
        self.save_slot_unix_timestamp().await;
        Ok(())
    }

    fn push_signature_for_address(
        &mut self,
        address: Pubkey,
        signature: Signature,
    ) {
        match self.signatures_by_address.entry(address) {
            Entry::Vacant(entry) => {
                entry.insert(vec![signature]);
            },
            Entry::Occupied(mut entry) => {
                entry.get_mut().push(signature);
            },
        };
    }

    fn insert_address_for_program_id(
        &mut self,
        program_id: Pubkey,
        address: Pubkey,
    ) {
        match self.addresses_by_program_id.entry(program_id) {
            Entry::Vacant(entry) => {
                entry.insert(HashSet::from_iter([address]));
            },
            Entry::Occupied(mut entry) => {
                entry.get_mut().insert(address);
            },
        };
    }
}

impl ToolboxEndpointProxyProgramTestContext {
    pub async fn get_address_lookup_table_addresses(
        &mut self,
        address_lookup_table: &Pubkey,
    ) -> Result<Option<Vec<Pubkey>>> {
        match self.get_account(address_lookup_table).await? {
            Some(account) => Ok(Some(
                AddressLookupTable::deserialize(&account.data)?
                    .addresses
                    .to_vec(),
            )),
            _ => Ok(None),
        }
    }

    pub async fn resolve_versioned_transaction(
        &mut self,
        versioned_transaction: &VersionedTransaction,
    ) -> Result<(Pubkey, Vec<Instruction>)> {
        let mut resolved_address_lookup_tables = vec![];
        if let Some(message_address_table_lookups) =
            versioned_transaction.message.address_table_lookups()
        {
            for message_address_table_lookup in message_address_table_lookups {
                let message_address_lookup_table =
                    message_address_table_lookup.account_key;
                if let Some(address_lookup_table_addresses) = self
                    .get_address_lookup_table_addresses(
                        &message_address_lookup_table,
                    )
                    .await?
                {
                    resolved_address_lookup_tables.push((
                        message_address_lookup_table,
                        address_lookup_table_addresses,
                    ));
                }
            }
        }
        ToolboxEndpoint::decompile_versioned_transaction(
            versioned_transaction,
            &resolved_address_lookup_tables,
        )
    }
}
