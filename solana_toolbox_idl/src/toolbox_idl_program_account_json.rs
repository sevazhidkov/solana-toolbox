use serde_json::{json, Value};

use crate::ToolboxIdlProgramAccount;

// TODO - this parse/json could be serde serialize/deserialize trait implementations?
impl ToolboxIdlProgramAccount {
    pub fn as_json(&self, backward_compatibility: bool) -> Value {
        if backward_compatibility {
            json!({
                "name": self.name,
                "disriminator": self.discriminator,
                "type": self.data_type_flat.as_json(backward_compatibility),
            })
        } else {
            // TODO - what if discriminator is the default one, we can shortcut ?
            json!({
                "disriminator": self.discriminator,
                "type": self.data_type_flat.as_json(backward_compatibility),
            })
        }
    }
}
