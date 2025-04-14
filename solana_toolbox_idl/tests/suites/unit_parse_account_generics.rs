use serde_json::json;
use solana_toolbox_idl::ToolboxIdlAccount;
use solana_toolbox_idl::ToolboxIdlProgram;
use solana_toolbox_idl::ToolboxIdlTypeFlat;
use solana_toolbox_idl::ToolboxIdlTypeFlatFields;
use solana_toolbox_idl::ToolboxIdlTypeFull;
use solana_toolbox_idl::ToolboxIdlTypeFullFields;
use solana_toolbox_idl::ToolboxIdlTypePrimitive;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl_program = ToolboxIdlProgram::try_parse_from_value(&json!({
        "accounts": {
            "MyAccount": {
                "discriminator": [77],
                "fields": [
                    {
                        "defined": {
                            "name": "MyDefinedEnum",
                            "generics": ["u8"],
                        }
                    },
                    {
                        "defined": {
                            "name": "MyDefinedStruct",
                            "generics": ["f32", "f64"],
                        },
                    },
                    {
                        "defined": {
                            "name": "MyArray",
                            "generics": ["i8", 4],
                        }
                    }
                ],
            },
        },
        "types": {
            "MyDefinedEnum": {
                "generics": ["D"],
                "defined": {
                    "name": "MyEnum",
                    "generics": [
                        [{"generic": "D"}],
                        {"generic": "D"},
                    ],
                },
            },
            "MyDefinedStruct": {
                "generics": ["D", "E"],
                "defined": {
                    "name": "MyStruct",
                    "generics": [
                        {"option": {"generic": "E"}},
                        [{"generic": "D"}],
                    ],
                },
            },
            "MyEnum": {
                "generics": ["A", "B"],
                "variants": [
                    { "name": "CaseA", "fields": [{"generic": "A"}] },
                    { "name": "CaseB", "fields": [{"generic": "B"}] },
                ],
            },
            "MyStruct": {
                "generics": ["A", "B"],
                "fields": [
                    { "name": "field_a", "generic": "A" },
                    { "name": "field_b", "generic": "B" },
                ],
            },
            "MyArray": {
                "generics": ["C", "L"],
                "type": [{"generic": "C"}, {"generic": "L"}],
            },
        },
    }))
    .unwrap();
    // Assert that the content is correct
    assert_eq!(
        *idl_program.accounts.get("MyAccount").unwrap(),
        ToolboxIdlAccount {
            name: "MyAccount".to_string(),
            docs: None,
            space: None,
            blobs: vec![],
            discriminator: vec![77],
            content_type_flat: ToolboxIdlTypeFlat::Struct {
                fields: ToolboxIdlTypeFlatFields::Unnamed(vec![
                    (
                        None,
                        ToolboxIdlTypeFlat::Defined {
                            name: "MyDefinedEnum".to_string(),
                            generics: vec![ToolboxIdlTypeFlat::Primitive {
                                primitive: ToolboxIdlTypePrimitive::U8
                            }]
                        }
                    ),
                    (
                        None,
                        ToolboxIdlTypeFlat::Defined {
                            name: "MyDefinedStruct".to_string(),
                            generics: vec![
                                ToolboxIdlTypeFlat::Primitive {
                                    primitive: ToolboxIdlTypePrimitive::F32
                                },
                                ToolboxIdlTypeFlat::Primitive {
                                    primitive: ToolboxIdlTypePrimitive::F64
                                },
                            ]
                        }
                    ),
                    (
                        None,
                        ToolboxIdlTypeFlat::Defined {
                            name: "MyArray".to_string(),
                            generics: vec![
                                ToolboxIdlTypeFlat::Primitive {
                                    primitive: ToolboxIdlTypePrimitive::I8
                                },
                                ToolboxIdlTypeFlat::Const { literal: 4 },
                            ]
                        }
                    ),
                ])
            },
            content_type_full: ToolboxIdlTypeFull::Struct {
                fields: ToolboxIdlTypeFullFields::Unnamed(vec![
                    ToolboxIdlTypeFull::Enum {
                        variants: vec![
                            (
                                "CaseA".to_string(),
                                ToolboxIdlTypeFullFields::Unnamed(vec![
                                    ToolboxIdlTypeFull::Vec {
                                        items: Box::new(
                                            ToolboxIdlTypeFull::Primitive {
                                                primitive:
                                                    ToolboxIdlTypePrimitive::U8
                                            }
                                        )
                                    },
                                ])
                            ),
                            (
                                "CaseB".to_string(),
                                ToolboxIdlTypeFullFields::Unnamed(vec![
                                    ToolboxIdlTypeFull::Primitive {
                                        primitive: ToolboxIdlTypePrimitive::U8
                                    }
                                ])
                            ),
                        ]
                    },
                    ToolboxIdlTypeFull::Struct {
                        fields: ToolboxIdlTypeFullFields::Named(vec![
                            (
                                "field_a".to_string(),
                                ToolboxIdlTypeFull::Option {
                                    prefix_bytes: 1,
                                    content: Box::new(
                                        ToolboxIdlTypeFull::Primitive {
                                            primitive:
                                                ToolboxIdlTypePrimitive::F64
                                        }
                                    )
                                }
                            ),
                            (
                                "field_b".to_string(),
                                ToolboxIdlTypeFull::Vec {
                                    items: Box::new(
                                        ToolboxIdlTypeFull::Primitive {
                                            primitive:
                                                ToolboxIdlTypePrimitive::F32
                                        }
                                    )
                                },
                            ),
                        ])
                    },
                    ToolboxIdlTypeFull::Array {
                        items: Box::new(ToolboxIdlTypeFull::Primitive {
                            primitive: ToolboxIdlTypePrimitive::I8
                        }),
                        length: 4
                    }
                ])
            }
        }
        .into()
    )
}
