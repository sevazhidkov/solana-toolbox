use serde_json::json;
use solana_toolbox_idl::ToolboxIdlProgram;
use solana_toolbox_idl::ToolboxIdlTypeFlat;
use solana_toolbox_idl::ToolboxIdlTypeFlatFieldNamed;
use solana_toolbox_idl::ToolboxIdlTypeFlatFields;
use solana_toolbox_idl::ToolboxIdlTypePrefix;
use solana_toolbox_idl::ToolboxIdlTypePrimitive;
use solana_toolbox_idl::ToolboxIdlTypedef;

#[tokio::test]
pub async fn run() {
    // Create IDL checking different formats
    let idl_program = ToolboxIdlProgram::try_parse_from_value(&json!({
        "types": {
            "MyStruct": {
                "fields": [
                    { "name": "u8", "type": "u8" },
                    { "name": "u64", "type": "u64" },
                    { "name": "string", "type": "string" },
                    { "name": "vec1_u8", "type": ["u8"] },
                    { "name": "vec2_u8", "type": {"vec": "u8"} },
                    { "name": "vec1_vec_u8", "type": [["u8"]] },
                    { "name": "vec2_vec_u8", "type": [{ "vec": "u8" }] },
                    { "name": "array1_u32_4", "type": ["u32", 4] },
                    { "name": "array2_u32_4", "type": {"array": ["u32", 4]} },
                    { "name": "struct1", "type" : {"fields": []} },
                    { "name": "struct2", "fields": [] },
                    { "name": "enum1", "type" : {"variants": []} },
                    { "name": "enum2", "variants": [] },
                    { "name": "defined1", "defined": "Other" },
                    { "name": "defined2", "defined": {"name": "Other"} },
                    { "name": "defined3", "type": {"defined": "Other"} },
                    { "name": "defined4", "type": {"defined": {"name": "Other"}} },
                    { "name": "option_1_f32", "option": "f32" },
                    { "name": "option_2_f32", "type": {"option": "f32"} },
                    { "name": "generic1", "generic": "G" },
                    { "name": "generic2", "type": {"generic": "G"} },
                    { "name": "docs", "type": "u8", "docs": ["Hello"] },
                ]
            },
        },
    }))
    .unwrap();
    // Assert that the content is correct
    assert_eq!(
        *idl_program.typedefs.get("MyStruct").unwrap(),
        ToolboxIdlTypedef {
            name: "MyStruct".to_string(),
            docs: None,
            repr: None,
            generics: vec![],
            type_flat: ToolboxIdlTypeFlat::Struct {
                fields: ToolboxIdlTypeFlatFields::Named(vec![
                    ToolboxIdlTypeFlatFieldNamed {
                        name: "u8".to_string(),
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Primitive {
                            primitive: ToolboxIdlTypePrimitive::U8
                        }
                    },
                    ToolboxIdlTypeFlatFieldNamed {
                        name: "u64".to_string(),
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Primitive {
                            primitive: ToolboxIdlTypePrimitive::U64
                        }
                    },
                    ToolboxIdlTypeFlatFieldNamed {
                        name: "string".to_string(),
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Primitive {
                            primitive: ToolboxIdlTypePrimitive::String
                        }
                    },
                    ToolboxIdlTypeFlatFieldNamed {
                        name: "vec1_u8".to_string(),
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Vec {
                            prefix: ToolboxIdlTypePrefix::U32,
                            items: Box::new(ToolboxIdlTypeFlat::Primitive {
                                primitive: ToolboxIdlTypePrimitive::U8,
                            }),
                        }
                    },
                    ToolboxIdlTypeFlatFieldNamed {
                        name: "vec2_u8".to_string(),
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Vec {
                            prefix: ToolboxIdlTypePrefix::U32,
                            items: Box::new(ToolboxIdlTypeFlat::Primitive {
                                primitive: ToolboxIdlTypePrimitive::U8,
                            }),
                        }
                    },
                    ToolboxIdlTypeFlatFieldNamed {
                        name: "vec1_vec_u8".to_string(),
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Vec {
                            prefix: ToolboxIdlTypePrefix::U32,
                            items: Box::new(ToolboxIdlTypeFlat::Vec {
                                prefix: ToolboxIdlTypePrefix::U32,
                                items: Box::new(
                                    ToolboxIdlTypeFlat::Primitive {
                                        primitive: ToolboxIdlTypePrimitive::U8,
                                    }
                                ),
                            }),
                        }
                    },
                    ToolboxIdlTypeFlatFieldNamed {
                        name: "vec2_vec_u8".to_string(),
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Vec {
                            prefix: ToolboxIdlTypePrefix::U32,
                            items: Box::new(ToolboxIdlTypeFlat::Vec {
                                prefix: ToolboxIdlTypePrefix::U32,
                                items: Box::new(
                                    ToolboxIdlTypeFlat::Primitive {
                                        primitive: ToolboxIdlTypePrimitive::U8,
                                    }
                                ),
                            }),
                        }
                    },
                    ToolboxIdlTypeFlatFieldNamed {
                        name: "array1_u32_4".to_string(),
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Array {
                            items: Box::new(ToolboxIdlTypeFlat::Primitive {
                                primitive: ToolboxIdlTypePrimitive::U32,
                            }),
                            length: Box::new(ToolboxIdlTypeFlat::Const {
                                literal: 4
                            }),
                        }
                    },
                    ToolboxIdlTypeFlatFieldNamed {
                        name: "array2_u32_4".to_string(),
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Array {
                            items: Box::new(ToolboxIdlTypeFlat::Primitive {
                                primitive: ToolboxIdlTypePrimitive::U32,
                            }),
                            length: Box::new(ToolboxIdlTypeFlat::Const {
                                literal: 4
                            }),
                        }
                    },
                    ToolboxIdlTypeFlatFieldNamed {
                        name: "struct1".to_string(),
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Struct {
                            fields: ToolboxIdlTypeFlatFields::None
                        }
                    },
                    ToolboxIdlTypeFlatFieldNamed {
                        name: "struct2".to_string(),
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Struct {
                            fields: ToolboxIdlTypeFlatFields::None
                        }
                    },
                    ToolboxIdlTypeFlatFieldNamed {
                        name: "enum1".to_string(),
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Enum {
                            prefix: ToolboxIdlTypePrefix::U8,
                            variants: vec![]
                        },
                    },
                    ToolboxIdlTypeFlatFieldNamed {
                        name: "enum2".to_string(),
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Enum {
                            prefix: ToolboxIdlTypePrefix::U8,
                            variants: vec![]
                        }
                    },
                    ToolboxIdlTypeFlatFieldNamed {
                        name: "defined1".to_string(),
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Defined {
                            name: "Other".to_string(),
                            generics: vec![]
                        }
                    },
                    ToolboxIdlTypeFlatFieldNamed {
                        name: "defined2".to_string(),
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Defined {
                            name: "Other".to_string(),
                            generics: vec![]
                        }
                    },
                    ToolboxIdlTypeFlatFieldNamed {
                        name: "defined3".to_string(),
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Defined {
                            name: "Other".to_string(),
                            generics: vec![]
                        }
                    },
                    ToolboxIdlTypeFlatFieldNamed {
                        name: "defined4".to_string(),
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Defined {
                            name: "Other".to_string(),
                            generics: vec![]
                        }
                    },
                    ToolboxIdlTypeFlatFieldNamed {
                        name: "option_1_f32".to_string(),
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Option {
                            prefix: ToolboxIdlTypePrefix::U8,
                            content: Box::new(ToolboxIdlTypeFlat::Primitive {
                                primitive: ToolboxIdlTypePrimitive::F32,
                            })
                        }
                    },
                    ToolboxIdlTypeFlatFieldNamed {
                        name: "option_2_f32".to_string(),
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Option {
                            prefix: ToolboxIdlTypePrefix::U8,
                            content: Box::new(ToolboxIdlTypeFlat::Primitive {
                                primitive: ToolboxIdlTypePrimitive::F32,
                            })
                        }
                    },
                    ToolboxIdlTypeFlatFieldNamed {
                        name: "generic1".to_string(),
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Generic {
                            symbol: "G".to_string()
                        }
                    },
                    ToolboxIdlTypeFlatFieldNamed {
                        name: "generic2".to_string(),
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Generic {
                            symbol: "G".to_string()
                        }
                    },
                    ToolboxIdlTypeFlatFieldNamed {
                        name: "docs".to_string(),
                        docs: Some(json!(["Hello"])),
                        type_flat: ToolboxIdlTypeFlat::Primitive {
                            primitive: ToolboxIdlTypePrimitive::U8
                        }
                    }
                ])
            }
        }
        .into()
    )
}
