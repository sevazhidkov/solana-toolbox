use serde_json::json;
use solana_toolbox_idl::ToolboxIdl;
use solana_toolbox_idl::ToolboxIdlPrimitive;
use solana_toolbox_idl::ToolboxIdlProgramType;
use solana_toolbox_idl::ToolboxIdlTypeFlat;
use solana_toolbox_idl::ToolboxIdlTypeFlatFields;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl1 = ToolboxIdl::try_from_value(&json!({
        "types": {
            "MyEnum": {
                "variants": [
                    {
                        "name": "Named",
                        "fields": [
                            { "name": "f1", "type": {"defined": "Other"} },
                            { "name": "f2", "type": {"vec": "u8"} },
                            { "name": "f3", "type": {"generic": "G"} },
                        ]
                    },
                    {
                        "name": "Unamed",
                        "fields": [
                            { "type": "u64" },
                            { "type": ["u8"] },
                            { "type": {"vec": "u8"} },
                        ]
                    },
                    { "name": "Empty", "fields": [] },
                ]
            },
        },
    }))
    .unwrap();
    let idl2 = ToolboxIdl::try_from_value(&json!({
        "types": {
            "MyEnum": {
                "variants": [
                    {
                        "name": "Named",
                        "fields": [
                            { "name": "f1", "defined": "Other" },
                            { "name": "f2", "vec": "u8" },
                            { "name": "f3", "generic": "G" },
                        ]
                    },
                    {
                        "name": "Unamed",
                        "fields": [
                            "u64",
                            ["u8"],
                            {"vec": "u8"},
                        ],
                    },
                    { "name": "Empty" },
                ]
            },
        },
    }))
    .unwrap();
    // Assert that all are equivalent
    assert_eq!(idl1, idl2);
    // Assert that the content is correct
    assert_eq!(
        idl1.program_types.get("MyEnum").unwrap(),
        &ToolboxIdlProgramType {
            name: "MyEnum".to_string(),
            generics: vec![],
            type_flat: ToolboxIdlTypeFlat::Enum {
                variants: vec![
                    (
                        "Named".to_string(),
                        ToolboxIdlTypeFlatFields::Named(vec![
                            (
                                "f1".to_string(),
                                ToolboxIdlTypeFlat::Defined {
                                    name: "Other".to_string(),
                                    generics: vec![]
                                },
                            ),
                            (
                                "f2".to_string(),
                                ToolboxIdlTypeFlat::Vec {
                                    items: Box::new(
                                        ToolboxIdlTypeFlat::Primitive {
                                            primitive: ToolboxIdlPrimitive::U8,
                                        }
                                    )
                                },
                            ),
                            (
                                "f3".to_string(),
                                ToolboxIdlTypeFlat::Generic {
                                    symbol: "G".to_string()
                                },
                            ),
                        ])
                    ),
                    (
                        "Unamed".to_string(),
                        ToolboxIdlTypeFlatFields::Unamed(vec![
                            ToolboxIdlTypeFlat::Primitive {
                                primitive: ToolboxIdlPrimitive::U64,
                            },
                            ToolboxIdlTypeFlat::Vec {
                                items: Box::new(
                                    ToolboxIdlTypeFlat::Primitive {
                                        primitive: ToolboxIdlPrimitive::U8,
                                    }
                                )
                            },
                            ToolboxIdlTypeFlat::Vec {
                                items: Box::new(
                                    ToolboxIdlTypeFlat::Primitive {
                                        primitive: ToolboxIdlPrimitive::U8,
                                    }
                                )
                            },
                        ]),
                    ),
                    ("Empty".to_string(), ToolboxIdlTypeFlatFields::None),
                ]
            }
        }
    )
}
