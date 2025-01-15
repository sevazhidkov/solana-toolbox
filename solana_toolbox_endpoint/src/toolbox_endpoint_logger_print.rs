use solana_sdk::account::Account;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;

use crate::toolbox_endpoint_error::ToolboxEndpointError;
use crate::toolbox_endpoint_logger::ToolboxEndpointLogger;
use crate::toolbox_endpoint_logger::ToolboxEndpointLoggerTransaction;

#[derive(Default)]
pub struct ToolboxEndpointLoggerPrint {}

#[async_trait::async_trait]
impl ToolboxEndpointLogger for ToolboxEndpointLoggerPrint {
    async fn on_transaction(
        &self,
        transaction: &ToolboxEndpointLoggerTransaction,
        result: &Result<Signature, ToolboxEndpointError>,
    ) {
        println!("---------------------------- TRANSACTION PROCESSED -----------------------------");
        println!("----");
        println!("transaction.payer: {:?}", transaction.payer);
        println!("----");
        for signer_index in 0..transaction.signers.len() {
            println!(
                "transaction.signers: #{:?}: {:?}",
                signer_index + 1,
                transaction.signers[signer_index]
            );
        }
        for instruction in &transaction.instructions {
            println!("----");
            println!("> instruction.program_id: {:?}", instruction.program_id);
            for account_index in 0..instruction.accounts.len() {
                println!(
                    "> instruction.accounts: #{:03?}: {:?}",
                    account_index + 1,
                    instruction.accounts[account_index]
                );
            }
            self.print_bytes(
                "> instruction.data".to_string(),
                &instruction.data,
            );
        }
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
        self.print_backtrace("from".to_string());
        println!();
    }

    async fn on_account(
        &self,
        address: &Pubkey,
        account: &Option<Account>,
    ) {
        println!("--------------------------------- READ ACCOUNT ---------------------------------");
        println!("----");
        println!("address.key: {:?}", address);
        println!("----");
        let account = account.clone().unwrap_or_default();
        println!("> account.lamports: {:?}", account.lamports);
        println!("> account.owner: {:?}", account.owner);
        println!("> account.executable: {:?}", account.executable);
        self.print_bytes("> account.data".to_string(), &account.data);
        println!("----");
        self.print_backtrace("from".to_string());
        println!();
    }
}

impl ToolboxEndpointLoggerPrint {
    fn print_bytes(
        &self,
        prefix: String,
        data: &[u8],
    ) {
        let data_len = data.len();
        println!("{}.len: {:?} bytes", prefix, data_len);
        let data_packing = 16;
        let data_lines = data_len.div_ceil(data_packing);
        for data_line in 0..data_lines {
            let data_start = data_line * data_packing;
            let mut data_bytes = vec![];
            for data_offset in 0..data_packing {
                let data_index = data_start + data_offset;
                if data_index < data_len {
                    if data_offset == 8 {
                        data_bytes.push("".to_string());
                    }
                    data_bytes.push(format!(
                        "{:02X}",
                        data[data_start + data_offset]
                    ));
                }
            }
            println!(
                "{}: #{:08}: {}",
                prefix,
                data_start,
                data_bytes.join(" "),
            );
        }
    }

    fn print_backtrace(
        &self,
        prefix: String,
    ) {
        let backtrace_data = std::backtrace::Backtrace::force_capture();
        let backtrace_formatted = std::format!("{}", backtrace_data);
        let backtrace_lines = backtrace_formatted.lines();
        for backtrace_line in backtrace_lines {
            let backtrace_line_trimmed = backtrace_line.trim();
            if backtrace_line_trimmed.starts_with("at ./") {
                println!("{}: &{}", prefix, backtrace_line_trimmed);
            }
        }
    }
}
