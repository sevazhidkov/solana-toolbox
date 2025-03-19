use serde_json::json;

use crate::toolbox_idl_program::ToolboxIdlProgram;

pub fn idl_lib_native_compute_budget() -> ToolboxIdlProgram {
    ToolboxIdlProgram::try_parse_from_value(&json!({
        "name": "compute_budget",
        "instructions": {
            "RequestHeapFrame": {
                "discriminator": [1],
                "args": ["u32"],
                "accounts": [],
            },
            "SetComputeUnitLimit": {
                "discriminator": [2],
                "args": ["u32"],
                "accounts": [],
            },
            "SetComputeUnitPrice": {
                "discriminator": [3],
                "args": ["u64"],
                "accounts": [],
            },
        },
        "accounts": {
            "Account": {
                "discriminator": [],
                "fields": []
            },
        },
        "types": {},
        "errors": {},
    }))
    .unwrap()
}
