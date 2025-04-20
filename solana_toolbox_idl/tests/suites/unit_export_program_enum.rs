use serde_json::json;
use solana_toolbox_idl::ToolboxIdlFormat;
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
                        "docs": ["My Enum Variant B"],
                        "fields": ["u8"]
                    },
                    {
                        "name": "C",
                        "code": 42,
                        "fields": [{"name": "v", "type": "u8"}]
                    },
                ],
            }
        },
    }))
    .unwrap();
    // Check the JSON human compact version
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::human()),
        json!({
            "metadata": {},
            "instructions": {},
            "accounts": {},
            "types": {
                "MyEnum": {
                    "variants": [
                        "A",
                        {
                            "name": "B",
                            "docs": ["My Enum Variant B"],
                            "fields": ["u8"],
                        },
                        {
                            "name": "C",
                            "code": 42,
                            "fields": [{"name": "v", "type": "u8"}]
                        },
                    ],
                }
            },
            "events": {},
            "errors": {},
        })
    );
    // Check the JSON backward compatibility version for anchor 26
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::anchor_26()),
        json!({
            "metadata": {},
            "instructions": [],
            "accounts": [],
            "types": [
                {
                    "name": "MyEnum",
                    "type": {
                        "kind": "enum",
                        "variants": [
                            { "name": "A" },
                            {
                                "name": "B",
                                "docs": ["My Enum Variant B"],
                                "fields": [{"type": "u8"}],
                            },
                            {
                                "name": "C",
                                "code": 42,
                                "fields": [{"name": "v", "type": "u8"}]
                            },
                        ]
                    }
                }
            ],
            "events": [],
            "errors": [],
        }),
    );
    // Check the JSON backward compatibility version for anchor 30
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::anchor_30()),
        json!({
            "metadata": {},
            "instructions": [],
            "accounts": [],
            "types": [
                {
                    "name": "MyEnum",
                    "type": {
                        "kind": "enum",
                        "variants": [
                            { "name": "A" },
                            {
                                "name": "B",
                                "docs": ["My Enum Variant B"],
                                "fields": [{"type": "u8"}],
                            },
                            {
                                "name": "C",
                                "code": 42,
                                "fields": [{"name": "v", "type": "u8"}]
                            },
                        ]
                    }
                }
            ],
            "events": [],
            "errors": [],
        })
    );
}
