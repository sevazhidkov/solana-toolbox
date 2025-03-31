use serde_json::json;
use solana_toolbox_idl::ToolboxIdlFormat;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program = ToolboxIdlProgram::try_parse_from_value(&json!({
        "types": {
            "MyStruct1": {
                "docs": ["My Struct 1"],
                "fields": [
                    { "name": "id", "type": "u16", "docs": ["My Struct 1 Field Id"] },
                    { "name": "data", "vec": "u8" },
                    { "name": "addr", "type": "publicKey" },
                ]
            },
        },
    }))
    .unwrap();
    // Check the JSON human compact version
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::Human),
        json!({
            "metadata": {},
            "instructions": {},
            "accounts": {},
            "errors": {},
            "types": {
                "MyStruct1": {
                    "docs": ["My Struct 1"],
                    "type": {
                        "fields": [
                            { "name": "id", "type": "u16", "docs": ["My Struct 1 Field Id"] },
                            { "name": "data", "type": ["u8"] },
                            { "name": "addr", "type": "pubkey" },
                        ]
                    }
                },
            },
        })
    );
    // Check the JSON backward compatibility version for anchor 26
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::Anchor26),
        json!({
            "accounts": [],
            "errors": [],
            "instructions": [],
            "types": [
                {
                    "name": "MyStruct1",
                    "docs": ["My Struct 1"],
                    "type": {
                        "kind": "struct",
                        "fields": [
                            { "name": "id", "type": "u16", "docs": ["My Struct 1 Field Id"] },
                            { "name": "data", "type": {"vec": "u8"} },
                            { "name": "addr", "type": "publicKey" },
                        ]
                    }
                },
            ]
        })
    );
    // Check the JSON backward compatibility version for anchor 30
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::Anchor30),
        json!({
            "metadata": {},
            "accounts": [],
            "errors": [],
            "instructions": [],
            "types": [
                {
                    "name": "MyStruct1",
                    "docs": ["My Struct 1"],
                    "type": {
                        "kind": "struct",
                        "fields": [
                            { "name": "id", "type": "u16", "docs": ["My Struct 1 Field Id"] },
                            { "name": "data", "type": {"vec": "u8"} },
                            { "name": "addr", "type": "pubkey" },
                        ]
                    }
                },
            ]
        })
    );
}
