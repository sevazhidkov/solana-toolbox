use serde_json::json;

use crate::ToolboxIdlProgram;

pub fn idl_lib_native_compute_budget() -> ToolboxIdlProgram {
    ToolboxIdlProgram::try_parse_from_value(&json!({
        "instructions": {
            "RequestHeapFrame": {
                "discriminator": [1, 0, 0, 0],
                "accounts": [],
                "args": ["u32"]
            },
            "SetComputeUnitLimit": {
                "discriminator": [2, 0, 0, 0],
                "accounts": [],
                "args": ["u32"]
            },
            "SetComputeUnitPrice": {
                "discriminator": [3, 0, 0, 0],
                "accounts": [],
                "args": ["u64"]
            },
        },
        "accounts": {
            "ComputeBudgetAccount": {
                "discriminator": [],
                "fields": []
            },
        },
        "types": [],
        "errors": [],
    }))
    .unwrap()
}
