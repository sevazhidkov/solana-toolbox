use async_trait::async_trait;
use solana_sdk::account::Account;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::sysvar::clock::Clock;
use solana_sdk::transaction::Transaction;

use crate::endpoint_error::EndpointError;

#[async_trait]
pub trait EndpointInner {
    async fn get_latest_blockhash(&mut self) -> Result<Hash, EndpointError>;

    async fn get_rent_minimum_balance(
        &mut self,
        space: usize,
    ) -> Result<u64, EndpointError>;

    async fn get_clock(&mut self) -> Result<Clock, EndpointError>;

    async fn get_accounts(
        &mut self,
        addresses: &[Pubkey],
    ) -> Result<Vec<Option<Account>>, EndpointError>;

    async fn process_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<Signature, EndpointError>;

    async fn process_airdrop(
        &mut self,
        to: &Pubkey,
        lamports: u64,
    ) -> Result<Signature, EndpointError>;

    async fn move_clock_forward(
        &mut self,
        unix_timestamp_delta: u64,
        slot_delta: u64,
    ) -> Result<(), EndpointError>;
}
