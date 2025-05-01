use std::collections::HashSet;

use anyhow::Result;
use solana_sdk::account::Account;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::transaction::VersionedTransaction;

use crate::toolbox_endpoint_execution::ToolboxEndpointExecution;

#[async_trait::async_trait]
pub trait ToolboxEndpointProxy {
    async fn get_latest_blockhash(&mut self) -> Result<Hash>;

    async fn get_slot_unix_timestamp(&mut self, slot: u64) -> Result<i64>;

    async fn get_balance(&mut self, address: &Pubkey) -> Result<u64>;

    async fn get_account(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<Account>>;

    async fn get_accounts(
        &mut self,
        addresses: &[Pubkey],
    ) -> Result<Vec<Option<Account>>>;

    async fn simulate_transaction(
        &mut self,
        versioned_transaction: VersionedTransaction,
        verify_signatures: bool,
    ) -> Result<ToolboxEndpointExecution>;

    async fn process_transaction(
        &mut self,
        versioned_transaction: VersionedTransaction,
        verify_prelight: bool,
    ) -> Result<(Signature, ToolboxEndpointExecution)>;

    async fn request_airdrop(
        &mut self,
        address: &Pubkey,
        lamports: u64,
    ) -> Result<(Signature, ToolboxEndpointExecution)>;

    async fn get_execution(
        &mut self,
        signature: &Signature,
    ) -> Result<ToolboxEndpointExecution>;

    async fn search_addresses(
        &mut self,
        program_id: &Pubkey,
        data_len: Option<usize>,
        data_chunks: &[(usize, &[u8])],
    ) -> Result<HashSet<Pubkey>>;

    async fn search_signatures(
        &mut self,
        address: &Pubkey,
        start_before: Option<Signature>,
        rewind_until: Option<Signature>,
        limit: usize,
    ) -> Result<Vec<Signature>>;

    async fn forward_clock_unix_timestamp(
        &mut self,
        unix_timestamp_delta: u64,
    ) -> Result<()>;

    async fn forward_clock_slot(&mut self, slot_delta: u64) -> Result<()>;

    async fn forward_clock_epoch(&mut self, epoch_delta: u64) -> Result<()>;
}
