use std::str::FromStr;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use clap::Args;
use serde_json::json;
use solana_cli_config::Config;
use solana_sdk::bs58;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdlBreadcrumbs;
use solana_toolbox_idl::ToolboxIdlTypeFull;
use solana_toolbox_idl::ToolboxIdlTypePrimitive;

use crate::toolbox_cli_error::ToolboxCliError;
use crate::toolbox_cli_utils::ToolboxCliUtils;

#[derive(Debug, Clone, Args)]
pub struct ToolboxCliCommandRawSearchAddressesArgs {
    program_id: String,
    data_len: Option<usize>,
    data_chunks: Vec<String>,
}

impl ToolboxCliCommandRawSearchAddressesArgs {
    pub async fn process(
        &self,
        config: &Config,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = ToolboxCliUtils::new_endpoint(config)?;
        let program_id = Pubkey::from_str(&self.program_id).unwrap();
        let mut data_chunks = vec![];
        for data_chunk in &self.data_chunks {
            let parts = data_chunk.split(":").collect::<Vec<_>>();
            if let [offset, encoding, blob] = parts[..] {
                data_chunks.push((
                    offset.parse::<usize>(),
                    parse_blob(encoding, blob),
                ));
            } else {
                return Err(ToolboxCliError::Custom(
                    "Invalid data chunk, expected: offset:encoding:data"
                        .to_string(),
                ));
            }
        }
        let addresses = endpoint
            .search_addresses(&program_id, self.data_len, &[])
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
        ToolboxIdlTypeFull::Primitive {
            primitive: ToolboxIdlTypePrimitive::Bytes,
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
