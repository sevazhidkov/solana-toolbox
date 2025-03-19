use serde_json::json;

use crate::toolbox_idl_program::ToolboxIdlProgram;

pub fn idl_lib_spl_token() -> ToolboxIdlProgram {
    ToolboxIdlProgram::try_parse_from_value(&json!({
        "name": "spl_token",
        "instructions": {
            "InitializeMint": {
                "discriminator": [0, 0, 0, 0],
                "args": [],
                "accounts": [],
            },
            "InitializeAccount": {
                "discriminator": [1, 0, 0, 0],
                "args": [],
                "accounts": [],
            },
            "InitializeMultisig": {
                "discriminator": [2, 0, 0, 0],
                "args": [],
                "accounts": [],
            },
            "Transfer": {
                "discriminator": [3, 0, 0, 0],
                "args": [],
                "accounts": [],
            },
            "Approve": {
                "discriminator": [4, 0, 0, 0],
                "args": [],
                "accounts": [],
            },
            "Revoke": {
                "discriminator": [5, 0, 0, 0],
                "args": [],
                "accounts": [],
            },
            "SetAuthority": {
                "discriminator": [6, 0, 0, 0],
                "args": [],
                "accounts": [],
            },
            "MintTo": {
                "discriminator": [7, 0, 0, 0],
                "args": [],
                "accounts": [],
            },
            "Burn": {
                "discriminator": [8, 0, 0, 0],
                "args": [],
                "accounts": [],
            },
            "Close": {
                "discriminator": [9, 0, 0, 0],
                "args": [],
                "accounts": [],
            },
            "Freeze": {
                "discriminator": [10, 0, 0, 0],
                "args": [],
                "accounts": [],
            },
            "Thaw": {
                "discriminator": [11, 0, 0, 0],
                "args": [],
                "accounts": [],
            },
            "TransferChecked": {
                "discriminator": [12, 0, 0, 0],
                "args": [],
                "accounts": [],
            },
            "ApproveChecked": {
                "discriminator": [13, 0, 0, 0],
                "args": [],
                "accounts": [],
            },
            "MintToChecked": {
                "discriminator": [14, 0, 0, 0],
                "args": [],
                "accounts": [],
            },
            "BurnChecked": {
                "discriminator": [15, 0, 0, 0],
                "args": [],
                "accounts": [],
            },
            "InitializeAccount2": {
                "discriminator": [16, 0, 0, 0],
                "args": [],
                "accounts": [],
            },
            "SyncNative": {
                "discriminator": [17, 0, 0, 0],
                "args": [],
                "accounts": [],
            },
            "InitializeAccount3": {
                "discriminator": [18, 0, 0, 0],
                "args": [],
                "accounts": [],
            },
            "InitializeMultisig2": {
                "discriminator": [19, 0, 0, 0],
                "args": [],
                "accounts": [],
            },
            "InitializeMint2": {
                "discriminator": [20, 0, 0, 0],
                "args": [],
                "accounts": [],
            },
        },
        "accounts": {
            "Account": { // TODO - ability to select between both based on length
                "discriminator": [],
                "fields": []
            },
            "Mint": {
                "discriminator": [],
                "fields": []
            },
        },
        "types": {},
        "errors": {},
    }))
    .unwrap()
}
