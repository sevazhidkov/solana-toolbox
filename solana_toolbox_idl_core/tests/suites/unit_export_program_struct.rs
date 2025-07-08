use serde_json::json;
use solana_toolbox_idl_core::ToolboxIdlFormat;
use solana_toolbox_idl_core::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create IDLs on the fly
    let idl_program1 = ToolboxIdlProgram::try_parse(&json!({
        "types": {
            "MyStruct": {
                "docs": ["My Struct"],
                "fields": [
                    { "name": "id", "type": "u16", "docs": ["My Struct Field Id"] },
                    { "name": "my_data", "vec": "u8" },
                    { "name": "addr", "type": "publicKey" },
                ]
            },
        },
    }))
    .unwrap();
    let idl_program2 = ToolboxIdlProgram::try_parse(&json!({
        "types": {
            "MyStruct": { "fields": [] },
        },
    }))
    .unwrap();
    // Check the JSON human compact version
    assert_eq!(
        idl_program1.export(&ToolboxIdlFormat::human()),
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
                            { "name": "my_data", "type": ["u8"] },
                            { "name": "addr", "type": "pubkey" },
                        ]
                    }
                },
            },
            "events": {},
            "errors": {},
        })
    );
    assert_eq!(
        idl_program2.export(&ToolboxIdlFormat::human()),
        json!({
            "metadata": {},
            "instructions": {},
            "accounts": {},
            "types": {
                "MyStruct": { "fields": [] },
            },
            "events": {},
            "errors": {},
        })
    );
    // Check the JSON backward compatibility version for anchor 26
    assert_eq!(
        idl_program1.export(&ToolboxIdlFormat::anchor_26()),
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
                            { "name": "myData", "type": "bytes" },
                            { "name": "addr", "type": "publicKey" },
                        ]
                    }
                },
            ],
            "events": [],
            "errors": [],
        })
    );
    assert_eq!(
        idl_program2.export(&ToolboxIdlFormat::anchor_26()),
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
            "events": [],
            "errors": [],
        })
    );
    // Check the JSON backward compatibility version for anchor 30
    assert_eq!(
        idl_program1.export(&ToolboxIdlFormat::anchor_30()),
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
                            { "name": "my_data", "type": "bytes" },
                            { "name": "addr", "type": "pubkey" },
                        ]
                    }
                },
            ],
            "events": [],
            "errors": [],
        })
    );
    assert_eq!(
        idl_program2.export(&ToolboxIdlFormat::anchor_30()),
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
            "events": [],
            "errors": [],
        })
    );
}
