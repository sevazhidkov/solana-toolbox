use solana_sdk::account::Account;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;

#[derive(Default)]
pub struct ToolboxEndpointPrinter {}

impl ToolboxEndpointPrinter {
    pub fn print_transaction(
        payer: &Pubkey,
        instructions: &[Instruction],
    ) {
        println!("----");
        println!("transaction.payer: {:?}", payer);
        for instruction in instructions {
            ToolboxEndpointPrinter::print_instruction(instruction);
        }
    }

    pub fn print_instruction(instruction: &Instruction) {
        println!("----");
        println!("> instruction.program_id: {:?}", instruction.program_id);
        for account_index in 0..instruction.accounts.len() {
            let account_meta = &instruction.accounts[account_index];
            println!(
                "> instruction.accounts: #{:03}: ({}{}) {}",
                account_index + 1,
                if account_meta.is_writable { "W" } else { "R" },
                if account_meta.is_signer { "S" } else { "-" },
                account_meta.pubkey,
            );
        }
        ToolboxEndpointPrinter::print_data(
            "> instruction.data",
            &instruction.data,
        );
    }

    pub fn print_account(
        address: &Pubkey,
        account: &Option<Account>,
    ) {
        println!("----");
        println!("address.key: {:?}", address);
        println!("----");
        let account = account.clone().unwrap_or_default();
        println!("> account.lamports: {:?}", account.lamports);
        println!("> account.owner: {:?}", account.owner);
        println!("> account.executable: {:?}", account.executable);
        ToolboxEndpointPrinter::print_data("> account.data", &account.data);
    }

    pub fn print_data(
        prefix: &str,
        data: &[u8],
    ) {
        let data_len = data.len();
        println!("{}.len: {:?} bytes", prefix, data_len);
        let data_packing = 16;
        let data_spacing = 1;
        let data_lines = data_len.div_ceil(data_packing);
        for data_line in 0..data_lines {
            let data_start = data_line * data_packing;
            let mut data_chunks = vec![];
            for data_offset in 0..data_packing {
                if data_offset % data_spacing == 0 {
                    data_chunks.push(" ".to_string());
                }
                let data_index = data_start + data_offset;
                if data_index < data_len {
                    data_chunks.push(format!("{:02X}", data[data_index]));
                } else {
                    data_chunks.push("..".to_string());
                }
            }
            let data_decompiled = data_chunks.concat();
            println!("{}: #{:08}:{}", prefix, data_start, data_decompiled);
        }
    }

    pub fn print_backtrace(prefix: &str) {
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
