use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_instruction::ToolboxIdlInstruction;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccountPda;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccountPdaBlob;

impl ToolboxIdlInstruction {
    pub fn get_needs(&self) -> (Value, Map<String, Value>) {
        let instruction_needs_payload = self.args_type_full_fields.explain();
        let mut instruction_needs_addresses = Map::new();
        for account in &self.accounts {
            if let Some(account_address) = &account.address {
                instruction_needs_addresses.insert(
                    account.name.to_string(),
                    json!(account_address.to_string()),
                );
            } else if let Some(account_pda) = &account.pda {
                instruction_needs_addresses.insert(
                    account.name.to_string(),
                    json!(account_pda.get_needs()),
                );
            } else {
                instruction_needs_addresses
                    .insert(account.name.to_string(), json!(null));
            }
        }
        (instruction_needs_payload, instruction_needs_addresses)
    }
}

impl ToolboxIdlInstructionAccountPda {
    pub fn get_needs(&self) -> Vec<Value> {
        let mut needs_blobs = vec![];
        for seed in &self.seeds {
            if let Some((kind, path, explained)) = seed.get_need() {
                needs_blobs.push(json!({
                    "kind": kind,
                    "path": path,
                    "type": explained,
                }));
            }
        }
        if let Some(program) = &self.program {
            if let Some((kind, path, explained)) = program.get_need() {
                needs_blobs.push(json!({
                    "kind": kind,
                    "path": path,
                    "type": explained,
                }));
            }
        }
        needs_blobs
    }
}

impl ToolboxIdlInstructionAccountPdaBlob {
    pub fn get_need(&self) -> Option<(&str, String, Value)> {
        match self {
            ToolboxIdlInstructionAccountPdaBlob::Const { .. } => None,
            ToolboxIdlInstructionAccountPdaBlob::Arg {
                path,
                type_full,
                ..
            } => Some(("arg", path.to_string(), type_full.explain())),
            ToolboxIdlInstructionAccountPdaBlob::Account {
                path,
                type_full,
                ..
            } => Some(("account", path.to_string(), type_full.explain())),
        }
    }
}
