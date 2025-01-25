use solana_sdk::signature::Signature;
use solana_sdk::transaction::Transaction;

use crate::toolbox_endpoint_error::ToolboxEndpointError;

#[async_trait::async_trait]
pub trait ToolboxEndpointLogger {
    async fn on_transaction(
        &self,
        transaction: &Transaction,
        result: &Result<Signature, ToolboxEndpointError>,
    );
}
