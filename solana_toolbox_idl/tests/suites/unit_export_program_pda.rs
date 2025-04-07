use serde_json::json;
use solana_toolbox_idl::ToolboxIdlFormat;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program = ToolboxIdlProgram::try_parse_from_value(&json!({
        "instructions": {
            "my_ix": {
                "discriminator": [],
                "accounts": [
                    { "name": "info" },
                    {
                        "name": "addr",
                        "pda": {
                            "seeds": [
                                [1, 2, 3],
                                {
                                    "kind": "arg",
                                    "path": "param.field",
                                },
                                {
                                    "path": "info.struct.field",
                                },
                            ]
                        }
                    }
                ],
                "args": [
                    {"name": "param", "type": "Struct"},
                ]
            }
        },
        "accounts": {
            "Accounts": {
                "fields": ["Struct"]
            },
        },
        "types": {
            "Struct": {
                "fields": [
                    { "name": "field", "type": "u8" },
                ],
            },
        },
    }))
    .unwrap();
    // Check the JSON human compact version
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::human()),
        json!({
            "metadata": {},
            "instructions": {
                "my_ix": {
                    "discriminator": [],
                    "accounts": [
                        {
                            "name": "addr",
                            "pda": {
                                "seeds": [
                                    { "path": "my.path" }
                                ]
                            }
                        }
                    ],
                    "args": []
                }
            },
            "accounts": {},
            "errors": {},
            "types": {},
        })
    );
    // Check the JSON backward compatibility version for anchor 26
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::anchor_26()),
        json!({
            "instructions": [
                {
                    "name": "my_ix",
                    "discriminator": [],
                    "accounts": [
                        {
                            "name": "addr",
                        }
                    ],
                    "args": [],
                }
            ],
            "accounts": [],
            "types": [],
            "errors": [],
        })
    );
    // Check the JSON backward compatibility version for anchor 30
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::anchor_30()),
        json!({
            "metadata": {},
            "instructions": [
                {
                    "name": "my_ix",
                    "discriminator": [],
                    "accounts": [
                        {
                            "name": "addr",
                        }
                    ],
                    "args": [],
                }
            ],
            "accounts": [],
            "types": [],
            "errors": [],
        })
    );
}
