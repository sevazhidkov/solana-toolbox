use std::collections::HashSet;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

use solana_account_decoder::UiAccountEncoding;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::RpcAccountInfoConfig;
use solana_client::rpc_config::RpcRequestAirdropConfig;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_sdk::account::Account;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::sysvar::clock::Clock;
use solana_sdk::transaction::VersionedTransaction;
use solana_transaction_status::UiTransactionEncoding;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_error::ToolboxEndpointError;
use crate::toolbox_endpoint_execution::ToolboxEndpointExecution;
use crate::toolbox_endpoint_proxy::ToolboxEndpointProxy;

const WAIT_SLEEP_DURATION: Duration = Duration::from_millis(100);
const WAIT_TIMEOUT_DURATION: Duration = Duration::from_secs(30);

pub struct ToolboxEndpointProxyRpcClient {
    pub(crate) inner: RpcClient,
}

impl ToolboxEndpointProxyRpcClient {
    pub fn new(rpc_client: RpcClient) -> ToolboxEndpointProxyRpcClient {
        ToolboxEndpointProxyRpcClient { inner: rpc_client }
    }
}

#[async_trait::async_trait]
impl ToolboxEndpointProxy for ToolboxEndpointProxyRpcClient {
    async fn get_latest_blockhash(
        &mut self,
    ) -> Result<Hash, ToolboxEndpointError> {
        Ok(self
            .inner
            .get_latest_blockhash_with_commitment(self.get_commitment())
            .await?
            .0)
    }

    async fn get_balance(
        &mut self,
        address: &Pubkey,
    ) -> Result<u64, ToolboxEndpointError> {
        Ok(self
            .inner
            .get_balance_with_commitment(address, self.get_commitment())
            .await?
            .value)
    }

    async fn get_account(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<Account>, ToolboxEndpointError> {
        Ok(self
            .inner
            .get_account_with_config(
                address,
                RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::Base64Zstd),
                    data_slice: None,
                    commitment: Some(self.get_commitment()),
                    min_context_slot: None,
                },
            )
            .await?
            .value)
    }

    async fn get_accounts(
        &mut self,
        addresses: &[Pubkey],
    ) -> Result<Vec<Option<Account>>, ToolboxEndpointError> {
        Ok(self
            .inner
            .get_multiple_accounts_with_config(
                addresses,
                RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::Base64Zstd),
                    data_slice: None,
                    commitment: Some(self.get_commitment()),
                    min_context_slot: None,
                },
            )
            .await?
            .value)
    }

    async fn simulate_transaction(
        &mut self,
        versioned_transaction: VersionedTransaction,
    ) -> Result<ToolboxEndpointExecution, ToolboxEndpointError> {
        self.simulate_transaction_using_rpc(versioned_transaction)
            .await
    }

    async fn process_transaction(
        &mut self,
        versioned_transaction: VersionedTransaction,
        skip_preflight: bool,
    ) -> Result<(Signature, ToolboxEndpointExecution), ToolboxEndpointError>
    {
        let signature = self
            .inner
            .send_transaction_with_config(
                &versioned_transaction,
                RpcSendTransactionConfig {
                    skip_preflight,
                    preflight_commitment: Some(
                        self.get_commitment().commitment,
                    ),
                    encoding: Some(UiTransactionEncoding::Base64),
                    max_retries: None,
                    min_context_slot: None,
                },
            )
            .await?;
        self.wait_until_execution(&signature).await
    }

    async fn request_airdrop(
        &mut self,
        to: &Pubkey,
        lamports: u64,
    ) -> Result<(Signature, ToolboxEndpointExecution), ToolboxEndpointError>
    {
        let signature = self
            .inner
            .request_airdrop_with_config(
                to,
                lamports,
                RpcRequestAirdropConfig {
                    recent_blockhash: None,
                    commitment: Some(self.get_commitment()),
                },
            )
            .await?;
        self.wait_until_execution(&signature).await
    }

    async fn get_execution(
        &mut self,
        signature: &Signature,
    ) -> Result<ToolboxEndpointExecution, ToolboxEndpointError> {
        self.get_execution_using_rpc(signature)
            .await?
            .ok_or_else(|| ToolboxEndpointError::UnknownSignature(*signature))
    }

    async fn search_addresses(
        &mut self,
        program_id: &Pubkey,
        data_len: Option<usize>,
        data_chunks: &[(usize, &[u8])],
    ) -> Result<HashSet<Pubkey>, ToolboxEndpointError> {
        self.search_addresses_using_rpc(program_id, data_len, data_chunks)
            .await
    }

    async fn search_signatures(
        &mut self,
        address: &Pubkey,
        start_before: Option<Signature>,
        rewind_until: Option<Signature>,
        limit: usize,
    ) -> Result<Vec<Signature>, ToolboxEndpointError> {
        self.search_signatures_using_rpc(
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
        let until_unix_timestamp =
            self.get_sysvar_clock().await?.unix_timestamp
                + (unix_timestamp_delta as i64);
        self.wait_until_clock(Some(until_unix_timestamp), None, None)
            .await
    }

    async fn forward_clock_slot(
        &mut self,
        slot_delta: u64,
    ) -> Result<(), ToolboxEndpointError> {
        let until_slot = self.get_sysvar_clock().await?.slot + slot_delta;
        self.wait_until_clock(None, Some(until_slot), None).await
    }

    async fn forward_clock_epoch(
        &mut self,
        epoch_delta: u64,
    ) -> Result<(), ToolboxEndpointError> {
        let until_epoch = self.get_sysvar_clock().await?.epoch + epoch_delta;
        self.wait_until_clock(None, None, Some(until_epoch)).await
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
                self.get_execution_using_rpc(signature).await?
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

    async fn wait_until_clock(
        &mut self,
        until_unix_timestamp: Option<i64>,
        until_slot: Option<u64>,
        until_epoch: Option<u64>,
    ) -> Result<(), ToolboxEndpointError> {
        let timer = Instant::now();
        loop {
            let clock = self.get_sysvar_clock().await?;
            if let Some(until_unix_timestamp) = until_unix_timestamp {
                if clock.unix_timestamp >= until_unix_timestamp {
                    return Ok(());
                }
            }
            if let Some(until_slot) = until_slot {
                if clock.slot >= until_slot {
                    return Ok(());
                }
            }
            if let Some(until_epoch) = until_epoch {
                if clock.epoch >= until_epoch {
                    return Ok(());
                }
            }
            if timer.elapsed() > WAIT_TIMEOUT_DURATION {
                return Err(ToolboxEndpointError::Timeout("Clock forwarding"));
            }
            sleep(WAIT_SLEEP_DURATION)
        }
    }

    async fn get_sysvar_clock(
        &mut self,
    ) -> Result<Clock, ToolboxEndpointError> {
        bincode::deserialize::<Clock>(
            &self
                .get_account(&ToolboxEndpoint::SYSVAR_CLOCK_ID)
                .await?
                .ok_or_else(|| {
                    ToolboxEndpointError::AccountDoesNotExist(
                        ToolboxEndpoint::SYSVAR_CLOCK_ID,
                        "Sysvar Clock".to_string(),
                    )
                })?
                .data,
        )
        .map_err(ToolboxEndpointError::Bincode)
    }
}
