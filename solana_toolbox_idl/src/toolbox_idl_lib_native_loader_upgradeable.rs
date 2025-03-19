use serde_json::json;

use crate::ToolboxIdlProgram;

pub fn idl_lib_native_loader_upgradeable() -> ToolboxIdlProgram {
    ToolboxIdlProgram::try_parse_from_value(&json!({
        "instructions": {
            "InitializeBuffer": {
                "discriminator": [0, 0, 0, 0],
                "accounts": [
                    {"name": "buffer"},
                    {"name": "buffer_authority"},
                ],
                "args": []
            },
            "Write": {
                "discriminator": [1, 0, 0, 0],
                "accounts": [
                    {"name": "buffer"},
                    {"name": "buffer_authority"},
                ],
                "args": [
                    {"name": "offset", "type": "u32"},
                    {"name": "bytes", "type": ["u8"]},
                ],
            },
            "DeployWithMaxDataLen": {
                "discriminator": [2, 0, 0, 0],
                "accounts": [
                    {"name": "payer"},
                    {"name": "program_data"},
                    {"name": "program_id"},
                    {"name": "buffer"},
                    {"name": "rent"},
                    {"name": "clock"},
                    {"name": "system_program"},
                    {"name": "upgrade_authority"},
                ],
                "args": [
                    {"name": "max_data_len", "type": "u64"},
                ],
            },
            "Upgrade": {
                "discriminator": [3, 0, 0, 0],
                "accounts": [
                    {"name": "program_data"},
                    {"name": "program_id"},
                    {"name": "buffer"},
                    {"name": "spill"},
                    {"name": "rent"},
                    {"name": "clock"},
                    {"name": "upgrade_authority"},
                ],
                "args": [],
            },
            "SetAuthority": {
                "discriminator": [4, 0, 0, 0],
                "accounts": [
                    {"name": "modified"},
                    {"name": "prev_authority"},
                    {"name": "next_authority"},
                ],
                "args": [],
            },
            "Close": {
                "discriminator": [5, 0, 0, 0],
                "accounts": [
                    // TODO - this is complicated with optional accounts
                ],
                "args": [],
            },
            "ExtendProgram": {
                "discriminator": [6, 0, 0, 0],
                "accounts": [
                    // TODO - this is complicated with optional accounts
                ],
                "args": [
                    {"name": "additional_bytes", "type": "u32"},
                ],
            },
            "SetAuthorityChecked": {
                "discriminator": [7, 0, 0, 0],
                "accounts": [
                    {"name": "modified"},
                    {"name": "prev_authority"},
                    {"name": "next_authority"},
                ],
                "args": [],
            },
        },
        "accounts": {
            "LoaderUpgradeableAccount": {
                "discriminator": [],
                "fields": [],
            },
        },
        "types": [],
        "errors": [],
    }))
    .unwrap()
}
