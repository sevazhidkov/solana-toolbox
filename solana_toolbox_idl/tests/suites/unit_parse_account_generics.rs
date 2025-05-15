use serde_json::json;
use solana_toolbox_idl::ToolboxIdlAccount;
use solana_toolbox_idl::ToolboxIdlProgram;
use solana_toolbox_idl::ToolboxIdlTypeFlat;
use solana_toolbox_idl::ToolboxIdlTypeFlatFieldUnnamed;
use solana_toolbox_idl::ToolboxIdlTypeFlatFields;
use solana_toolbox_idl::ToolboxIdlTypeFull;
use solana_toolbox_idl::ToolboxIdlTypeFullEnumVariant;
use solana_toolbox_idl::ToolboxIdlTypeFullFieldNamed;
use solana_toolbox_idl::ToolboxIdlTypeFullFieldUnnamed;
use solana_toolbox_idl::ToolboxIdlTypeFullFields;
use solana_toolbox_idl::ToolboxIdlTypePrefix;
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
                    ToolboxIdlTypeFlatFieldUnnamed {
                        docs: None,
                        content: ToolboxIdlTypeFlat::Defined {
                            name: "MyDefinedEnum".to_string(),
                            generics: vec![ToolboxIdlTypePrimitive::U8.into()]
                        }
                    },
                    ToolboxIdlTypeFlatFieldUnnamed {
                        docs: None,
                        content: ToolboxIdlTypeFlat::Defined {
                            name: "MyDefinedStruct".to_string(),
                            generics: vec![
                                ToolboxIdlTypePrimitive::F32.into(),
                                ToolboxIdlTypePrimitive::F64.into(),
                            ]
                        }
                    },
                    ToolboxIdlTypeFlatFieldUnnamed {
                        docs: None,
                        content: ToolboxIdlTypeFlat::Defined {
                            name: "MyArray".to_string(),
                            generics: vec![
                                ToolboxIdlTypePrimitive::I8.into(),
                                ToolboxIdlTypeFlat::Const { literal: 4 },
                            ]
                        }
                    },
                ])
            },
            content_type_full: ToolboxIdlTypeFull::Struct {
                fields: ToolboxIdlTypeFullFields::Unnamed(vec![
                    ToolboxIdlTypeFullFieldUnnamed {
                        content: ToolboxIdlTypeFull::Typedef {
                            name: "MyDefinedEnum".to_string(),
                            repr: None,
                            content: Box::new(ToolboxIdlTypeFull::Typedef {
                                name: "MyEnum".to_string(),
                                repr: None,
                                content: Box::new(ToolboxIdlTypeFull::Enum {
                                    prefix: ToolboxIdlTypePrefix::U8,
                                    variants: vec![
                                        ToolboxIdlTypeFullEnumVariant {
                                            name: "CaseA".to_string(),
                                            code: 0,
                                            fields: ToolboxIdlTypeFullFields::Unnamed(
                                                vec![ToolboxIdlTypeFullFieldUnnamed {
                                                    content: ToolboxIdlTypeFull::Vec {
                                                        prefix:
                                                            ToolboxIdlTypePrefix::U32,
                                                        items: Box::new(
                                                            ToolboxIdlTypePrimitive::U8
                                                                .into()
                                                        )
                                                    }
                                                }]
                                            )
                                        },
                                        ToolboxIdlTypeFullEnumVariant {
                                            name: "CaseB".to_string(),
                                            code: 1,
                                            fields: ToolboxIdlTypeFullFields::Unnamed(
                                                vec![ToolboxIdlTypeFullFieldUnnamed {
                                                    content:
                                                        ToolboxIdlTypePrimitive::U8
                                                            .into()
                                                }]
                                            )
                                        },
                                    ]
                                })
                            })
                        }
                    },
                    ToolboxIdlTypeFullFieldUnnamed {
                        content: ToolboxIdlTypeFull::Typedef {
                            name: "MyDefinedStruct".to_string(),
                            repr: None,
                            content: Box::new(ToolboxIdlTypeFull::Typedef {
                            name: "MyStruct".to_string(),
                            repr: None,
                            content: Box::new(ToolboxIdlTypeFull::Struct {
                                fields: ToolboxIdlTypeFullFields::Named(vec![
                                    ToolboxIdlTypeFullFieldNamed {
                                        name: "field_a".to_string(),
                                        content: ToolboxIdlTypeFull::Option {
                                            prefix: ToolboxIdlTypePrefix::U8,
                                            content: Box::new(
                                                ToolboxIdlTypePrimitive::F64
                                                    .into()
                                            )
                                        }
                                    },
                                    ToolboxIdlTypeFullFieldNamed {
                                        name: "field_b".to_string(),
                                        content: ToolboxIdlTypeFull::Vec {
                                            prefix: ToolboxIdlTypePrefix::U32,
                                            items: Box::new(
                                                ToolboxIdlTypePrimitive::F32
                                                    .into()
                                            )
                                        },
                                    },
                                ])
                            })
                        })}
                    },
                    ToolboxIdlTypeFullFieldUnnamed {
                        content: ToolboxIdlTypeFull::Typedef {
                            name: "MyArray".to_string(),
                            repr: None,
                            content: Box::new(ToolboxIdlTypeFull::Array {
                            items: Box::new(ToolboxIdlTypePrimitive::I8.into()),
                            length: 4
                        })}
                    }
                ])
            }
        }
        .into()
    )
}
