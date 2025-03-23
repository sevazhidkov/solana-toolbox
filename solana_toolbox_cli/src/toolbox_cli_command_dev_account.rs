use std::str::FromStr;

use clap::Args;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_cli_config::ToolboxCliConfig;
use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Args)]
#[command(about = "Dump an account's information for development purposes")]
pub struct ToolboxCliCommandDevAccountArgs {
    #[arg(help = "The account's pubkey")]
    address: String,
}

impl ToolboxCliCommandDevAccountArgs {
    pub async fn process(
        &self,
        config: &ToolboxCliConfig,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = config.create_endpoint()?;
        let address = Pubkey::from_str(&self.address)?;
        let account = endpoint.get_account_or_default(&address).await?;
        println!("+{:-^78}+", "Addr");
        println!("|{: ^78}|", address.to_string());
        println!("+{:-^78}+", "Meta");
        println!(
            "|{: ^78}|",
            format!(
                "data: {} byte(s), balance: {} SOL, executable: {}",
                account.data.len(),
                (account.lamports as f64) / 1_000_000_000.0,
                account.executable,
            )
        );
        // TODO - nits on display of hexes
        println!("|{: ^78}|", format!("owner: {}", account.owner));
        if !account.data.is_empty() {
            println!("+{:-^78}+", "Data");
            println!(
                "| {: <7} | {: <47} | {: <16} | ",
                "Index", "Data (Hexadecimal)", "Data (Ascii)"
            );
            let data = account.data;
            let data_len = data.len();
            let data_packing = 16;
            let data_lines = data_len.div_ceil(data_packing);
            for data_line in 0..data_lines {
                let data_start = data_line * data_packing;
                let mut data_hexes = vec![];
                let mut data_ascii = vec![];
                for data_offset in 0..data_packing {
                    let data_index = data_start + data_offset;
                    if data_index < data_len {
                        let data_byte = data[data_index];
                        let data_char = data_byte as char;
                        data_hexes.push(format!("{:02X}", data_byte));
                        if data_char.is_ascii_alphanumeric()
                            || data_char.is_ascii_punctuation()
                        {
                            data_ascii.push(data_char.to_string());
                        } else {
                            data_ascii.push(".".to_string())
                        }
                    } else {
                        data_hexes.push("  ".to_string());
                        data_ascii.push(" ".to_string());
                    }
                }
                println!(
                    "| {: >7} | {} | {} |",
                    data_start,
                    data_hexes.join(" "),
                    data_ascii.concat()
                );
            }
        }
        println!("+{:-^78}+", "-");
        Ok(())
    }
}
