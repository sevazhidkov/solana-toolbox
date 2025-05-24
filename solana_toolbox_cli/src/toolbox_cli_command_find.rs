use anyhow::anyhow;
use anyhow::Result;
use clap::Args;
use serde_json::json;
use serde_json::Value;
use solana_toolbox_idl::ToolboxIdlPath;
use solana_toolbox_idl::ToolboxIdlTypeFull;
use solana_toolbox_idl::ToolboxIdlTypePrefix;
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
        display_order = 11,
        long = "limit",
        value_name = "COUNT",
        help = "The max amount of accounts being searched (to avoid rate limiting)"
    )]
    limit: Option<usize>,
    #[arg(
        display_order = 12,
        long = "space",
        value_name = "LENGTH",
        help = "Expect exact data byte size of the searched accounts"
    )]
    space: Option<usize>,
    #[arg(
        display_order = 13,
        long = "chunk",
        alias = "chunks",
        value_name = "OFFSET:JSON_BYTES",
        help = "Expect data slices of the searched accounts"
    )]
    chunks: Vec<String>,
    #[arg(
        display_order = 14,
        long = "name",
        value_name = "ACCOUNT_NAME",
        help = "Expect parsed IDL account name"
    )]
    name: Option<String>,
    #[arg(
        display_order = 15,
        long = "state-fit",
        alias = "states-fits",
        value_name = "JSON_VALUE",
        help = "Expect account state to fit this value (to be a superset of it)"
    )]
    states_fits: Vec<String>,
    #[arg(
        display_order = 16,
        long = "state-path",
        alias = "states-pathses",
        value_name = "JSON_PATH",
        help = "Expect account state to contain a value at this path"
    )]
    states_paths: Vec<String>,
}

impl ToolboxCliCommandFindArgs {
    // TODO - should add a last-modified field and sorting ?
    pub async fn process(&self, context: &ToolboxCliContext) -> Result<Value> {
        let mut endpoint = context.create_endpoint().await?;
        let mut idl_service = context.create_service().await?;
        let program_id = context.parse_key(&self.program_id)?.address();
        let mut chunks = vec![];
        for chunk in &self.chunks {
            if let Some((offset, encoded)) = chunk.split_once(":") {
                let mut bytes = vec![];
                ToolboxIdlTypeFull::Vec {
                    prefix: ToolboxIdlTypePrefix::U32,
                    items: Box::new(ToolboxIdlTypePrimitive::U8.into()),
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
            let account =
                endpoint.get_account(&address).await?.unwrap_or_default();
            let idl_program = idl_service
                .load_program(&mut endpoint, &account.owner)
                .await?
                .unwrap_or_default();
            let idl_account =
                idl_program.guess_account(&account.data).unwrap_or_default();
            let account_name =
                context.compute_account_name(&idl_program, &idl_account);
            if let Some(name) = &self.name {
                if !account_name.contains(name) {
                    continue;
                }
            }
            let account_state = match idl_account.decode(&account.data) {
                Ok(account_state) => account_state,
                Err(error) => json!({
                    "decode_error": context.compute_error_json(error), // TODO - better error handling
                }),
            };
            let mut ignored = false;
            for state_fit in &self.states_fits {
                if !cli_json_value_fit(
                    &account_state,
                    &context.parse_hjson(state_fit)?,
                ) {
                    ignored = true;
                }
            }
            let account_state = if self.states_paths.is_empty() {
                account_state
            } else {
                let mut account_state_filtered = json!({});
                for state_path in &self.states_paths {
                    let path = ToolboxIdlPath::try_parse(state_path)?;
                    account_state_filtered =
                        match path.try_get_json_value(&account_state) {
                            Ok(value) => path.try_set_json_value(
                                Some(account_state_filtered),
                                value.clone(),
                            )?,
                            Err(_) => {
                                ignored = true;
                                continue;
                            },
                        };
                }
                account_state_filtered
            };
            if !ignored {
                json_accounts.push(json!({
                    "address": address.to_string(),
                    "name": account_name,
                    "state": account_state,
                    "explorer": context.compute_explorer_address_link(&address)
                }));
            }
        }
        Ok(json!(json_accounts))
    }
}
