use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;

use solana_program_test::ProgramTestBanksClientExt;
use solana_program_test::ProgramTestContext;
use solana_sdk::account::Account;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction::transfer;
use solana_sdk::sysvar::clock::Clock;
use solana_sdk::transaction::Transaction;

use crate::toolbox_endpoint_data_execution::ToolboxEndpointDataExecution;
use crate::toolbox_endpoint_error::ToolboxEndpointError;
use crate::toolbox_endpoint_proxy::ToolboxEndpointProxy;

const SLOTS_PER_EPOCH: u64 = 432_000;
const SLOTS_PER_SECOND: u64 = 2;
const SECONDS_PER_EPOCH: u64 = SLOTS_PER_EPOCH / SLOTS_PER_SECOND;

pub struct ToolboxEndpointProxyProgramTestContext {
    inner: ProgramTestContext,
    addresses_by_program_id: HashMap<Pubkey, HashSet<Pubkey>>,
    signatures_by_address: HashMap<Pubkey, Vec<Signature>>,
    execution_by_signature: HashMap<Signature, ToolboxEndpointDataExecution>,
}

impl ToolboxEndpointProxyProgramTestContext {
    pub fn new(
        program_test_context: ProgramTestContext
    ) -> ToolboxEndpointProxyProgramTestContext {
        ToolboxEndpointProxyProgramTestContext {
            inner: program_test_context,
            addresses_by_program_id: Default::default(),
            signatures_by_address: Default::default(),
            execution_by_signature: Default::default(),
        }
    }
}

#[async_trait::async_trait]
impl ToolboxEndpointProxy for ToolboxEndpointProxyProgramTestContext {
    async fn get_latest_blockhash(
        &mut self
    ) -> Result<Hash, ToolboxEndpointError> {
        Ok(self.inner.last_blockhash)
    }

    async fn get_balance(
        &mut self,
        address: &Pubkey,
    ) -> Result<u64, ToolboxEndpointError> {
        Ok(self.inner.banks_client.get_balance(*address).await?)
    }

    async fn get_accounts(
        &mut self,
        addresses: &[Pubkey],
    ) -> Result<Vec<Option<Account>>, ToolboxEndpointError> {
        let mut accounts = vec![];
        for address in addresses {
            accounts.push(self.inner.banks_client.get_account(*address).await?)
        }
        Ok(accounts)
    }

    async fn simulate_transaction(
        &mut self,
        transaction: &Transaction,
    ) -> Result<ToolboxEndpointDataExecution, ToolboxEndpointError> {
        let current_slot =
            self.inner.banks_client.get_sysvar::<Clock>().await?.slot;
        let outcome = self
            .inner
            .banks_client
            .simulate_transaction(transaction.clone())
            .await?;
        if let Some(simulation_details) = outcome.simulation_details {
            return Ok(ToolboxEndpointDataExecution {
                slot: current_slot,
                error: outcome.result.transpose().err(),
                logs: Some(simulation_details.logs),
                return_data: simulation_details
                    .return_data
                    .map(|return_data| return_data.data),
                units_consumed: Some(simulation_details.units_consumed),
            });
        }
        Ok(ToolboxEndpointDataExecution {
            slot: current_slot,
            error: outcome.result.transpose().err(),
            logs: None,
            return_data: None,
            units_consumed: None,
        })
    }

    async fn process_transaction(
        &mut self,
        transaction: &Transaction,
    ) -> Result<Signature, ToolboxEndpointError> {
        let current_slot =
            self.inner.banks_client.get_sysvar::<Clock>().await?.slot;
        let signature = Signature::new_unique();
        let outcome = self
            .inner
            .banks_client
            .process_transaction_with_metadata(transaction.clone())
            .await?;
        let execution = match outcome.metadata {
            Some(metadata) => ToolboxEndpointDataExecution {
                slot: current_slot,
                error: outcome.result.err(),
                logs: Some(metadata.log_messages),
                return_data: metadata
                    .return_data
                    .map(|return_data| return_data.data),
                units_consumed: Some(metadata.compute_units_consumed),
            },
            None => ToolboxEndpointDataExecution {
                slot: current_slot,
                error: outcome.result.err(),
                logs: None,
                return_data: None,
                units_consumed: None,
            },
        };
        for instruction in &transaction.message.instructions {
            let instruction_program_id = transaction.message.account_keys
                [usize::from(instruction.program_id_index)];
            for instruction_account_index in &instruction.accounts {
                let instruction_account = transaction.message.account_keys
                    [usize::from(*instruction_account_index)];
                match self.addresses_by_program_id.entry(instruction_program_id)
                {
                    Entry::Vacant(entry) => {
                        entry.insert(HashSet::from_iter([instruction_account]));
                    },
                    Entry::Occupied(mut entry) => {
                        entry.get_mut().insert(instruction_account);
                    },
                };
            }
        }
        for account_key in &transaction.message.account_keys {
            match self.signatures_by_address.entry(*account_key) {
                Entry::Vacant(entry) => {
                    entry.insert(vec![signature]);
                },
                Entry::Occupied(mut entry) => {
                    entry.get_mut().push(signature);
                },
            };
        }
        self.execution_by_signature.insert(signature, execution);
        Ok(signature)
    }

    async fn request_airdrop(
        &mut self,
        to: &Pubkey,
        lamports: u64,
    ) -> Result<Signature, ToolboxEndpointError> {
        let instruction = transfer(&self.inner.payer.pubkey(), to, lamports);
        let latest_blockhash = self.get_latest_blockhash().await?;
        let mut transaction = Transaction::new_with_payer(
            &[instruction.clone()],
            Some(&self.inner.payer.pubkey()),
        );
        transaction.partial_sign(&[&self.inner.payer], latest_blockhash);
        self.process_transaction(&transaction).await
    }

    async fn get_execution(
        &mut self,
        signature: &Signature,
    ) -> Result<ToolboxEndpointDataExecution, ToolboxEndpointError> {
        self.execution_by_signature
            .get(&signature)
            .ok_or_else(|| {
                ToolboxEndpointError::Custom(
                    "Unknown execution signature".to_string(),
                )
            })
            .cloned()
    }

    async fn search_addresses(
        &mut self,
        program_id: &Pubkey,
        data_len: Option<usize>,
        data_chunks: &[(usize, &[u8])],
    ) -> Result<HashSet<Pubkey>, ToolboxEndpointError> {
        let mut found_addresses = HashSet::new();
        if let Some(addresses) = self.addresses_by_program_id.get(&program_id) {
            for address in addresses {
                let account = self
                    .inner
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
        start_before: Option<Signature>,
        rewind_until: Option<Signature>,
        limit: usize,
    ) -> Result<Vec<Signature>, ToolboxEndpointError> {
        let mut found_signatures = vec![];
        if let Some(signatures) = self.signatures_by_address.get(address) {
            let mut started = start_before.is_none();
            for signature in signatures.iter().rev() {
                if started {
                    found_signatures.push(*signature);
                }
                if let Some(start_before) = start_before {
                    if *signature == start_before {
                        started = true;
                    }
                }
                if let Some(rewind_until) = rewind_until {
                    if *signature == rewind_until {
                        break;
                    }
                }
                if found_signatures.len() >= limit {
                    break;
                }
            }
        }
        Ok(found_signatures)
    }

    async fn forward_clock_unix_timestamp(
        &mut self,
        unix_timestamp_delta: u64,
    ) -> Result<(), ToolboxEndpointError> {
        if unix_timestamp_delta <= 0 {
            return Ok(());
        }
        let current_clock =
            self.inner.banks_client.get_sysvar::<Clock>().await?;
        let mut forwarded_clock = current_clock;
        forwarded_clock.slot += unix_timestamp_delta * SLOTS_PER_SECOND;
        forwarded_clock.unix_timestamp +=
            i64::try_from(unix_timestamp_delta).unwrap();
        forwarded_clock.epoch += unix_timestamp_delta / SECONDS_PER_EPOCH;
        self.update_clock(&forwarded_clock).await
    }

    async fn forward_clock_slot(
        &mut self,
        slot_delta: u64,
    ) -> Result<(), ToolboxEndpointError> {
        if slot_delta <= 0 {
            return Ok(());
        }
        let current_clock =
            self.inner.banks_client.get_sysvar::<Clock>().await?;
        let mut forwarded_clock = current_clock;
        forwarded_clock.slot += slot_delta;
        forwarded_clock.unix_timestamp +=
            i64::try_from(slot_delta / SLOTS_PER_SECOND).unwrap();
        forwarded_clock.epoch += slot_delta / SLOTS_PER_EPOCH;
        self.update_clock(&forwarded_clock).await
    }

    async fn forward_clock_epoch(
        &mut self,
        epoch_delta: u64,
    ) -> Result<(), ToolboxEndpointError> {
        if epoch_delta <= 0 {
            return Ok(());
        }
        let current_clock =
            self.inner.banks_client.get_sysvar::<Clock>().await?;
        let mut forwarded_clock = current_clock;
        forwarded_clock.slot += epoch_delta * SLOTS_PER_EPOCH;
        forwarded_clock.unix_timestamp +=
            i64::try_from(epoch_delta * SECONDS_PER_EPOCH).unwrap();
        forwarded_clock.epoch += epoch_delta;
        self.update_clock(&forwarded_clock).await
    }
}

impl ToolboxEndpointProxyProgramTestContext {
    async fn update_clock(
        &mut self,
        clock: &Clock,
    ) -> Result<(), ToolboxEndpointError> {
        self.inner.set_sysvar::<Clock>(clock);
        self.update_blockhash().await
    }

    async fn update_blockhash(&mut self) -> Result<(), ToolboxEndpointError> {
        self.inner.last_blockhash = self
            .inner
            .banks_client
            .get_new_latest_blockhash(&self.inner.last_blockhash)
            .await
            .map_err(ToolboxEndpointError::Io)?;
        Ok(())
    }
}
