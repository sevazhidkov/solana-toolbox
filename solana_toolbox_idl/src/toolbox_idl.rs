use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::{ToolboxIdlError, ToolboxIdlProgramRoot};

pub struct ToolboxIdl {}

impl ToolboxIdl {
    pub fn get_account_state(
        &self,
        address: &Pubkey,
        idl: Option<ToolboxIdlProgramRoot>,
    ) -> Result<Option<Value>, ToolboxIdlError> {
        Ok(None) // TODO - implement helpers
    }
}
