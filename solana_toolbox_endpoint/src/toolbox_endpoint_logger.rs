use solana_sdk::account::Account;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;

use crate::toolbox_endpoint_error::ToolboxEndpointError;

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

#[async_trait::async_trait]
pub trait ToolboxEndpointLogger {
    async fn on_transaction(
        &self,
        transaction: &ToolboxEndpointLoggerTransaction,
        result: &Result<Signature, ToolboxEndpointError>,
    );

    async fn on_account(
        &self,
        address: &Pubkey,
        account: &Option<Account>,
    );
}
