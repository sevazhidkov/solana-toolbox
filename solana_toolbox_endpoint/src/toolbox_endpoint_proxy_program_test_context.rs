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
use crate::toolbox_endpoint_proxy::ToolboxEndpointProxy;
use crate::toolbox_endpoint_simulation::ToolboxEndpointSimulation;

const SLOTS_PER_EPOCH: u64 = 432_000;
const SLOTS_PER_SECOND: u64 = 2;
const SECONDS_PER_EPOCH: u64 = SLOTS_PER_EPOCH / SLOTS_PER_SECOND;

#[async_trait::async_trait]
impl ToolboxEndpointProxy for ProgramTestContext {
    async fn get_latest_blockhash(
        &mut self
    ) -> Result<Hash, ToolboxEndpointError> {
        Ok(self.last_blockhash)
    }

    async fn get_balance(
        &mut self,
        address: &Pubkey,
    ) -> Result<u64, ToolboxEndpointError> {
        Ok(self.banks_client.get_balance(*address).await?)
    }

    async fn get_accounts(
        &mut self,
        addresses: &[Pubkey],
    ) -> Result<Vec<Option<Account>>, ToolboxEndpointError> {
        let mut accounts = vec![];
        for address in addresses {
            accounts.push(self.banks_client.get_account(*address).await?)
        }
        Ok(accounts)
    }

    async fn simulate_transaction(
        &mut self,
        transaction: &Transaction,
    ) -> Result<ToolboxEndpointSimulation, ToolboxEndpointError> {
        let simulation_outcome =
            self.banks_client.simulate_transaction(transaction.clone()).await?;
        let simulation_err = simulation_outcome.result.transpose().err();
        if let Some(simulation_details) = simulation_outcome.simulation_details
        {
            return Ok(ToolboxEndpointSimulation {
                err: simulation_err,
                logs: Some(simulation_details.logs),
                return_data: simulation_details
                    .return_data
                    .map(|return_data| return_data.data),
                units_consumed: Some(simulation_details.units_consumed),
            });
        }
        Ok(ToolboxEndpointSimulation {
            err: simulation_err,
            logs: None,
            return_data: None,
            units_consumed: None,
        })
    }

    async fn process_transaction(
        &mut self,
        transaction: &Transaction,
    ) -> Result<Signature, ToolboxEndpointError> {
        self.last_blockhash = self
            .banks_client
            .get_new_latest_blockhash(&self.last_blockhash)
            .await
            .map_err(ToolboxEndpointError::Io)?;
        self.banks_client.process_transaction(transaction.clone()).await?;
        Ok(Signature::default())
    }

    async fn process_airdrop(
        &mut self,
        to: &Pubkey,
        lamports: u64,
    ) -> Result<Signature, ToolboxEndpointError> {
        let instruction = transfer(&self.payer.pubkey(), to, lamports);
        let latest_blockhash = self.get_latest_blockhash().await?;
        let mut transaction = Transaction::new_with_payer(
            &[instruction.clone()],
            Some(&self.payer.pubkey()),
        );
        transaction.partial_sign(&[&self.payer], latest_blockhash);
        self.process_transaction(&transaction).await
    }

    async fn check_transaction(
        &mut self,
        signature: &Signature,
    ) -> Result<(), ToolboxEndpointError> {
        // TODO - rpc transaction saving
        return Ok(());
    }

    async fn forward_clock_unix_timestamp(
        &mut self,
        unix_timestamp_delta: u64,
    ) -> Result<(), ToolboxEndpointError> {
        let current_clock = self.banks_client.get_sysvar::<Clock>().await?;
        let mut forwarded_clock = current_clock;
        forwarded_clock.slot += unix_timestamp_delta * SLOTS_PER_SECOND;
        forwarded_clock.unix_timestamp +=
            i64::try_from(unix_timestamp_delta).unwrap();
        forwarded_clock.epoch += unix_timestamp_delta / SECONDS_PER_EPOCH;
        self.set_sysvar::<Clock>(&forwarded_clock);
        Ok(())
    }

    async fn forward_clock_slot(
        &mut self,
        slot_delta: u64,
    ) -> Result<(), ToolboxEndpointError> {
        let current_clock = self.banks_client.get_sysvar::<Clock>().await?;
        let mut forwarded_clock = current_clock;
        forwarded_clock.slot += slot_delta;
        forwarded_clock.unix_timestamp +=
            i64::try_from(slot_delta / SLOTS_PER_SECOND).unwrap();
        forwarded_clock.epoch += slot_delta / SLOTS_PER_EPOCH;
        self.set_sysvar::<Clock>(&forwarded_clock);
        Ok(())
    }

    async fn forward_clock_epoch(
        &mut self,
        epoch_delta: u64,
    ) -> Result<(), ToolboxEndpointError> {
        let current_clock = self.banks_client.get_sysvar::<Clock>().await?;
        let mut forwarded_clock = current_clock;
        forwarded_clock.slot += epoch_delta * SLOTS_PER_EPOCH;
        forwarded_clock.unix_timestamp +=
            i64::try_from(epoch_delta * SECONDS_PER_EPOCH).unwrap();
        forwarded_clock.epoch += epoch_delta;
        self.set_sysvar::<Clock>(&forwarded_clock);
        Ok(())
    }
}
