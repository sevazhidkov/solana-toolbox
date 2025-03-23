use std::str::FromStr;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use clap::Args;
use serde_json::json;
use solana_sdk::bs58;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdlBreadcrumbs;
use solana_toolbox_idl::ToolboxIdlTypeFull;
use solana_toolbox_idl::ToolboxIdlTypePrimitive;

use crate::toolbox_cli_config::ToolboxCliConfig;
use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Args)]
#[command(about = "Search addresses of accounts of given program")]
pub struct ToolboxCliCommandAddressesArgs {
    #[arg(help = "The ProgramID pubkey that owns the searched accounts")]
    program_id: String,
    #[arg(help = "Expected exact data size of the searched accounts")]
    data_len: Option<usize>,
    #[arg(
        help = "Expected data slices of the searched accounts, format: [offset:encoding:data]"
    )]
    data_chunks: Vec<String>,
}

impl ToolboxCliCommandAddressesArgs {
    pub async fn process(
        &self,
        config: &ToolboxCliConfig,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = config.create_endpoint().await?;
        let program_id = Pubkey::from_str(&self.program_id).unwrap();
        // TODO - add IDL analysis
        let mut data_chunks = vec![];
        for data_chunk in &self.data_chunks {
            let parts = data_chunk.split(":").collect::<Vec<_>>();
            if let [offset, encoding, data] = parts[..] {
                data_chunks.push((
                    offset.parse::<usize>().unwrap(),
                    parse_blob(encoding, data),
                ));
            } else {
                return Err(ToolboxCliError::Custom(
                    "Invalid data chunk, expected: offset:encoding:data"
                        .to_string(),
                ));
            }
        }
        let mut data_chunks_slices = vec![];
        for data_chunk in &data_chunks {
            data_chunks_slices.push((data_chunk.0, &data_chunk.1[..]));
        }
        let addresses = endpoint
            .search_addresses(&program_id, self.data_len, &data_chunks_slices)
            .await?;
        println!(
            "{}",
            serde_json::to_string(&json!(addresses
                .iter()
                .map(|address| address.to_string())
                .collect::<Vec<_>>()))?
        );
        Ok(())
    }
}

fn parse_blob(encoding: &str, data: &str) -> Vec<u8> {
    if encoding == "base58" {
        bs58::decode(data).into_vec().unwrap()
    } else if encoding == "base64" {
        STANDARD.decode(data).unwrap()
    } else if encoding == "bytes" {
        let mut bytes = vec![];
        ToolboxIdlTypeFull::Vec {
            items: Box::new(ToolboxIdlTypeFull::Primitive {
                primitive: ToolboxIdlTypePrimitive::U8,
            }),
        }
        .try_serialize(
            &serde_json::from_str(data).unwrap(),
            &mut bytes,
            false,
            &ToolboxIdlBreadcrumbs::default(),
        )
        .unwrap();
        bytes
    } else {
        panic!("unknown encoding: {}", encoding);
    }
}
