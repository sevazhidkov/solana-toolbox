use serde_json::json;
use solana_toolbox_idl::ToolboxIdlProgramRoot;
use solana_toolbox_idl::ToolboxIdlProgramAccount;
use solana_toolbox_idl::ToolboxIdlProgramTypeFlat;
use solana_toolbox_idl::ToolboxIdlProgramTypeFlatFields;
use solana_toolbox_idl::ToolboxIdlProgramTypeFull;
use solana_toolbox_idl::ToolboxIdlProgramTypeFullFields;
use solana_toolbox_idl::ToolboxIdlProgramTypePrimitive;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl1 = ToolboxIdlProgramRoot::try_parse_from_value(&json!({
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
        idl1.accounts.get("MyAccount").unwrap(),
        &ToolboxIdlProgramAccount {
            name: "MyAccount".to_string(),
            discriminator: vec![77],
            data_type_flat: ToolboxIdlProgramTypeFlat::Struct {
                fields: ToolboxIdlProgramTypeFlatFields::Unamed(vec![
                    ToolboxIdlProgramTypeFlat::Defined {
                        name: "MyDefinedEnum".to_string(),
                        generics: vec![ToolboxIdlProgramTypeFlat::Primitive {
                            primitive: ToolboxIdlProgramTypePrimitive::U8
                        }]
                    },
                    ToolboxIdlProgramTypeFlat::Defined {
                        name: "MyDefinedStruct".to_string(),
                        generics: vec![
                            ToolboxIdlProgramTypeFlat::Primitive {
                                primitive: ToolboxIdlProgramTypePrimitive::F32
                            },
                            ToolboxIdlProgramTypeFlat::Primitive {
                                primitive: ToolboxIdlProgramTypePrimitive::F64
                            },
                        ]
                    },
                    ToolboxIdlProgramTypeFlat::Defined {
                        name: "MyArray".to_string(),
                        generics: vec![
                            ToolboxIdlProgramTypeFlat::Primitive {
                                primitive: ToolboxIdlProgramTypePrimitive::I8
                            },
                            ToolboxIdlProgramTypeFlat::Const { literal: 4 },
                        ]
                    },
                ])
            },
            data_type_full: ToolboxIdlProgramTypeFull::Struct {
                fields: ToolboxIdlProgramTypeFullFields::Unamed(vec![
                    ToolboxIdlProgramTypeFull::Enum {
                        variants: vec![
                            (
                                "CaseA".to_string(),
                                ToolboxIdlProgramTypeFullFields::Unamed(vec![
                                    ToolboxIdlProgramTypeFull::Vec {
                                        items: Box::new(
                                            ToolboxIdlProgramTypeFull::Primitive {
                                                primitive:
                                                    ToolboxIdlProgramTypePrimitive::U8
                                            }
                                        )
                                    },
                                ])
                            ),
                            (
                                "CaseB".to_string(),
                                ToolboxIdlProgramTypeFullFields::Unamed(vec![
                                    ToolboxIdlProgramTypeFull::Primitive {
                                        primitive: ToolboxIdlProgramTypePrimitive::U8
                                    }
                                ])
                            ),
                        ]
                    },
                    ToolboxIdlProgramTypeFull::Struct {
                        fields: ToolboxIdlProgramTypeFullFields::Named(vec![
                            (
                                "field_a".to_string(),
                                ToolboxIdlProgramTypeFull::Option {
                                    content: Box::new(
                                        ToolboxIdlProgramTypeFull::Primitive {
                                            primitive:
                                                ToolboxIdlProgramTypePrimitive::F64
                                        }
                                    )
                                }
                            ),
                            (
                                "field_b".to_string(),
                                ToolboxIdlProgramTypeFull::Vec {
                                    items: Box::new(
                                        ToolboxIdlProgramTypeFull::Primitive {
                                            primitive:
                                                ToolboxIdlProgramTypePrimitive::F32
                                        }
                                    )
                                },
                            ),
                        ])
                    },
                    ToolboxIdlProgramTypeFull::Array {
                        items: Box::new(ToolboxIdlProgramTypeFull::Primitive {
                            primitive: ToolboxIdlProgramTypePrimitive::I8
                        }),
                        length: 4
                    }
                ])
            }
        }
    )
}
