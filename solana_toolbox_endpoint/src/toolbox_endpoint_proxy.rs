use solana_sdk::account::Account;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::transaction::Transaction;

use crate::toolbox_endpoint_error::ToolboxEndpointError;

#[async_trait::async_trait]
pub trait ToolboxEndpointProxy {
    async fn get_latest_blockhash(
        &mut self
    ) -> Result<Hash, ToolboxEndpointError>;

    async fn get_accounts(
        &mut self,
        addresses: &[Pubkey],
    ) -> Result<Vec<Option<Account>>, ToolboxEndpointError>;

    async fn process_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<Signature, ToolboxEndpointError>;

    async fn process_airdrop(
        &mut self,
        to: &Pubkey,
        lamports: u64,
    ) -> Result<Signature, ToolboxEndpointError>;

    // TODO - this could be split into 2 different calls (one for time, one for slot)
    // TODO - This could also be implemented to a minimum for regular RPCs by waiting

    async fn move_clock_forward(
        &mut self,
        unix_timestamp_delta: u64,
        slot_delta: u64,
    ) -> Result<(), ToolboxEndpointError>;
}
