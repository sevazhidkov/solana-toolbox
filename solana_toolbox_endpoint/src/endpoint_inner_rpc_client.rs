use std::time::Duration;
use std::time::Instant;

use async_trait::async_trait;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::account::Account;
use solana_sdk::clock::Clock;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::sysvar::clock;
use solana_sdk::transaction::Transaction;
use tokio::time::sleep;

use crate::endpoint_error::EndpointError;
use crate::endpoint_inner::EndpointInner;

#[async_trait]
impl EndpointInner for RpcClient {
    async fn get_latest_blockhash(&mut self) -> Result<Hash, EndpointError> {
        RpcClient::get_latest_blockhash(self)
            .await
            .map_err(EndpointError::Client)
    }

    async fn get_rent_minimum_balance(
        &mut self,
        space: usize,
    ) -> Result<u64, EndpointError> {
        self.get_minimum_balance_for_rent_exemption(space)
            .await
            .map_err(EndpointError::Client)
    }

    async fn get_clock(&mut self) -> Result<Clock, EndpointError> {
        let accounts = self.get_accounts(&[clock::ID]).await?;
        match &accounts[0] {
            Some(account) => {
                Ok(bincode::deserialize(&account.data).ok().ok_or(
                    EndpointError::Custom("sysvar clock failed to deserialize"),
                )?)
            },
            None => Err(EndpointError::Custom("sysvar clock not found")),
        }
    }

    async fn get_accounts(
        &mut self,
        addresses: &[Pubkey],
    ) -> Result<Vec<Option<Account>>, EndpointError> {
        self.get_multiple_accounts(addresses)
            .await
            .map_err(EndpointError::Client)
    }

    async fn process_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<Signature, EndpointError> {
        let signature = self
            .send_transaction(&transaction)
            .await
            .map_err(EndpointError::Client)?;
        let start = Instant::now();
        loop {
            let confirmed = self
                .confirm_transaction(&signature)
                .await
                .map_err(EndpointError::Client)?;
            if confirmed {
                break;
            }
            let duration = start.elapsed();
            if duration > Duration::from_secs(5) {
                return Err(EndpointError::Custom(
                    "Timeout on awaiting confirmation",
                ));
            }
            sleep(Duration::from_secs(1)).await;
        }
        Ok(signature)
    }

    async fn process_airdrop(
        &mut self,
        to: &Pubkey,
        lamports: u64,
    ) -> Result<Signature, EndpointError> {
        let signature = self
            .request_airdrop(to, lamports)
            .await
            .map_err(EndpointError::Client)?;
        Ok(signature)
    }

    async fn move_clock_forward(
        &mut self,
        _unix_timestamp_delta: u64,
        _slot_delta: u64,
    ) -> Result<(), EndpointError> {
        Err(EndpointError::Custom("Clock forward not supported on RPCs"))
    }
}
