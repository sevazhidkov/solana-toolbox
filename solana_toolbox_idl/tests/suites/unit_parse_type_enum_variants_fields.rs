use serde_json::json;
use solana_toolbox_idl::ToolboxIdl;
use solana_toolbox_idl::ToolboxIdlProgramTypedef;
use solana_toolbox_idl::ToolboxIdlProgramTypeFlat;
use solana_toolbox_idl::ToolboxIdlProgramTypeFlatFields;
use solana_toolbox_idl::ToolboxIdlProgramTypePrimitive;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl1 = ToolboxIdl::try_parse_from_value(&json!({
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
    let idl2 = ToolboxIdl::try_parse_from_value(&json!({
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
        idl1.program_typedefs.get("MyEnum").unwrap(),
        &ToolboxIdlProgramTypedef {
            name: "MyEnum".to_string(),
            generics: vec![],
            type_flat: ToolboxIdlProgramTypeFlat::Enum {
                variants: vec![
                    (
                        "Named".to_string(),
                        ToolboxIdlProgramTypeFlatFields::Named(vec![
                            (
                                "f1".to_string(),
                                ToolboxIdlProgramTypeFlat::Defined {
                                    name: "Other".to_string(),
                                    generics: vec![]
                                },
                            ),
                            (
                                "f2".to_string(),
                                ToolboxIdlProgramTypeFlat::Vec {
                                    items: Box::new(
                                        ToolboxIdlProgramTypeFlat::Primitive {
                                            primitive:
                                                ToolboxIdlProgramTypePrimitive::U8,
                                        }
                                    )
                                },
                            ),
                            (
                                "f3".to_string(),
                                ToolboxIdlProgramTypeFlat::Generic {
                                    symbol: "G".to_string()
                                },
                            ),
                        ])
                    ),
                    (
                        "Unamed".to_string(),
                        ToolboxIdlProgramTypeFlatFields::Unamed(vec![
                            ToolboxIdlProgramTypeFlat::Primitive {
                                primitive: ToolboxIdlProgramTypePrimitive::U64,
                            },
                            ToolboxIdlProgramTypeFlat::Vec {
                                items: Box::new(
                                    ToolboxIdlProgramTypeFlat::Primitive {
                                        primitive: ToolboxIdlProgramTypePrimitive::U8,
                                    }
                                )
                            },
                            ToolboxIdlProgramTypeFlat::Vec {
                                items: Box::new(
                                    ToolboxIdlProgramTypeFlat::Primitive {
                                        primitive: ToolboxIdlProgramTypePrimitive::U8,
                                    }
                                )
                            },
                        ]),
                    ),
                    ("Empty".to_string(), ToolboxIdlProgramTypeFlatFields::None),
                ]
            }
        }
    )
}
