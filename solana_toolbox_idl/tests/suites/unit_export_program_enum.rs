use serde_json::json;
use solana_toolbox_idl::ToolboxIdlInfoFormat;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program = ToolboxIdlProgram::try_parse_from_value(&json!({
        "instructions": {},
        "types": {
            "MyEnum": {
                "variants": [
                    "A",
                    {
                        "name": "B",
                        // TODO (FAR) - support ToolboxIdlDocs for "docs": ["My Enum Field B"],
                        "fields": ["u8"]
                    },
                    {
                        "name": "C",
                        "fields": [{"name": "v", "type": "u8"}]
                    },
                ],
            }
        },
    }))
    .unwrap();
    // Check the JSON human compact version
    assert_eq!(
        idl_program.export(&ToolboxIdlInfoFormat::Human),
        json!({
            "metadata": {},
            "instructions": {},
            "accounts": {},
            "errors": {},
            "types": {
                "MyEnum": {
                    "variants": [
                        "A",
                        {
                            "name": "B",
                            "fields": ["u8"],
                        },
                        {
                            "name": "C",
                            "fields": [{"name": "v", "type": "u8"}]
                        },
                    ],
                }
            },
        })
    );
    // Check the JSON backward compatibility version for anchor 26
    assert_eq!(
        idl_program.export(&ToolboxIdlInfoFormat::Anchor26),
        json!({
            "accounts": [],
            "errors": [],
            "instructions": [],
            "types": [
                {
                    "name": "MyEnum",
                    "type": {
                        "kind": "enum",
                        "variants": [
                            { "name": "A" },
                            {
                                "name": "B",
                                "fields": [{"type": "u8"}],
                            },
                            {
                                "name": "C",
                                "fields": [{"name": "v", "type": "u8"}]
                            },
                        ]
                    }
                }
            ]
        })
    );
    // Check the JSON backward compatibility version for anchor 30
    assert_eq!(
        idl_program.export(&ToolboxIdlInfoFormat::Anchor30),
        json!({
            "metadata": {},
            "accounts": [],
            "errors": [],
            "instructions": [],
            "types": [
                {
                    "name": "MyEnum",
                    "type": {
                        "kind": "enum",
                        "variants": [
                            { "name": "A" },
                            {
                                "name": "B",
                                "fields": [{"type": "u8"}],
                            },
                            {
                                "name": "C",
                                "fields": [{"name": "v", "type": "u8"}]
                            },
                        ]
                    }
                }
            ]
        })
    );
}
