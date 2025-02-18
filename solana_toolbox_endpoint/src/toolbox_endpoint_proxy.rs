use solana_sdk::account::Account;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::transaction::Transaction;

use crate::toolbox_endpoint_error::ToolboxEndpointError;
use crate::toolbox_endpoint_execution::ToolboxEndpointExecution;

#[async_trait::async_trait]
pub trait ToolboxEndpointProxy {
    async fn get_latest_blockhash(
        &mut self
    ) -> Result<Hash, ToolboxEndpointError>;

    async fn get_balance(
        &mut self,
        address: &Pubkey,
    ) -> Result<u64, ToolboxEndpointError>;

    async fn get_accounts(
        &mut self,
        addresses: &[Pubkey],
    ) -> Result<Vec<Option<Account>>, ToolboxEndpointError>;

    async fn simulate_transaction(
        &mut self,
        transaction: &Transaction, // TODO - support versionned transactions
    ) -> Result<ToolboxEndpointExecution, ToolboxEndpointError>;

    async fn process_transaction(
        &mut self,
        transaction: &Transaction, // TODO - support versionned transactions
    ) -> Result<Signature, ToolboxEndpointError>;

    async fn request_airdrop(
        &mut self,
        address: &Pubkey,
        lamports: u64,
    ) -> Result<Signature, ToolboxEndpointError>;

    async fn get_execution(
        &mut self,
        signature: &Signature,
    ) -> Result<ToolboxEndpointExecution, ToolboxEndpointError>;

    async fn forward_clock_unix_timestamp(
        &mut self,
        unix_timestamp_delta: u64,
    ) -> Result<(), ToolboxEndpointError>;

    async fn forward_clock_slot(
        &mut self,
        slot_delta: u64,
    ) -> Result<(), ToolboxEndpointError>;

    async fn forward_clock_epoch(
        &mut self,
        epoch_delta: u64,
    ) -> Result<(), ToolboxEndpointError>;
}
