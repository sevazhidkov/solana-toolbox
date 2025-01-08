use solana_sdk::account::Account;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;

use crate::toolbox_endpoint_logger::ToolboxEndpointLoggerTransaction;
use crate::ToolboxEndpointError;
use crate::ToolboxEndpointLogger;

pub struct ToolboxEndpointLoggerPrint {}

impl ToolboxEndpointLoggerPrint {
    pub fn new() -> ToolboxEndpointLoggerPrint {
        ToolboxEndpointLoggerPrint {}
    }
}

impl ToolboxEndpointLogger for ToolboxEndpointLoggerPrint {
    fn on_transaction(
        &self,
        transaction: &ToolboxEndpointLoggerTransaction,
        result: &Result<Signature, ToolboxEndpointError>,
    ) {
        println!("-------- PROCESSED TRANSACTION --------");
        println!("transaction.payer: {:?}", transaction.payer);
        for instruction in &transaction.instructions {
            println!(" > instruction");
            println!(" > instruction.program_id: {:?}", instruction.program_id);
            let mut index = 1;
            for account in &instruction.accounts {
                println!(" > instruction.account: #{:?} {:?}", index, account);
                index += 1;
            }
            println!(" > instruction.data: {:?}", instruction.data);
        }
        match result {
            Ok(signature) => {
                println!("transaction.result: OK");
                println!("transaction.signature: {:?}", signature)
            },
            Err(error) => {
                println!("transaction.result: FAIL");
                println!("transaction.error: {:?}", error)
            },
        };
        self.print_backtrace();
        println!("");
    }

    fn on_account(
        &self,
        address: &Pubkey,
        account: &Option<Account>,
    ) {
        println!("-------- READ ACCOUNT --------");
        println!("address: {:?}", address);
        let account = account.clone().unwrap_or_default();
        println!(" > account.lamports: {:?}", account.lamports);
        println!(" > account.owner: {:?}", account.owner);
        println!(" > account.data: {:?}", account.data);
        println!(" > account.executable: {:?}", account.executable);
        self.print_backtrace();
        println!("");
    }
}

impl ToolboxEndpointLoggerPrint {
    fn print_backtrace(&self) {
        let backtrace_data = std::backtrace::Backtrace::force_capture();
        let backtrace_formatted = std::format!("{}", backtrace_data);
        let backtrace_lines = backtrace_formatted.lines();
        for backtrace_line in backtrace_lines {
            if backtrace_line.contains("at ./") {
                println!("from: {}", backtrace_line.trim());
            }
        }
    }
}
