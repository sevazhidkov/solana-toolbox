use serde_json::json;
use solana_toolbox_idl::ToolboxIdlProgram;
use solana_toolbox_idl::ToolboxIdlTypeFlat;
use solana_toolbox_idl::ToolboxIdlTypeFlatFields;
use solana_toolbox_idl::ToolboxIdlTypePrimitive;
use solana_toolbox_idl::ToolboxIdlTypedef;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl_program1 = ToolboxIdlProgram::try_parse_from_value(&json!({
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
                        "name": "Unnamed",
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
    let idl_program2 = ToolboxIdlProgram::try_parse_from_value(&json!({
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
                        "name": "Unnamed",
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
    assert_eq!(idl_program1, idl_program2);
    // Assert that the content is correct
    assert_eq!(
        *idl_program1.typedefs.get("MyEnum").unwrap(),
        ToolboxIdlTypedef {
            name: "MyEnum".to_string(),
            docs: None,
            generics: vec![],
            type_flat: ToolboxIdlTypeFlat::Enum {
                prefix_bytes: 1,
                variants: vec![
                    (
                        "Named".to_string(),
                        None,
                        ToolboxIdlTypeFlatFields::Named(vec![
                            (
                                "f1".to_string(),
                                None,
                                ToolboxIdlTypeFlat::Defined {
                                    name: "Other".to_string(),
                                    generics: vec![]
                                }
                            ),
                            (
                                "f2".to_string(),
                                None,
                                ToolboxIdlTypeFlat::Vec {
                                    prefix_bytes: 4,
                                    items: Box::new(
                                        ToolboxIdlTypeFlat::Primitive {
                                            primitive:
                                                ToolboxIdlTypePrimitive::U8,
                                        }
                                    )
                                }
                            ),
                            (
                                "f3".to_string(),
                                None,
                                ToolboxIdlTypeFlat::Generic {
                                    symbol: "G".to_string()
                                }
                            ),
                        ])
                    ),
                    (
                        "Unnamed".to_string(),
                        None,
                        ToolboxIdlTypeFlatFields::Unnamed(vec![
                            (
                                None,
                                ToolboxIdlTypeFlat::Primitive {
                                    primitive: ToolboxIdlTypePrimitive::U64,
                                }
                            ),
                            (
                                None,
                                ToolboxIdlTypeFlat::Vec {
                                    prefix_bytes: 4,
                                    items: Box::new(
                                        ToolboxIdlTypeFlat::Primitive {
                                            primitive:
                                                ToolboxIdlTypePrimitive::U8,
                                        }
                                    )
                                }
                            ),
                            (
                                None,
                                ToolboxIdlTypeFlat::Vec {
                                    prefix_bytes: 4,
                                    items: Box::new(
                                        ToolboxIdlTypeFlat::Primitive {
                                            primitive:
                                                ToolboxIdlTypePrimitive::U8,
                                        }
                                    )
                                }
                            ),
                        ]),
                    ),
                    ("Empty".to_string(), None, ToolboxIdlTypeFlatFields::None),
                ]
            }
        }
        .into()
    )
}
