use std::time::Duration;
use std::time::Instant;

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::account::Account;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::transaction::Transaction;

use crate::toolbox_endpoint_error::ToolboxEndpointError;
use crate::toolbox_endpoint_proxy::ToolboxEndpointProxy;

#[async_trait::async_trait]
impl ToolboxEndpointProxy for RpcClient {
    async fn get_latest_blockhash(
        &mut self
    ) -> Result<Hash, ToolboxEndpointError> {
        Ok(RpcClient::get_latest_blockhash(self).await?)
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
        let signature = self.send_transaction(&transaction).await?;
        let start = Instant::now();
        loop {
            let confirmed = self.confirm_transaction(&signature).await?;
            if confirmed {
                break;
            }
            let duration = start.elapsed();
            if duration > Duration::from_secs(5) {
                return Err(ToolboxEndpointError::Custom(
                    "Timeout on awaiting transaction confirmation".into(),
                ));
            }
        }
        Ok(signature)
    }

    async fn process_airdrop(
        &mut self,
        to: &Pubkey,
        lamports: u64,
    ) -> Result<Signature, ToolboxEndpointError> {
        let signature = self.request_airdrop(to, lamports).await?;
        Ok(signature)
    }

    async fn move_clock_forward( // TODO - this could be split into 2 different calls (one for time, one for slot)
        &mut self,
        _unix_timestamp_delta: u64,
        _slot_delta: u64,
    ) -> Result<(), ToolboxEndpointError> {
        Err(ToolboxEndpointError::Custom(
            "Clock forwarding not supported on RPCs".into(),
        ))
    }
}
