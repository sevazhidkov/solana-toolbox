use solana_sdk::account::Account;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;

use crate::toolbox_endpoint_error::ToolboxEndpointError;
use crate::toolbox_endpoint_logger::ToolboxEndpointLogger;
use crate::toolbox_endpoint_printer::ToolboxEndpointPrinter;
use crate::toolbox_endpoint_transaction::ToolboxEndpointTransaction;

#[derive(Default)]
pub struct ToolboxEndpointLoggerPrinter {}

#[async_trait::async_trait]
impl ToolboxEndpointLogger for ToolboxEndpointLoggerPrinter {
    async fn on_transaction(
        &self,
        transaction: &ToolboxEndpointTransaction,
        result: &Result<Signature, ToolboxEndpointError>,
    ) {
        println!("---------------------------- TRANSACTION PROCESSED -----------------------------");
        ToolboxEndpointPrinter::print_transaction(transaction);
        println!("----");
        match result {
            Ok(signature) => {
                println!("transaction.result: OK");
                if *signature != Signature::default() {
                    println!("transaction.signature: {:?}", signature)
                }
            },
            Err(error) => {
                println!("transaction.result: FAIL");
                println!("transaction.error: {:?}", error)
            },
        };
        println!("----");
        ToolboxEndpointPrinter::print_backtrace("from");
        println!();
    }

    async fn on_account(
        &self,
        address: &Pubkey,
        account: &Option<Account>,
    ) {
        println!("-------------------------------- ACCOUNT READ ----------------------------------");
        ToolboxEndpointPrinter::print_account(address, account);
        println!("----");
        ToolboxEndpointPrinter::print_backtrace("from");
        println!();
    }
}
