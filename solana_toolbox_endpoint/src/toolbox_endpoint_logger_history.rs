use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;

use solana_sdk::signature::Signature;
use solana_sdk::transaction::Transaction;

use crate::toolbox_endpoint_error::ToolboxEndpointError;
use crate::toolbox_endpoint_logger::ToolboxEndpointLogger;

#[derive(Debug, Clone)]
pub struct ToolboxEndpointLoggerHistoryTransaction {
    pub transaction: Transaction,
    pub result: Result<Signature, String>,
}

#[derive(Debug, Clone, Default)]
pub struct ToolboxEndpointLoggerHistory {
    transactions: Arc<RwLock<Vec<ToolboxEndpointLoggerHistoryTransaction>>>,
}

impl ToolboxEndpointLoggerHistory {
    pub fn new() -> ToolboxEndpointLoggerHistory {
        ToolboxEndpointLoggerHistory { ..Default::default() }
    }

    pub fn get_transactions(
        &self
    ) -> RwLockReadGuard<Vec<ToolboxEndpointLoggerHistoryTransaction>> {
        self.transactions.read().unwrap()
    }
}

#[async_trait::async_trait]
impl ToolboxEndpointLogger for ToolboxEndpointLoggerHistory {
    async fn on_transaction(
        &self,
        transaction: &Transaction,
        result: &Result<Signature, ToolboxEndpointError>,
    ) {
        // TODO - could this use a globally used API struct instead ?
        self.transactions.write().unwrap().push(
            ToolboxEndpointLoggerHistoryTransaction {
                transaction: transaction.clone(),
                result: result
                    .as_ref()
                    .map_err(|err| format!("{:?}", err))
                    .copied(),
            },
        );
    }
}
