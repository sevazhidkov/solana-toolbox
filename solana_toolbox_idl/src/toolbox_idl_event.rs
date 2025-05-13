use anyhow::anyhow;
use anyhow::Result;
use serde_json::Value;

use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlEvent {
    pub name: String,
    pub docs: Option<Value>,
    pub discriminator: Vec<u8>,
    pub info_type_flat: ToolboxIdlTypeFlat,
    pub info_type_full: ToolboxIdlTypeFull,
}

impl Default for ToolboxIdlEvent {
    fn default() -> ToolboxIdlEvent {
        ToolboxIdlEvent {
            name: "Unknown".to_string(),
            docs: None,
            discriminator: vec![],
            info_type_flat: ToolboxIdlTypeFlat::nothing(),
            info_type_full: ToolboxIdlTypeFull::nothing(),
        }
    }
}

impl ToolboxIdlEvent {
    pub fn encode(&self, event_message: &Value) -> Result<Vec<u8>> {
        let mut event_data = vec![];
        event_data.extend_from_slice(&self.discriminator);
        self.info_type_full.try_serialize(
            event_message,
            &mut event_data,
            true,
        )?;
        Ok(event_data)
    }

    pub fn decode(&self, event_data: &[u8]) -> Result<Value> {
        self.check(event_data)?;
        let (_, event_value) = self
            .info_type_full
            .try_deserialize(event_data, self.discriminator.len())?;
        Ok(event_value)
    }

    pub fn check(&self, event_data: &[u8]) -> Result<()> {
        if !event_data.starts_with(&self.discriminator) {
            return Err(anyhow!(
                "Invalid event discriminator, expected: {:?}, found: {:?}",
                self.discriminator,
                event_data,
            ));
        }
        Ok(())
    }
}
