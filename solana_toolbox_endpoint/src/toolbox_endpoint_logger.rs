use solana_sdk::account::Account;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;

use crate::toolbox_endpoint_error::ToolboxEndpointError;
use crate::toolbox_endpoint_transaction::ToolboxEndpointTransaction;

#[async_trait::async_trait]
pub trait ToolboxEndpointLogger {
    async fn on_transaction(
        &self,
        transaction: &ToolboxEndpointTransaction,
        result: &Result<Signature, ToolboxEndpointError>,
    );

    async fn on_account(
        &self,
        address: &Pubkey,
        account: &Option<Account>,
    );
}
