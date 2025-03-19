use serde_json::json;

use crate::toolbox_idl_program::ToolboxIdlProgram;

pub fn idl_lib_native_system() -> ToolboxIdlProgram {
    ToolboxIdlProgram::try_parse_from_value(&json!({
        "name": "system",
        "instructions": {
            "Create": {
                "discriminator": [0, 0, 0, 0],
                "args": [
                    {
                        "name": "lamports",
                        "type": "u64",
                    },
                    {
                        "name": "space",
                        "type": "u64",
                    },
                    {
                        "name": "owner",
                        "type": "publicKey",
                    }
                ],
                "accounts": [
                    {
                        "name": "payer",
                        "isMut": true,
                        "isSigner": true,
                    },
                    {
                        "name": "created",
                        "isMut": true,
                        "isSigner": true,
                    }
                ],
            },
            "Assign": {
                "discriminator": [1, 0, 0, 0],
                "args": [
                    {
                        "name": "owner",
                        "type": "publicKey",
                    }
                ],
                "accounts": [
                    {
                        "name": "assigned",
                        "isMut": true,
                        "isSigner": true,
                    }
                ],
            },
            "Transfer": {
                "discriminator": [2, 0, 0, 0],
                "args": [
                    {
                        "name": "lamports",
                        "type": "u64",
                    }
                ],
                "accounts": [
                    {
                        "name": "payer",
                        "isMut": true,
                        "isSigner": true,
                    },
                    {
                        "name": "receiver",
                        "isMut": true,
                    }
                ],
            },
            "Allocate": {
                "discriminator": [8, 0, 0, 0],
                "args": [
                    {
                        "name": "space",
                        "type": "u64",
                    }
                ],
                "accounts": [
                    {
                        "name": "allocated",
                        "isMut": true,
                        "isSigner": true,
                    }
                ],
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
