use serde_json::json;

use crate::toolbox_idl_program::ToolboxIdlProgram;

pub fn idl_lib_spl_associated_token() -> ToolboxIdlProgram {
    ToolboxIdlProgram::try_parse_from_value(&json!({
        "name": "spl_associated_token",
        "instructions": {
            "CreateAssociatedTokenAccount": {
                "discriminator": [],
                "args": [],
                "accounts": [
                    {"name": "payer"},
                    {"name": "ata"},
                    {"name": "wallet"},
                    {"name": "mint"},
                    {"name": "system_program"},
                    {"name": "token_program"},
                ],
            }
        },
        "accounts": {},
        "types": {},
        "errors": {},
    }))
    .unwrap()
}
