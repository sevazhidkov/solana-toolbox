use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;

use solana_sdk::signature::Signature;

use crate::toolbox_endpoint_logger::ToolboxEndpointLogger;

#[derive(Debug, Clone, Default)]
pub struct ToolboxEndpointLoggerHistory {
    signatures: Arc<RwLock<Vec<Signature>>>,
}

impl ToolboxEndpointLoggerHistory {
    pub fn new() -> ToolboxEndpointLoggerHistory {
        ToolboxEndpointLoggerHistory { ..Default::default() }
    }

    pub fn get_signatures(&self) -> RwLockReadGuard<Vec<Signature>> {
        self.signatures.read().unwrap()
    }
}

#[async_trait::async_trait]
impl ToolboxEndpointLogger for ToolboxEndpointLoggerHistory {
    async fn on_signature(
        &self,
        signature: &Signature,
    ) {
        self.signatures.write().unwrap().push(*signature);
    }
}
