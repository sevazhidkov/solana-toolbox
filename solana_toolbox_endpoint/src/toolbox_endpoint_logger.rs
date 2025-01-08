use solana_sdk::account::Account;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;

use crate::ToolboxEndpointError;

#[derive(Debug, Clone)]
pub struct ToolboxEndpointLoggerTransaction {
    pub payer: Pubkey,
    pub signers: Vec<Pubkey>,
    pub instructions: Vec<ToolboxEndpointLoggerInstruction>,
}

#[derive(Debug, Clone)]
pub struct ToolboxEndpointLoggerInstruction {
    pub program_id: Pubkey,
    pub accounts: Vec<Pubkey>,
    pub data: Vec<u8>,
}

pub trait ToolboxEndpointLogger {
    fn on_transaction(
        &self,
        transaction: &ToolboxEndpointLoggerTransaction,
        result: &Result<Signature, ToolboxEndpointError>,
    );

    fn on_account(
        &self,
        address: &Pubkey,
        account: &Option<Account>,
    );
}
