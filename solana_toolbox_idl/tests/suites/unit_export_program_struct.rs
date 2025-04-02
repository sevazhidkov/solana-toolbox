use serde_json::json;
use solana_toolbox_idl::ToolboxIdlInfoFormat;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create IDLs on the fly
    let idl_program1 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "types": {
            "MyStruct": {
                "docs": ["My Struct"],
                "fields": [
                    { "name": "id", "type": "u16", "docs": ["My Struct Field Id"] },
                    { "name": "data", "vec": "u8" },
                    { "name": "addr", "type": "publicKey" },
                ]
            },
        },
    }))
    .unwrap();
    let idl_program2 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "types": {
            "MyStruct": { "fields": [] },
        },
    }))
    .unwrap();
    // Check the JSON human compact version
    assert_eq!(
        idl_program1.export(&ToolboxIdlInfoFormat::Human),
        json!({
            "metadata": {},
            "instructions": {},
            "accounts": {},
            "types": {
                "MyStruct": {
                    "docs": ["My Struct"],
                    "type": {
                        "fields": [
                            { "name": "id", "type": "u16", "docs": ["My Struct Field Id"] },
                            { "name": "data", "type": ["u8"] },
                            { "name": "addr", "type": "pubkey" },
                        ]
                    }
                },
            },
            "errors": {},
        })
    );
    assert_eq!(
        idl_program2.export(&ToolboxIdlInfoFormat::Human),
        json!({
            "metadata": {},
            "instructions": {},
            "accounts": {},
            "types": {
                "MyStruct": { "fields": [] },
            },
            "errors": {},
        })
    );
    // Check the JSON backward compatibility version for anchor 26
    assert_eq!(
        idl_program1.export(&ToolboxIdlInfoFormat::Anchor26),
        json!({
            "instructions": [],
            "accounts": [],
            "types": [
                {
                    "name": "MyStruct",
                    "docs": ["My Struct"],
                    "type": {
                        "kind": "struct",
                        "fields": [
                            { "name": "id", "type": "u16", "docs": ["My Struct Field Id"] },
                            { "name": "data", "type": {"vec": "u8"} },
                            { "name": "addr", "type": "publicKey" },
                        ]
                    }
                },
            ],
            "errors": [],
        })
    );
    assert_eq!(
        idl_program2.export(&ToolboxIdlInfoFormat::Anchor26),
        json!({
            "instructions": [],
            "accounts": [],
            "types": [
                {
                    "name": "MyStruct",
                    "type": {
                        "kind": "struct",
                        "fields": []
                    }
                },
            ],
            "errors": [],
        })
    );
    // Check the JSON backward compatibility version for anchor 30
    assert_eq!(
        idl_program1.export(&ToolboxIdlInfoFormat::Anchor30),
        json!({
            "metadata": {},
            "instructions": [],
            "accounts": [],
            "types": [
                {
                    "name": "MyStruct",
                    "docs": ["My Struct"],
                    "type": {
                        "kind": "struct",
                        "fields": [
                            { "name": "id", "type": "u16", "docs": ["My Struct Field Id"] },
                            { "name": "data", "type": {"vec": "u8"} },
                            { "name": "addr", "type": "pubkey" },
                        ]
                    }
                },
            ],
            "errors": [],
        })
    );
    assert_eq!(
        idl_program2.export(&ToolboxIdlInfoFormat::Anchor30),
        json!({
            "metadata": {},
            "instructions": [],
            "accounts": [],
            "types": [
                {
                    "name": "MyStruct",
                    "type": {
                        "kind": "struct",
                        "fields": []
                    }
                },
            ],
            "errors": [],
        })
    );
}
