use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccount;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccountPda;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccountPdaBlob;

impl ToolboxIdlInstructionAccount {
    pub fn export(&self, backward_compatibility: bool) -> Value {
        let mut json_object = Map::new();
        json_object.insert("name".to_string(), json!(self.name));
        // TODO - support multiple anchor versions format
        if backward_compatibility {
            if self.signer {
                json_object.insert("isSigner".to_string(), json!(true));
            }
            if self.writable {
                json_object.insert("isMut".to_string(), json!(true));
            }
        } else {
            if self.signer {
                json_object.insert("signer".to_string(), json!(true));
            }
            if self.writable {
                json_object.insert("writable".to_string(), json!(true));
            }
        }
        if let Some(address) = &self.address {
            json_object
                .insert("address".to_string(), json!(address.to_string()));
        }
        if let Some(pda) = &self.pda {
            json_object
                .insert("pda".to_string(), pda.export(backward_compatibility));
        }
        json!(json_object)
    }
}

impl ToolboxIdlInstructionAccountPda {
    pub fn export(&self, backward_compatibility: bool) -> Value {
        let mut json_object = Map::new();
        json_object.insert(
            "seeds".to_string(),
            json!(self
                .seeds
                .iter()
                .map(|blob| blob.export(backward_compatibility))
                .collect::<Vec<_>>()),
        );
        if let Some(program) = &self.program {
            json_object.insert(
                "program".to_string(),
                program.export(backward_compatibility),
            );
        }
        json!(json_object)
    }
}

impl ToolboxIdlInstructionAccountPdaBlob {
    pub fn export(&self, _backward_compatibility: bool) -> Value {
        // TODO - support backward compatibility ?
        match self {
            ToolboxIdlInstructionAccountPdaBlob::Const { bytes } => json!({
                "kind": "const",
                "value": bytes,
            }),
            ToolboxIdlInstructionAccountPdaBlob::Arg { path } => json!({
                "kind": "arg",
                "path": path,
            }),
            ToolboxIdlInstructionAccountPdaBlob::Account { path } => json!({
                "kind": "account",
                "path": path,
            }),
        }
    }
}
