use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::TransactionError;

// TODO - find a better name ?
#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxEndpointExecution {
    pub payer: Pubkey,
    pub instructions: Vec<Instruction>,
    pub slot: u64,
    pub error: Option<TransactionError>,
    pub logs: Option<Vec<String>>,
    pub return_data: Option<Vec<u8>>,
    pub units_consumed: Option<u64>,
}
