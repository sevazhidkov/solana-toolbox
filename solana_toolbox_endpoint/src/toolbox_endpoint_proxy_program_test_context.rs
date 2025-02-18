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

use crate::toolbox_endpoint_error::ToolboxEndpointError;
use crate::toolbox_endpoint_execution::ToolboxEndpointExecution;
use crate::toolbox_endpoint_proxy::ToolboxEndpointProxy;

const SLOTS_PER_EPOCH: u64 = 432_000;
const SLOTS_PER_SECOND: u64 = 2;
const SECONDS_PER_EPOCH: u64 = SLOTS_PER_EPOCH / SLOTS_PER_SECOND;

pub struct ToolboxEndpointProxyProgramTestContext {
    inner: ProgramTestContext,
    processed: Vec<(Signature, ToolboxEndpointExecution)>,
}

impl ToolboxEndpointProxyProgramTestContext {
    pub fn new(
        program_test_context: ProgramTestContext
    ) -> ToolboxEndpointProxyProgramTestContext {
        ToolboxEndpointProxyProgramTestContext {
            inner: program_test_context,
            processed: Default::default(),
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
    ) -> Result<ToolboxEndpointExecution, ToolboxEndpointError> {
        let current_slot =
            self.inner.banks_client.get_sysvar::<Clock>().await?.slot;
        let outcome = self
            .inner
            .banks_client
            .simulate_transaction(transaction.clone())
            .await?;
        if let Some(simulation_details) = outcome.simulation_details {
            return Ok(ToolboxEndpointExecution {
                slot: current_slot,
                error: outcome.result.transpose().err(),
                logs: Some(simulation_details.logs),
                return_data: simulation_details
                    .return_data
                    .map(|return_data| return_data.data),
                units_consumed: Some(simulation_details.units_consumed),
            });
        }
        Ok(ToolboxEndpointExecution {
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
            Some(metadata) => ToolboxEndpointExecution {
                slot: current_slot,
                error: outcome.result.err(),
                logs: Some(metadata.log_messages),
                return_data: metadata
                    .return_data
                    .map(|return_data| return_data.data),
                units_consumed: Some(metadata.compute_units_consumed),
            },
            None => ToolboxEndpointExecution {
                slot: current_slot,
                error: outcome.result.err(),
                logs: None,
                return_data: None,
                units_consumed: None,
            },
        };
        self.processed.push((signature, execution));
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
        searched_signature: &Signature,
    ) -> Result<ToolboxEndpointExecution, ToolboxEndpointError> {
        for (signature, execution) in &self.processed {
            if searched_signature == signature {
                return Ok(execution.clone());
            }
        }
        Err(ToolboxEndpointError::Custom(
            "Unknown execution signature".to_string(),
        ))
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
