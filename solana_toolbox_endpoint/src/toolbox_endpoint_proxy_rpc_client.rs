use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::account::Account;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::sysvar::clock;
use solana_sdk::sysvar::clock::Clock;
use solana_sdk::transaction::Transaction;
use solana_transaction_status::UiReturnDataEncoding;
use solana_transaction_status::UiTransactionEncoding;
use solana_transaction_status::UiTransactionReturnData;

use crate::toolbox_endpoint_error::ToolboxEndpointError;
use crate::toolbox_endpoint_execution::ToolboxEndpointExecution;
use crate::toolbox_endpoint_proxy::ToolboxEndpointProxy;

const WAIT_SLEEP_DURATION: Duration = Duration::from_millis(100);
const WAIT_TIMEOUT_DURATION: Duration = Duration::from_secs(10);

pub struct ToolboxEndpointProxyRpcClient {
    inner: RpcClient,
}

impl ToolboxEndpointProxyRpcClient {
    pub fn new(rpc_client: RpcClient) -> ToolboxEndpointProxyRpcClient {
        ToolboxEndpointProxyRpcClient { inner: rpc_client }
    }
}

#[async_trait::async_trait]
impl ToolboxEndpointProxy for ToolboxEndpointProxyRpcClient {
    async fn get_latest_blockhash(
        &mut self
    ) -> Result<Hash, ToolboxEndpointError> {
        Ok(self.inner.get_latest_blockhash().await?)
    }

    async fn get_balance(
        &mut self,
        address: &Pubkey,
    ) -> Result<u64, ToolboxEndpointError> {
        Ok(self.inner.get_balance(address).await?)
    }

    async fn get_accounts(
        &mut self,
        addresses: &[Pubkey],
    ) -> Result<Vec<Option<Account>>, ToolboxEndpointError> {
        Ok(self.inner.get_multiple_accounts(addresses).await?)
    }

    async fn simulate_transaction(
        &mut self,
        transaction: &Transaction,
    ) -> Result<ToolboxEndpointExecution, ToolboxEndpointError> {
        let outcome = self.inner.simulate_transaction(transaction).await?;
        Ok(ToolboxEndpointExecution {
            slot: outcome.context.slot,
            error: outcome.value.err,
            logs: outcome.value.logs,
            return_data: ToolboxEndpointProxyRpcClient::prepare_return_data(
                outcome.value.return_data.into(),
            )?,
            units_consumed: outcome.value.units_consumed,
        })
    }

    async fn process_transaction(
        &mut self,
        transaction: &Transaction,
    ) -> Result<Signature, ToolboxEndpointError> {
        let timer = Instant::now();
        let signature = self.inner.send_transaction(transaction).await?;
        loop {
            if self.inner.confirm_transaction(&signature).await? {
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

    async fn request_airdrop(
        &mut self,
        to: &Pubkey,
        lamports: u64,
    ) -> Result<Signature, ToolboxEndpointError> {
        Ok(self.inner.request_airdrop(to, lamports).await?)
    }

    async fn get_execution(
        &mut self,
        signature: &Signature,
    ) -> Result<ToolboxEndpointExecution, ToolboxEndpointError> {
        let outcome = self
            .inner
            .get_transaction(signature, UiTransactionEncoding::Base64)
            .await?;
        match outcome.transaction.meta {
            Some(metadata) => {
                Ok(ToolboxEndpointExecution {
                    slot: outcome.slot,
                    error: metadata.err,
                    logs: metadata.log_messages.into(),
                    return_data:
                        ToolboxEndpointProxyRpcClient::prepare_return_data(
                            metadata.return_data.into(),
                        )?,
                    units_consumed: metadata.compute_units_consumed.into(),
                })
            },
            None => {
                Err(ToolboxEndpointError::Custom(
                    "Unknown transaction execution".to_string(),
                ))
            },
        }
    }

    async fn forward_clock_unix_timestamp(
        &mut self,
        unix_timestamp_delta: u64,
    ) -> Result<(), ToolboxEndpointError> {
        let timer = Instant::now();
        let unix_timestamp_after =
            self.get_sysvar_clock().await?.unix_timestamp
                + (unix_timestamp_delta as i64);
        loop {
            if self.get_sysvar_clock().await?.unix_timestamp
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
        let slot_after = self.get_sysvar_clock().await?.slot + slot_delta;
        loop {
            if self.get_sysvar_clock().await?.slot >= slot_after {
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
        let epoch_after = self.get_sysvar_clock().await?.epoch + epoch_delta;
        loop {
            if self.get_sysvar_clock().await?.epoch >= epoch_after {
                return Ok(());
            }
            if timer.elapsed() > WAIT_TIMEOUT_DURATION {
                return Err(ToolboxEndpointError::Timeout("Clock forwarding"));
            }
            sleep(WAIT_SLEEP_DURATION)
        }
    }
}

impl ToolboxEndpointProxyRpcClient {
    async fn get_sysvar_clock(
        &mut self
    ) -> Result<Clock, ToolboxEndpointError> {
        bincode::deserialize::<Clock>(
            &self.inner.get_account(&clock::ID).await?.data,
        )
        .map_err(ToolboxEndpointError::Bincode)
    }

    fn prepare_return_data(
        return_data: Option<UiTransactionReturnData>
    ) -> Result<Option<Vec<u8>>, ToolboxEndpointError> {
        return_data
            .map(|return_data| {
                let (payload, encoding) = return_data.data;
                if encoding != UiReturnDataEncoding::Base64 {
                    return Err(ToolboxEndpointError::Custom(
                        "Unknown return data encoding".to_string(),
                    ));
                }
                STANDARD
                    .decode(payload)
                    .map_err(ToolboxEndpointError::Base64Decode)
            })
            .transpose()
    }
}
