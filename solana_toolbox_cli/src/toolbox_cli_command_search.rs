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
pub struct ToolboxCliCommandSearchArgs {
    #[arg(help = "The ProgramID pubkey that owns the searched accounts")]
    program_id: String,
    #[arg(help = "The max amount of accounts being searched")]
    limit: Option<usize>,
    #[arg(
        long,
        help = "Expected exact data byte size of the searched accounts"
    )]
    space: Option<usize>,
    #[arg(
        long,
        value_delimiter = ',',
        help = "Expected data slices of the searched accounts, format: [offset:encoding:data]"
    )]
    chunks: Vec<String>,
    #[arg(long, help = "Expected account name")]
    name: Option<String>,
}

impl ToolboxCliCommandSearchArgs {
    pub async fn process(
        &self,
        config: &ToolboxCliConfig,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = config.create_endpoint().await?;
        let mut idl_resolver = config.create_resolver().await?;
        let program_id = Pubkey::from_str(&self.program_id).unwrap();
        // TODO - add IDL analysis
        let mut chunks = vec![];
        for chunk in &self.chunks {
            let parts = chunk.split(":").collect::<Vec<_>>();
            if let [offset, encoding, data] = parts[..] {
                chunks.push((
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
        let mut chunks_slices = vec![];
        for chunk in &chunks {
            chunks_slices.push((chunk.0, &chunk.1[..]));
        }
        let addresses = endpoint
            .search_addresses(&program_id, self.space, &chunks_slices)
            .await?;
        let mut json_accounts = vec![];
        for address in addresses {
            if json_accounts.len() >= self.limit.unwrap_or(10) {
                break;
            }
            let account =
                endpoint.get_account(&address).await?.unwrap_or_default();
            let idl_program = idl_resolver
                .resolve_program(&mut endpoint, &account.owner)
                .await?
                .unwrap_or_default();
            let idl_account =
                idl_program.guess_account(&account.data).unwrap_or_default();
            if let Some(name) = &self.name {
                if &idl_account.name != name {
                    continue;
                }
            }
            let account_state = idl_account.decompile(&account.data)?;
            json_accounts.push(json!({
                "address": address.to_string(),
                "kind": format!(
                    "{}.{}",
                    idl_program.name.clone().unwrap_or(account.owner.to_string()),
                    idl_account.name,
                ),
                "state": account_state,
            }));
        }
        println!("{}", serde_json::to_string(&json!(json_accounts))?);
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
