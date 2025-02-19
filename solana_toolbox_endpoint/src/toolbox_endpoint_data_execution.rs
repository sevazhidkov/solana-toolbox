use solana_sdk::transaction::TransactionError;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxEndpointDataExecution {
    pub slot: u64,
    pub error: Option<TransactionError>,
    pub logs: Option<Vec<String>>,
    pub return_data: Option<Vec<u8>>,
    pub units_consumed: Option<u64>,
}
