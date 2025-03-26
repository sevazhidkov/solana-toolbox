use serde_json::json;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program = ToolboxIdlProgram::try_parse_from_value(&json!({
        "instructions": {
            "my_ix": {
                "discriminator": [77, 78],
                "accounts": [
                    { "name": "addr", "signer": true, "writable": true, "optional": true }
                ],
                "args": [
                    { "name": "arg1", "type": {"defined": "MyArg"} },
                    { "name": "arg2", "type": "i16" },
                ]
            }
        },
        "types": {
            "MyArg": {
                "fields": [
                    { "name": "id", "type": "u16" },
                    { "name": "data", "type": {"vec": "u8"} },
                ]
            }
        },
    }))
    .unwrap();
    // Check the JSON human compact version
    assert_eq!(
        idl_program.export(false),
        json!({
            "metadata": {},
            "instructions": {
                "my_ix": {
                    "discriminator": [77, 78],
                    "accounts": [
                        { "name": "addr", "signer": true, "writable": true, "optional": true }
                    ],
                    "args": [
                        { "name": "arg1", "type": "MyArg" },
                        { "name": "arg2", "type": "i16" },
                    ]
                }
            },
            "accounts": {},
            "errors": {},
            "types": {
                "MyArg": {
                    "fields": [
                        { "name": "id", "type": "u16" },
                        { "name": "data", "type": ["u8"] },
                    ]
                }
            },
        })
    );
    // Check the JSON backward compatibility version
    assert_eq!( // TODO (FAR) - add more in-depth testing for all anchor version and parts of IDL (should we re-use the parse_ tests?)
        idl_program.export(true).to_string(),
        json!({
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
                        { "name": "arg1", "type": {"defined": {"name": "MyArg"}} },
                        { "name": "arg2", "type": "i16" }
                    ],
                }
            ],
            "types": [
                {
                    "name": "MyArg",
                    "type": {
                        "kind": "struct",
                        "fields":[
                            { "name": "id", "type": "u16" },
                            { "name": "data", "type": {"vec": "u8"} },
                        ]
                    }
                }
            ]
        }).to_string()
    );
}
