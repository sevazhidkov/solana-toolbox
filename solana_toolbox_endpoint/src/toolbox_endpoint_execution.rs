use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::TransactionError;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxEndpointExecution {
    pub slot: u64,
    pub payer: Pubkey,
    pub instructions: Vec<Instruction>,
    pub logs: Option<Vec<String>>,
    pub error: Option<TransactionError>,
    pub return_data: Option<Vec<u8>>,
    pub units_consumed: Option<u64>,
}
