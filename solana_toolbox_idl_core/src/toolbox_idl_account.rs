use anyhow::anyhow;
use anyhow::Result;
use serde_json::Value;

use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlAccount {
    pub name: String,
    pub docs: Option<Value>,
    pub space: Option<usize>,
    pub blobs: Vec<(usize, Vec<u8>)>,
    pub discriminator: Vec<u8>,
    pub content_type_flat: ToolboxIdlTypeFlat,
    pub content_type_full: ToolboxIdlTypeFull,
}

impl Default for ToolboxIdlAccount {
    fn default() -> ToolboxIdlAccount {
        ToolboxIdlAccount {
            name: "Unknown".to_string(),
            docs: None,
            space: None,
            blobs: vec![],
            discriminator: vec![],
            content_type_flat: ToolboxIdlTypeFlat::nothing(),
            content_type_full: ToolboxIdlTypeFull::nothing(),
        }
    }
}

impl ToolboxIdlAccount {
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
        self.check(account_data)?;
        let (_, account_value) = self
            .content_type_full
            .try_deserialize(account_data, self.discriminator.len())?;
        Ok(account_value)
    }

    pub fn check(&self, account_data: &[u8]) -> Result<()> {
        if let Some(space) = self.space {
            if account_data.len() != space {
                return Err(anyhow!(
                    "Invalid account space, expected: {}, found: {}",
                    space,
                    account_data.len(),
                ));
            }
        }
        for blob in &self.blobs {
            let bytes_expected = &blob.1;
            let offset = blob.0;
            let length = bytes_expected.len();
            let end = offset + length;
            let space = account_data.len();
            if space < end {
                return Err(anyhow!(
                    "Invalid account space for blob, expected at least: {}, found: {}",
                    end,
                    space,
                ));
            }
            let bytes_found = &account_data[offset..end];
            if bytes_found != bytes_expected {
                return Err(anyhow!(
                    "Invalid account blob, at offset: {}, expected: {:?}, found: {:?}",
                    offset,
                    bytes_expected,
                    bytes_found,
                ));
            }
        }
        if !account_data.starts_with(&self.discriminator) {
            return Err(anyhow!(
                "Invalid account discriminator, expected: {:?}, found: {:?}",
                self.discriminator,
                account_data,
            ));
        }
        Ok(())
    }
}
