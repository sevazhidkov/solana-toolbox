use serde_json::json;
use solana_toolbox_idl::ToolboxIdl;
use solana_toolbox_idl::ToolboxIdlProgramAccount;
use solana_toolbox_idl::ToolboxIdlTypeFlat;
use solana_toolbox_idl::ToolboxIdlTypeFlatFields;
use solana_toolbox_idl::ToolboxIdlTypeFull;
use solana_toolbox_idl::ToolboxIdlTypeFullFields;
use solana_toolbox_idl::ToolboxIdlTypePrimitive;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl1 = ToolboxIdl::try_from_value(&json!({
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
        idl1.program_accounts.get("MyAccount").unwrap(),
        &ToolboxIdlProgramAccount {
            name: "MyAccount".to_string(),
            discriminator: vec![77],
            data_type_flat: ToolboxIdlTypeFlat::Struct {
                fields: ToolboxIdlTypeFlatFields::Unamed(vec![
                    ToolboxIdlTypeFlat::Defined {
                        name: "MyDefinedEnum".to_string(),
                        generics: vec![ToolboxIdlTypeFlat::Primitive {
                            primitive: ToolboxIdlTypePrimitive::U8
                        }]
                    },
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
                    },
                    ToolboxIdlTypeFlat::Defined {
                        name: "MyArray".to_string(),
                        generics: vec![
                            ToolboxIdlTypeFlat::Primitive {
                                primitive: ToolboxIdlTypePrimitive::I8
                            },
                            ToolboxIdlTypeFlat::Const { literal: 4 },
                        ]
                    },
                ])
            },
            data_type_full: ToolboxIdlTypeFull::Struct {
                fields: ToolboxIdlTypeFullFields::Unamed(vec![
                    ToolboxIdlTypeFull::Enum {
                        variants: vec![
                            (
                                "CaseA".to_string(),
                                ToolboxIdlTypeFullFields::Unamed(vec![
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
                                ToolboxIdlTypeFullFields::Unamed(vec![
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
    )
}
