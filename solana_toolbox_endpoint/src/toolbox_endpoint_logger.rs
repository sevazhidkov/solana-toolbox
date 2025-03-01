use solana_sdk::signature::Signature;

use crate::toolbox_endpoint_execution::ToolboxEndpointExecution;

#[async_trait::async_trait]
pub trait ToolboxEndpointLogger {
    async fn on_processed(
        &self,
        processed: &(Signature, ToolboxEndpointExecution),
    );
}
