use serde_json::json;
use solana_toolbox_idl::ToolboxIdlAccount;
use solana_toolbox_idl::ToolboxIdlProgram;
use solana_toolbox_idl::ToolboxIdlTypeFlat;
use solana_toolbox_idl::ToolboxIdlTypeFlatFieldUnamed;
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
                    ToolboxIdlTypeFlatFieldUnamed {
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Defined {
                            name: "MyDefinedEnum".to_string(),
                            generics: vec![ToolboxIdlTypeFlat::Primitive {
                                primitive: ToolboxIdlTypePrimitive::U8
                            }]
                        }
                    },
                    ToolboxIdlTypeFlatFieldUnamed {
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Defined {
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
                    },
                    ToolboxIdlTypeFlatFieldUnamed {
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Defined {
                            name: "MyArray".to_string(),
                            generics: vec![
                                ToolboxIdlTypeFlat::Primitive {
                                    primitive: ToolboxIdlTypePrimitive::I8
                                },
                                ToolboxIdlTypeFlat::Const { literal: 4 },
                            ]
                        }
                    },
                ])
            },
            content_type_full: ToolboxIdlTypeFull::Struct {
                fields: ToolboxIdlTypeFullFields::Unnamed(vec![
                    ToolboxIdlTypeFullFieldUnnamed {
                        type_full: ToolboxIdlTypeFull::Enum {
                            prefix: ToolboxIdlTypePrefix::U8,
                            variants: vec![
                                ToolboxIdlTypeFullEnumVariant {
                                    name: "CaseA".to_string(),
                                    code: 0,
                                    fields: ToolboxIdlTypeFullFields::Unnamed(
                                        vec![ToolboxIdlTypeFullFieldUnnamed {
                                            type_full: ToolboxIdlTypeFull::Vec {
                                                prefix: ToolboxIdlTypePrefix::U32,
                                                items: Box::new(
                                                    ToolboxIdlTypeFull::Primitive {
                                                        primitive:
                                                            ToolboxIdlTypePrimitive::U8
                                                    }
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
                                            type_full: ToolboxIdlTypeFull::Primitive {
                                                primitive: ToolboxIdlTypePrimitive::U8
                                            }
                                        }]
                                    )
                                },
                            ]
                        }
                    },
                    ToolboxIdlTypeFullFieldUnnamed {
                        type_full: ToolboxIdlTypeFull::Struct {
                            fields: ToolboxIdlTypeFullFields::Named(vec![
                                ToolboxIdlTypeFullFieldNamed {
                                    name: "field_a".to_string(),
                                    type_full: ToolboxIdlTypeFull::Option {
                                        prefix: ToolboxIdlTypePrefix::U8,
                                        content: Box::new(
                                            ToolboxIdlTypeFull::Primitive {
                                                primitive:
                                                    ToolboxIdlTypePrimitive::F64
                                            }
                                        )
                                    }
                                },
                                ToolboxIdlTypeFullFieldNamed {
                                    name: "field_b".to_string(),
                                    type_full: ToolboxIdlTypeFull::Vec {
                                        prefix: ToolboxIdlTypePrefix::U32,
                                        items: Box::new(
                                            ToolboxIdlTypeFull::Primitive {
                                                primitive:
                                                    ToolboxIdlTypePrimitive::F32
                                            }
                                        )
                                    },
                                },
                            ])
                        }
                    },
                    ToolboxIdlTypeFullFieldUnnamed {
                        type_full: ToolboxIdlTypeFull::Array {
                            items: Box::new(ToolboxIdlTypeFull::Primitive {
                                primitive: ToolboxIdlTypePrimitive::I8
                            }),
                            length: 4
                        }
                    }
                ])
            }
        }
        .into()
    )
}
