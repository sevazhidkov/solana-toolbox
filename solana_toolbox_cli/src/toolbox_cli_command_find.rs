use anyhow::anyhow;
use anyhow::Result;
use clap::Args;
use serde_json::json;
use serde_json::Value;
use solana_toolbox_idl::ToolboxIdlTypeFull;
use solana_toolbox_idl::ToolboxIdlTypePrimitive;

use crate::toolbox_cli_context::ToolboxCliContext;
use crate::toolbox_cli_json::cli_json_value_fit;

#[derive(Debug, Clone, Args)]
#[command(about = "Search addresses of accounts of given program")]
pub struct ToolboxCliCommandFindArgs {
    #[arg(
        value_name = "PROGRAM_ID",
        help = "The ProgramID pubkey that owns the searched accounts"
    )]
    program_id: String,
    #[arg(
        long = "limit",
        value_name = "COUNT",
        help = "The max amount of accounts being searched (to avoid rate limiting)"
    )]
    limit: Option<usize>,
    #[arg(
        long = "space",
        value_name = "LENGTH",
        help = "Expect exact data byte size of the searched accounts"
    )]
    space: Option<usize>,
    #[arg(
        long = "chunk",
        alias = "chunks",
        value_name = "OFFSET:JSON_BYTES",
        help = "Expect data slices of the searched accounts"
    )]
    chunks: Vec<String>,
    #[arg(
        long = "name",
        value_name = "ACCOUNT_NAME",
        help = "Expect parsed IDL account name"
    )]
    name: Option<String>,
    #[arg(
        long = "state",
        alias = "states",
        value_name = "JSON_VALUE",
        help = "Expect account state to match this value"
    )]
    states: Vec<String>,
}

impl ToolboxCliCommandFindArgs {
    pub async fn process(&self, context: &ToolboxCliContext) -> Result<Value> {
        let mut endpoint = context.create_endpoint().await?;
        let mut idl_service = context.create_service().await?;
        let program_id = context.parse_key(&self.program_id)?.address();
        let mut chunks = vec![];
        for chunk in &self.chunks {
            if let Some((offset, encoded)) = chunk.split_once(":") {
                let mut bytes = vec![];
                ToolboxIdlTypeFull::Vec {
                    prefix_bytes: 4,
                    items: Box::new(ToolboxIdlTypeFull::Primitive {
                        primitive: ToolboxIdlTypePrimitive::U8,
                    }),
                }
                .try_serialize(
                    &serde_hjson::from_str(encoded)?,
                    &mut bytes,
                    false,
                )?;
                chunks.push((offset.parse::<usize>()?, bytes));
            } else {
                return Err(anyhow!(
                    "Invalid data chunk, expected: offset:bytes",
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
            if json_accounts.len() >= self.limit.unwrap_or(5) {
                break;
            }
            let account_decoded = idl_service
                .get_and_decode_account(&mut endpoint, &address)
                .await?;
            if let Some(name) = &self.name {
                if &account_decoded.account.name != name {
                    continue;
                }
            }
            for state in &self.states {
                if !cli_json_value_fit(
                    &account_decoded.state,
                    &context.parse_hjson(&state)?,
                ) {
                    continue;
                }
            }
            json_accounts.push(json!({
                "address": address.to_string(),
                "name": context.compute_account_name(
                    &account_decoded.program,
                    &account_decoded.account,
                ),
                "state": account_decoded.state,
                "explorer": context.compute_explorer_address_link(&address)
            }));
        }
        Ok(json!(json_accounts))
    }
}
