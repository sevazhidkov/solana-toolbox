use anyhow::anyhow;
use anyhow::Result;
use serde_json::Value;

use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_utils::idl_convert_to_type_name;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlAccount {
    pub name: String,
    pub docs: Option<Value>,
    pub space: Option<usize>,
    // TODO (FAR) - support discrimination by data chunks (for token account 2022 for example)
    pub discriminator: Vec<u8>,
    pub content_type_flat: ToolboxIdlTypeFlat,
    pub content_type_full: ToolboxIdlTypeFull,
}

impl Default for ToolboxIdlAccount {
    fn default() -> ToolboxIdlAccount {
        ToolboxIdlAccount {
            name: ToolboxIdlAccount::sanitize_name("UnknownAccount"),
            docs: None,
            space: None,
            discriminator: vec![],
            content_type_flat: ToolboxIdlTypeFlat::nothing(),
            content_type_full: ToolboxIdlTypeFull::nothing(),
        }
    }
}

impl ToolboxIdlAccount {
    pub fn sanitize_name(name: &str) -> String {
        idl_convert_to_type_name(name)
    }

    pub fn encode(&self, account_state: &Value) -> Result<Vec<u8>> {
        let mut account_data = vec![];
        account_data.extend_from_slice(&self.discriminator);
        self.content_type_full.try_serialize(
            account_state,
            &mut account_data,
            true,
        )?;
        Ok(account_data)
    }

    pub fn decode(&self, account_data: &[u8]) -> Result<Value> {
        if !account_data.starts_with(&self.discriminator) {
            return Err(anyhow!(
                "Invalid account discriminator, expected: {:?}, found: {:?}",
                self.discriminator,
                account_data,
            ));
        }
        if let Some(space) = self.space {
            if account_data.len() != space {
                return Err(anyhow!(
                    "Invalid account size, expected: {}, found: {}",
                    space,
                    account_data.len(),
                ));
            }
        }
        let (_, account_value) = self
            .content_type_full
            .try_deserialize(account_data, self.discriminator.len())?;
        Ok(account_value)
    }
}
