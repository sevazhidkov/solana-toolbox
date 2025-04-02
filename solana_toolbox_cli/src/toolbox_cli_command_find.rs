use clap::Args;
use serde_json::json;
use serde_json::Value;
use solana_toolbox_idl::ToolboxIdlBreadcrumbs;
use solana_toolbox_idl::ToolboxIdlTypeFull;
use solana_toolbox_idl::ToolboxIdlTypePrimitive;

use crate::toolbox_cli_context::ToolboxCliContext;
use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Args)]
#[command(about = "Search addresses of accounts of given program")]
pub struct ToolboxCliCommandFindArgs {
    #[arg(
        value_name = "PROGRAM_ID",
        help = "The ProgramID pubkey that owns the searched accounts"
    )]
    program_id: String,
    #[arg(
        long,
        value_name = "COUNT",
        help = "The max amount of accounts being searched (to avoid rate limiting)"
    )]
    limit: Option<usize>,
    #[arg(
        long,
        value_name = "LENGTH",
        help = "Expect exact data byte size of the searched accounts"
    )]
    space: Option<usize>,
    #[arg(
        long = "chunk",
        value_name = "OFFSET:JSON_BYTES",
        help = "Expect data slices of the searched accounts"
    )]
    chunks: Vec<String>,
    #[arg(
        long,
        value_name = "ACCOUNT_NAME",
        help = "Expect parsed IDL account name"
    )]
    name: Option<String>,
    #[arg(
        long = "state",
        value_name = "JSON_VALUE",
        help = "Expect account state to match this value"
    )]
    states: Vec<String>,
}

impl ToolboxCliCommandFindArgs {
    pub async fn process(
        &self,
        context: &ToolboxCliContext,
    ) -> Result<Value, ToolboxCliError> {
        let mut endpoint = context.create_endpoint().await?;
        let mut idl_service = context.create_service().await?;
        let program_id = context.parse_key(&self.program_id)?.address();
        let mut chunks = vec![];
        for chunk in &self.chunks {
            if let Some((offset, encoded)) = chunk.split_once(":") {
                let mut bytes = vec![];
                ToolboxIdlTypeFull::Vec {
                    items: Box::new(ToolboxIdlTypeFull::Primitive {
                        primitive: ToolboxIdlTypePrimitive::U8,
                    }),
                }
                .try_serialize(
                    &serde_hjson::from_str(encoded).unwrap(),
                    &mut bytes,
                    false,
                    &ToolboxIdlBreadcrumbs::default(),
                )
                .unwrap();
                chunks.push((offset.parse::<usize>().unwrap(), bytes));
            } else {
                return Err(ToolboxCliError::Custom(
                    "Invalid data chunk, expected: offset:bytes".to_string(),
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
                let expected_state = context.parse_hjson(&state)?;
                if !json_match(&account_decoded.state, &expected_state) {
                    continue;
                }
            }
            json_accounts.push(json!({
                "address": address.to_string(),
                "owner": &account_decoded.owner,
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

// TODO (MEDIUM) - where should this go ?
fn json_match(found: &Value, expected: &Value) -> bool {
    match expected {
        Value::Null => {
            if let Some(()) = found.as_null() {
                return true;
            }
            false
        },
        Value::Bool(expected) => {
            if let Some(found) = found.as_bool() {
                return found == *expected;
            }
            false
        },
        Value::Number(expected) => {
            if let Some(found) = found.as_number() {
                return found == expected;
            }
            false
        },
        Value::String(expected) => {
            if let Some(found) = found.as_str() {
                return found == expected;
            }
            false
        },
        Value::Array(expected) => {
            if let Some(found) = found.as_array() {
                if found.len() < expected.len() {
                    return false;
                }
                for (idx, expected) in expected.iter().enumerate() {
                    if !json_match(&found[idx], expected) {
                        return false;
                    }
                }
                return true;
            }
            false
        },
        Value::Object(expected) => {
            if let Some(found) = found.as_object() {
                for (key, expected) in expected {
                    if let Some(found) = found.get(key) {
                        if !json_match(found, expected) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                return true;
            }
            false
        },
    }
}
