use solana_sdk::signature::Signature;
use solana_sdk::transaction::Transaction;

use crate::toolbox_endpoint_error::ToolboxEndpointError;
use crate::toolbox_endpoint_logger::ToolboxEndpointLogger;
use crate::toolbox_endpoint_printer::ToolboxEndpointPrinter;

#[derive(Default)]
pub struct ToolboxEndpointLoggerPrinter {}

#[async_trait::async_trait]
impl ToolboxEndpointLogger for ToolboxEndpointLoggerPrinter {
    async fn on_signature(
        &self,
        signature: &Signature,
    ) {
        // println!("---------------------------- TRANSACTION PROCESSED -----------------------------");
        // ToolboxEndpointPrinter::print_transaction(transaction);
        // println!("----");
        // match result {
        // Ok(signature) => {
        // println!("transaction.result: OK");
        // if *signature != Signature::default() {
        // println!("transaction.signature: {:?}", signature)
        // }
        // },
        // Err(error) => {
        // println!("transaction.result: FAIL");
        // println!("transaction.error: {:?}", error)
        // },
        // };
        // println!("----");
        // ToolboxEndpointPrinter::print_backtrace("from");
        // println!();
    }
}
