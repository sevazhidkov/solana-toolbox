use serde_json::json;
use serde_json::Value;

use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccount;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccountPda;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccountPdaBlob;

impl ToolboxIdlInstructionAccount {
    pub fn export(&self, backward_compatibility: bool) -> Value {
        let signer = if self.signer { Some(true) } else { None };
        let writable = if self.writable { Some(true) } else { None };
        let address = self.address.map(|address| address.to_string());
        let pda = self
            .pda
            .as_ref()
            .map(|pda| pda.export(backward_compatibility));
        // TODO - support multiple anchor versions format
        if backward_compatibility {
            json!({
                "name": self.name,
                "isSigner": signer,
                "isMut": writable,
                "address": address,
                "pda": pda,
            })
        } else {
            json!({
                "name": self.name,
                "signer": signer,
                "writable": writable,
                "address": address,
                "pda": pda,
            })
        }
    }
}

impl ToolboxIdlInstructionAccountPda {
    pub fn export(&self, backward_compatibility: bool) -> Value {
        json!({
            "seeds": self.seeds.iter().map(|blob| blob.export(backward_compatibility)).collect::<Vec<_>>(),
            "program": self.program.as_ref().map(|blob| blob.export(backward_compatibility)),
        })
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
