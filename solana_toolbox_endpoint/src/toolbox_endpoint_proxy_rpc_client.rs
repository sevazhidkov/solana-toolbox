use std::collections::HashSet;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_sdk::account::Account;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::sysvar::clock::Clock;
use solana_sdk::transaction::VersionedTransaction;
use solana_transaction_status::UiReturnDataEncoding;
use solana_transaction_status::UiTransactionReturnData;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_error::ToolboxEndpointError;
use crate::toolbox_endpoint_execution::ToolboxEndpointExecution;
use crate::toolbox_endpoint_proxy::ToolboxEndpointProxy;

const WAIT_SLEEP_DURATION: Duration = Duration::from_millis(100);
const WAIT_TIMEOUT_DURATION: Duration = Duration::from_secs(30);

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
        versioned_transaction: VersionedTransaction,
    ) -> Result<ToolboxEndpointExecution, ToolboxEndpointError> {
        ToolboxEndpointProxyRpcClient::simulate_transaction_using_rpc(
            &self.inner,
            versioned_transaction,
        )
        .await
    }

    async fn process_transaction(
        &mut self,
        versioned_transaction: VersionedTransaction,
        skip_preflight: bool,
    ) -> Result<(Signature, ToolboxEndpointExecution), ToolboxEndpointError>
    {
        self.wait_until_execution(
            &self
                .inner
                .send_transaction_with_config(
                    &versioned_transaction,
                    RpcSendTransactionConfig {
                        skip_preflight,
                        preflight_commitment: None,
                        encoding: None,
                        max_retries: None,
                        min_context_slot: None,
                    },
                )
                .await?,
        )
        .await
    }

    async fn request_airdrop(
        &mut self,
        to: &Pubkey,
        lamports: u64,
    ) -> Result<(Signature, ToolboxEndpointExecution), ToolboxEndpointError>
    {
        self.wait_until_execution(
            &self.inner.request_airdrop(to, lamports).await?,
        )
        .await
    }

    async fn get_execution(
        &mut self,
        signature: &Signature,
    ) -> Result<ToolboxEndpointExecution, ToolboxEndpointError> {
        ToolboxEndpointProxyRpcClient::get_execution_using_rpc(
            &self.inner,
            signature,
        )
        .await?
        .ok_or_else(|| ToolboxEndpointError::UnknownSignature(*signature))
    }

    async fn search_addresses(
        &mut self,
        program_id: &Pubkey,
        data_len: Option<usize>,
        data_chunks: &[(usize, &[u8])],
    ) -> Result<HashSet<Pubkey>, ToolboxEndpointError> {
        ToolboxEndpointProxyRpcClient::search_addresses_using_rpc(
            &self.inner,
            program_id,
            data_len,
            data_chunks,
        )
        .await
    }

    async fn search_signatures(
        &mut self,
        address: &Pubkey,
        start_before: Option<Signature>,
        rewind_until: Option<Signature>,
        limit: usize,
    ) -> Result<Vec<Signature>, ToolboxEndpointError> {
        ToolboxEndpointProxyRpcClient::search_signatures_using_rpc(
            &self.inner,
            address,
            start_before,
            rewind_until,
            limit,
        )
        .await
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
    async fn wait_until_execution(
        &mut self,
        signature: &Signature,
    ) -> Result<(Signature, ToolboxEndpointExecution), ToolboxEndpointError>
    {
        let timer = Instant::now();
        loop {
            if let Some(execution) =
                ToolboxEndpointProxyRpcClient::get_execution_using_rpc(
                    &self.inner,
                    signature,
                )
                .await?
            {
                return Ok((*signature, execution));
            }
            if timer.elapsed() > WAIT_TIMEOUT_DURATION {
                return Err(ToolboxEndpointError::Timeout(
                    "Waiting confirmation",
                ));
            }
            sleep(WAIT_SLEEP_DURATION)
        }
    }

    async fn get_sysvar_clock(
        &mut self
    ) -> Result<Clock, ToolboxEndpointError> {
        bincode::deserialize::<Clock>(
            &self
                .inner
                .get_account(&ToolboxEndpoint::SYSVAR_CLOCK_ID)
                .await?
                .data,
        )
        .map_err(ToolboxEndpointError::Bincode)
    }

    pub(crate) fn decode_transaction_return_data(
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
