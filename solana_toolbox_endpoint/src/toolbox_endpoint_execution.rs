use solana_sdk::transaction::TransactionError;
use solana_sdk::transaction::VersionedTransaction;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxEndpointExecution {
    pub versioned_transaction: VersionedTransaction,
    pub slot: u64,
    pub error: Option<TransactionError>,
    pub logs: Option<Vec<String>>,
    pub return_data: Option<Vec<u8>>,
    pub units_consumed: Option<u64>,
}
