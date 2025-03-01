use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;

use solana_sdk::signature::Signature;

use crate::toolbox_endpoint_execution::ToolboxEndpointExecution;
use crate::toolbox_endpoint_logger::ToolboxEndpointLogger;

#[derive(Debug, Clone, Default)]
pub struct ToolboxEndpointLoggerHistory {
    processed: Arc<RwLock<Vec<(Signature, ToolboxEndpointExecution)>>>,
}

impl ToolboxEndpointLoggerHistory {
    pub fn new() -> ToolboxEndpointLoggerHistory {
        ToolboxEndpointLoggerHistory { ..Default::default() }
    }

    pub fn get_processed(
        &self
    ) -> RwLockReadGuard<Vec<(Signature, ToolboxEndpointExecution)>> {
        self.processed.read().unwrap()
    }
}

#[async_trait::async_trait]
impl ToolboxEndpointLogger for ToolboxEndpointLoggerHistory {
    async fn on_processed(
        &self,
        processed: &(Signature, ToolboxEndpointExecution),
    ) {
        self.processed.write().unwrap().push(processed.clone());
    }
}
