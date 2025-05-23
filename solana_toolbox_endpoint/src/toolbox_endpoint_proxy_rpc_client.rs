use std::collections::HashSet;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

use anyhow::anyhow;
use anyhow::Result;
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
use crate::toolbox_endpoint_execution::ToolboxEndpointExecution;
use crate::toolbox_endpoint_proxy::ToolboxEndpointProxy;

const WAIT_SLEEP_DURATION: Duration = Duration::from_secs(1);
const WAIT_TIMEOUT_DURATION: Duration = Duration::from_secs(30);

pub struct ToolboxEndpointProxyRpcClient {
    pub(crate) rpc_client: RpcClient,
}

impl ToolboxEndpointProxyRpcClient {
    pub fn new(rpc_client: RpcClient) -> ToolboxEndpointProxyRpcClient {
        ToolboxEndpointProxyRpcClient { rpc_client }
    }
}

#[async_trait::async_trait]
impl ToolboxEndpointProxy for ToolboxEndpointProxyRpcClient {
    async fn get_latest_blockhash(&mut self) -> Result<Hash> {
        Ok(self
            .rpc_client
            .get_latest_blockhash_with_commitment(self.get_commitment())
            .await?
            .0)
    }

    async fn get_slot_unix_timestamp(&mut self, slot: u64) -> Result<i64> {
        Ok(self.rpc_client.get_block_time(slot).await?)
    }

    async fn get_balance(&mut self, address: &Pubkey) -> Result<u64> {
        Ok(self
            .rpc_client
            .get_balance_with_commitment(address, self.get_commitment())
            .await?
            .value)
    }

    async fn get_account(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<Account>> {
        Ok(self
            .rpc_client
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
    ) -> Result<Vec<Option<Account>>> {
        Ok(self
            .rpc_client
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
        verify_signatures: bool,
    ) -> Result<ToolboxEndpointExecution> {
        self.simulate_transaction_using_rpc(
            versioned_transaction,
            verify_signatures,
        )
        .await
    }

    async fn process_transaction(
        &mut self,
        versioned_transaction: VersionedTransaction,
        process_preflight: bool,
    ) -> Result<(Signature, ToolboxEndpointExecution)> {
        let signature = self
            .rpc_client
            .send_transaction_with_config(
                &versioned_transaction,
                RpcSendTransactionConfig {
                    skip_preflight: !process_preflight,
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
    ) -> Result<(Signature, ToolboxEndpointExecution)> {
        let signature = self
            .rpc_client
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
    ) -> Result<ToolboxEndpointExecution> {
        self.get_execution_using_rpc(signature)
            .await?
            .ok_or_else(|| {
                anyhow!("Unknown execution signature: {}", signature)
            })
    }

    async fn search_addresses(
        &mut self,
        program_id: &Pubkey,
        data_len: Option<usize>,
        data_chunks: &[(usize, &[u8])],
    ) -> Result<HashSet<Pubkey>> {
        self.search_addresses_using_rpc(program_id, data_len, data_chunks)
            .await
    }

    async fn search_signatures(
        &mut self,
        address: &Pubkey,
        limit: usize,
        start_before: Option<Signature>,
        rewind_until: Option<Signature>,
    ) -> Result<Vec<Signature>> {
        self.search_signatures_using_rpc(
            address,
            limit,
            start_before,
            rewind_until,
        )
        .await
    }

    async fn forward_clock_unix_timestamp(
        &mut self,
        unix_timestamp_delta: u64,
    ) -> Result<()> {
        let until_unix_timestamp =
            self.get_sysvar_clock().await?.unix_timestamp
                + (unix_timestamp_delta as i64);
        self.wait_until_clock(Some(until_unix_timestamp), None, None)
            .await
    }

    async fn forward_clock_slot(&mut self, slot_delta: u64) -> Result<()> {
        let until_slot = self.get_sysvar_clock().await?.slot + slot_delta;
        self.wait_until_clock(None, Some(until_slot), None).await
    }

    async fn forward_clock_epoch(&mut self, epoch_delta: u64) -> Result<()> {
        let until_epoch = self.get_sysvar_clock().await?.epoch + epoch_delta;
        self.wait_until_clock(None, None, Some(until_epoch)).await
    }
}

impl ToolboxEndpointProxyRpcClient {
    async fn wait_until_execution(
        &mut self,
        signature: &Signature,
    ) -> Result<(Signature, ToolboxEndpointExecution)> {
        let timer = Instant::now();
        loop {
            if let Some(execution) =
                self.get_execution_using_rpc(signature).await?
            {
                return Ok((*signature, execution));
            }
            if timer.elapsed() > WAIT_TIMEOUT_DURATION {
                return Err(anyhow!("Waiting confirmation"));
            }
            sleep(WAIT_SLEEP_DURATION)
        }
    }

    async fn wait_until_clock(
        &mut self,
        until_unix_timestamp: Option<i64>,
        until_slot: Option<u64>,
        until_epoch: Option<u64>,
    ) -> Result<()> {
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
                return Err(anyhow!("Clock forwarding timeout"));
            }
            sleep(WAIT_SLEEP_DURATION)
        }
    }

    async fn get_sysvar_clock(&mut self) -> Result<Clock> {
        Ok(bincode::deserialize::<Clock>(
            &self
                .get_account(&ToolboxEndpoint::SYSVAR_CLOCK_ID)
                .await?
                .ok_or_else(|| {
                    anyhow!(
                        "Account does not exists: {} (system clock)",
                        ToolboxEndpoint::SYSVAR_CLOCK_ID,
                    )
                })?
                .data,
        )?)
    }
}
