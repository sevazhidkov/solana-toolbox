use solana_sdk::transaction::Transaction;

use crate::toolbox_endpoint::ToolboxEndpoint;

impl ToolboxEndpoint {
    pub fn signature_fees_lamports() -> u64 {
        return 5_000;
    }

    pub fn signatures_fees_lamports(signatures: u64) -> u64 {
        ToolboxEndpoint::signature_fees_lamports() * signatures
    }

    pub fn transaction_fees_lamports(transaction: &Transaction) -> u64 {
        ToolboxEndpoint::signatures_fees_lamports(
            transaction.signatures.len().try_into().unwrap(),
        )
    }
}
