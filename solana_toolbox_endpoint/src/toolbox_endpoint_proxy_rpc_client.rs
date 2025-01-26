use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::account::Account;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::sysvar::clock;
use solana_sdk::sysvar::clock::Clock;
use solana_sdk::transaction::Transaction;

use crate::toolbox_endpoint_error::ToolboxEndpointError;
use crate::toolbox_endpoint_proxy::ToolboxEndpointProxy;

const WAIT_SLEEP_DURATION: Duration = Duration::from_millis(100);
const WAIT_TIMEOUT_DURATION: Duration = Duration::from_secs(10);

async fn rpc_get_sysvar_clock(
    rpc_client: &mut RpcClient
) -> Result<Clock, ToolboxEndpointError> {
    bincode::deserialize::<Clock>(
        &rpc_client.get_account(&clock::ID).await?.data,
    )
    .map_err(ToolboxEndpointError::Bincode)
}

#[async_trait::async_trait]
impl ToolboxEndpointProxy for RpcClient {
    async fn get_latest_blockhash(
        &mut self
    ) -> Result<Hash, ToolboxEndpointError> {
        Ok(RpcClient::get_latest_blockhash(self).await?)
    }

    async fn get_balance(
        &mut self,
        address: &Pubkey,
    ) -> Result<u64, ToolboxEndpointError> {
        Ok(self.get_balance(address).await?)
    }

    async fn get_accounts(
        &mut self,
        addresses: &[Pubkey],
    ) -> Result<Vec<Option<Account>>, ToolboxEndpointError> {
        Ok(self.get_multiple_accounts(addresses).await?)
    }

    async fn process_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<Signature, ToolboxEndpointError> {
        let timer = Instant::now();
        let signature = self.send_transaction(&transaction).await?;
        loop {
            if self.confirm_transaction(&signature).await? {
                return Ok(signature);
            }
            if timer.elapsed() > WAIT_TIMEOUT_DURATION {
                return Err(ToolboxEndpointError::Timeout(
                    "Waiting confirmation",
                ));
            }
            sleep(WAIT_SLEEP_DURATION)
        }
    }

    async fn process_airdrop(
        &mut self,
        to: &Pubkey,
        lamports: u64,
    ) -> Result<Signature, ToolboxEndpointError> {
        let signature = self.request_airdrop(to, lamports).await?;
        Ok(signature)
    }

    async fn forward_clock_unix_timestamp(
        &mut self,
        unix_timestamp_delta: u64,
    ) -> Result<(), ToolboxEndpointError> {
        let timer = Instant::now();
        let unix_timestamp_after =
            rpc_get_sysvar_clock(self).await?.unix_timestamp
                + (unix_timestamp_delta as i64);
        loop {
            if rpc_get_sysvar_clock(self).await?.unix_timestamp
                >= unix_timestamp_after
            {
                return Ok(());
            }
            if timer.elapsed() > WAIT_TIMEOUT_DURATION {
                return Err(ToolboxEndpointError::Timeout("Clock forwarding"));
            }
            sleep(WAIT_SLEEP_DURATION)
        }
    }

    async fn forward_clock_slot(
        &mut self,
        slot_delta: u64,
    ) -> Result<(), ToolboxEndpointError> {
        let timer = Instant::now();
        let slot_after = rpc_get_sysvar_clock(self).await?.slot + slot_delta;
        loop {
            if rpc_get_sysvar_clock(self).await?.slot >= slot_after {
                return Ok(());
            }
            if timer.elapsed() > WAIT_TIMEOUT_DURATION {
                return Err(ToolboxEndpointError::Timeout("Clock forwarding"));
            }
            sleep(WAIT_SLEEP_DURATION)
        }
    }

    async fn forward_clock_epoch(
        &mut self,
        epoch_delta: u64,
    ) -> Result<(), ToolboxEndpointError> {
        let timer = Instant::now();
        let epoch_after = rpc_get_sysvar_clock(self).await?.epoch + epoch_delta;
        loop {
            if rpc_get_sysvar_clock(self).await?.epoch >= epoch_after {
                return Ok(());
            }
            if timer.elapsed() > WAIT_TIMEOUT_DURATION {
                return Err(ToolboxEndpointError::Timeout("Clock forwarding"));
            }
            sleep(WAIT_SLEEP_DURATION)
        }
    }
}
