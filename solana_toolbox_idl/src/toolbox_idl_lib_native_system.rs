use serde_json::json;

use crate::ToolboxIdlProgram;

pub fn idl_lib_native_system() -> ToolboxIdlProgram {
    ToolboxIdlProgram::try_parse_from_value(&json!({
        "instructions": {
            "Create": {
                "discriminator": [0, 0, 0, 0],
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
                ]
            },
            "Assign": {
                "discriminator": [1, 0, 0, 0],
                "accounts": [
                    {
                        "name": "assigned",
                        "isMut": true,
                        "isSigner": true,
                    }
                ],
                "args": [
                    {
                        "name": "owner",
                        "type": "publicKey",
                    }
                ]
            },
            "Transfer": {
                "discriminator": [2, 0, 0, 0],
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
                "args": [
                    {
                        "name": "lamports",
                        "type": "u64",
                    }
                ]
            },
            "Allocate": {
                "discriminator": [8, 0, 0, 0],
                "accounts": [
                    {
                        "name": "allocated",
                        "isMut": true,
                        "isSigner": true,
                    }
                ],
                "args": [
                    {
                        "name": "space",
                        "type": "u64",
                    }
                ]
            },
        },
        "accounts": {
            "SystemAccount": { "discriminator": [], "fields": [] }
        },
        "types": [],
        "errors": [],
    }))
    .unwrap()
}
