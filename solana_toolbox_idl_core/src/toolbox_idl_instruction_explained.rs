use crate::toolbox_idl_encoding as encoding;
use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_instruction::ToolboxIdlInstruction;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccountPda;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccountPdaBlob;

impl ToolboxIdlInstruction {
    pub fn explained(&self) -> (Value, Map<String, Value>) {
        let json_payload = self.args_type_full_fields.explained();
        let mut json_addresses = Map::new();
        for account in &self.accounts {
            if let Some(account_address) = &account.address {
                json_addresses.insert(
                    account.name.to_string(),
                    json!(account_address.to_string()),
                );
            } else if let Some(account_pda) = &account.pda {
                json_addresses.insert(
                    account.name.to_string(),
                    json!(account_pda.explained()),
                );
            } else {
                json_addresses.insert(account.name.to_string(), json!(null));
            }
        }
        (json_payload, json_addresses)
    }
}

impl ToolboxIdlInstructionAccountPda {
    pub fn explained(&self) -> Value {
        let mut json_seeds = vec![];
        for seed in &self.seeds {
            json_seeds.push(seed.explained());
        }
        let mut json_pda = Map::new();
        json_pda.insert("seeds".to_string(), json!(json_seeds));
        if let Some(program) = &self.program {
            json_pda.insert("program".to_string(), program.explained());
        }
        json!(json_pda)
    }
}

impl ToolboxIdlInstructionAccountPdaBlob {
    pub fn explained(&self) -> Value {
        match self {
            ToolboxIdlInstructionAccountPdaBlob::Const {
                value,
                type_full,
                ..
            } => {
                let mut data = vec![];
                match type_full.try_serialize(value, &mut data, false) {
                    Ok(()) => json!({
                        "const": {
                            "base16": encoding::encode_base16(&data),
                            "base58": encoding::encode_base58(&data),
                            "base64": encoding::encode_base64(&data),
                            "utf8_lossy": String::from_utf8_lossy(&data)
                        }
                    }),
                    Err(error) => json!({
                        "const": {
                            "error": error.to_string()
                        }
                    }),
                }
            },
            ToolboxIdlInstructionAccountPdaBlob::Arg { path, .. } => {
                json!({ "arg": path.value() })
            },
            ToolboxIdlInstructionAccountPdaBlob::Account { path, .. } => {
                json!({ "account": path.value() })
            },
        }
    }
}
