use serde_json::json;
use solana_toolbox_idl::ToolboxIdlFormat;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program = ToolboxIdlProgram::try_parse_from_value(&json!({
        "name": "my_Program",
        "metadata": {
            "description": "My program description"
        },
        "instructions": {
            "my_ix": {
                "discriminator": [77, 78],
                "accounts": [
                    { "name": "addr", "signer": true, "writable": true, "optional": true }
                ],
                "args": [
                    { "name": "arg1", "type": {"defined": "MyStruct"} },
                    { "name": "arg2", "type": "i16" },
                ]
            }
        },
        "types": {
            "MyStruct": {
                "fields": [
                    { "name": "id", "type": "u16" },
                    { "name": "data", "vec": "u8" },
                    { "name": "addr", "type": "publicKey" },
                ]
            }
        },
    }))
    .unwrap();
    // Check the JSON human compact version
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::Human),
        json!({
            "metadata": {
                "name": "MyProgram",
                "description": "My program description",
            },
            "instructions": {
                "my_ix": {
                    "discriminator": [77, 78],
                    "accounts": [
                        { "name": "addr", "signer": true, "writable": true, "optional": true }
                    ],
                    "args": [
                        { "name": "arg1", "type": "MyStruct" },
                        { "name": "arg2", "type": "i16" },
                    ]
                }
            },
            "accounts": {},
            "errors": {},
            "types": {
                "MyStruct": {
                    "fields": [
                        { "name": "id", "type": "u16" },
                        { "name": "data", "type": ["u8"] },
                        { "name": "addr", "type": "pubkey" },
                    ]
                }
            },
        })
    );
    // Check the JSON backward compatibility version for anchor 26
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::Anchor26),
        json!({
            "name": "MyProgram",
            "description": "My program description",
            "accounts": [],
            "errors": [],
            "instructions": [
                {
                    "name": "my_ix",
                    "discriminator": [77, 78],
                    "accounts": [
                        { "name": "addr", "isSigner": true, "isMut": true, "isOptional": true }
                    ],
                    "args": [
                        { "name": "arg1", "type": {"defined": "MyStruct"} },
                        { "name": "arg2", "type": "i16" },
                    ],
                }
            ],
            "types": [
                {
                    "name": "MyStruct",
                    "type": {
                        "kind": "struct",
                        "fields":[
                            { "name": "id", "type": "u16" },
                            { "name": "data", "type": {"vec": "u8"} },
                            { "name": "addr", "type": "publicKey" },
                        ]
                    }
                }
            ]
        })
    );
    // Check the JSON backward compatibility version for anchor 30
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::Anchor30),
        json!({
            "metadata": {
                "name": "MyProgram",
                "description": "My program description",
            },
            "accounts": [],
            "errors": [],
            "instructions": [
                {
                    "name": "my_ix",
                    "discriminator": [77, 78],
                    "accounts": [
                        { "name": "addr", "signer": true, "writable": true, "optional": true }
                    ],
                    "args": [
                        { "name": "arg1", "type": {"defined": {"name": "MyStruct"}} },
                        { "name": "arg2", "type": "i16" }
                    ],
                }
            ],
            "types": [
                {
                    "name": "MyStruct",
                    "type": {
                        "kind": "struct",
                        "fields":[
                            { "name": "id", "type": "u16" },
                            { "name": "data", "type": {"vec": "u8"} },
                            { "name": "addr", "type": "pubkey" },
                        ]
                    }
                }
            ]
        })
    );
}
