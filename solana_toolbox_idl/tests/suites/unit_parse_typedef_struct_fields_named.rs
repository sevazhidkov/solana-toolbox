use serde_json::json;
use solana_toolbox_idl::ToolboxIdlProgram;
use solana_toolbox_idl::ToolboxIdlTypeFlat;
use solana_toolbox_idl::ToolboxIdlTypeFlatFields;
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
            generics: vec![],
            type_flat: ToolboxIdlTypeFlat::Struct {
                fields: ToolboxIdlTypeFlatFields::Named(vec![
                    (
                        "u8".to_string(),
                        None,
                        ToolboxIdlTypeFlat::Primitive {
                            primitive: ToolboxIdlTypePrimitive::U8
                        }
                    ),
                    (
                        "u64".to_string(),
                        None,
                        ToolboxIdlTypeFlat::Primitive {
                            primitive: ToolboxIdlTypePrimitive::U64
                        }
                    ),
                    (
                        "string".to_string(),
                        None,
                        ToolboxIdlTypeFlat::Primitive {
                            primitive: ToolboxIdlTypePrimitive::String
                        }
                    ),
                    (
                        "vec1_u8".to_string(),
                        None,
                        ToolboxIdlTypeFlat::Vec {
                            prefix_bytes: 4,
                            items: Box::new(ToolboxIdlTypeFlat::Primitive {
                                primitive: ToolboxIdlTypePrimitive::U8,
                            }),
                        }
                    ),
                    (
                        "vec2_u8".to_string(),
                        None,
                        ToolboxIdlTypeFlat::Vec {
                            prefix_bytes: 4,
                            items: Box::new(ToolboxIdlTypeFlat::Primitive {
                                primitive: ToolboxIdlTypePrimitive::U8,
                            }),
                        }
                    ),
                    (
                        "vec1_vec_u8".to_string(),
                        None,
                        ToolboxIdlTypeFlat::Vec {
                            prefix_bytes: 4,
                            items: Box::new(ToolboxIdlTypeFlat::Vec {
                                prefix_bytes: 4,
                                items: Box::new(
                                    ToolboxIdlTypeFlat::Primitive {
                                        primitive: ToolboxIdlTypePrimitive::U8,
                                    }
                                ),
                            }),
                        }
                    ),
                    (
                        "vec2_vec_u8".to_string(),
                        None,
                        ToolboxIdlTypeFlat::Vec {
                            prefix_bytes: 4,
                            items: Box::new(ToolboxIdlTypeFlat::Vec {
                                prefix_bytes: 4,
                                items: Box::new(
                                    ToolboxIdlTypeFlat::Primitive {
                                        primitive: ToolboxIdlTypePrimitive::U8,
                                    }
                                ),
                            }),
                        }
                    ),
                    (
                        "array1_u32_4".to_string(),
                        None,
                        ToolboxIdlTypeFlat::Array {
                            items: Box::new(ToolboxIdlTypeFlat::Primitive {
                                primitive: ToolboxIdlTypePrimitive::U32,
                            }),
                            length: Box::new(ToolboxIdlTypeFlat::Const {
                                literal: 4
                            }),
                        }
                    ),
                    (
                        "array2_u32_4".to_string(),
                        None,
                        ToolboxIdlTypeFlat::Array {
                            items: Box::new(ToolboxIdlTypeFlat::Primitive {
                                primitive: ToolboxIdlTypePrimitive::U32,
                            }),
                            length: Box::new(ToolboxIdlTypeFlat::Const {
                                literal: 4
                            }),
                        }
                    ),
                    (
                        "struct1".to_string(),
                        None,
                        ToolboxIdlTypeFlat::Struct {
                            fields: ToolboxIdlTypeFlatFields::None
                        }
                    ),
                    (
                        "struct2".to_string(),
                        None,
                        ToolboxIdlTypeFlat::Struct {
                            fields: ToolboxIdlTypeFlatFields::None
                        }
                    ),
                    (
                        "enum1".to_string(),
                        None,
                        ToolboxIdlTypeFlat::Enum {
                            prefix_bytes: 1,
                            variants: vec![]
                        },
                    ),
                    (
                        "enum2".to_string(),
                        None,
                        ToolboxIdlTypeFlat::Enum {
                            prefix_bytes: 1,
                            variants: vec![]
                        }
                    ),
                    (
                        "defined1".to_string(),
                        None,
                        ToolboxIdlTypeFlat::Defined {
                            name: "Other".to_string(),
                            generics: vec![]
                        }
                    ),
                    (
                        "defined2".to_string(),
                        None,
                        ToolboxIdlTypeFlat::Defined {
                            name: "Other".to_string(),
                            generics: vec![]
                        }
                    ),
                    (
                        "defined3".to_string(),
                        None,
                        ToolboxIdlTypeFlat::Defined {
                            name: "Other".to_string(),
                            generics: vec![]
                        }
                    ),
                    (
                        "defined4".to_string(),
                        None,
                        ToolboxIdlTypeFlat::Defined {
                            name: "Other".to_string(),
                            generics: vec![]
                        }
                    ),
                    (
                        "option_1_f32".to_string(),
                        None,
                        ToolboxIdlTypeFlat::Option {
                            prefix_bytes: 1,
                            content: Box::new(ToolboxIdlTypeFlat::Primitive {
                                primitive: ToolboxIdlTypePrimitive::F32,
                            })
                        }
                    ),
                    (
                        "option_2_f32".to_string(),
                        None,
                        ToolboxIdlTypeFlat::Option {
                            prefix_bytes: 1,
                            content: Box::new(ToolboxIdlTypeFlat::Primitive {
                                primitive: ToolboxIdlTypePrimitive::F32,
                            })
                        }
                    ),
                    (
                        "generic1".to_string(),
                        None,
                        ToolboxIdlTypeFlat::Generic {
                            symbol: "G".to_string()
                        }
                    ),
                    (
                        "generic2".to_string(),
                        None,
                        ToolboxIdlTypeFlat::Generic {
                            symbol: "G".to_string()
                        }
                    ),
                    (
                        "docs".to_string(),
                        Some(json!(["Hello"])),
                        ToolboxIdlTypeFlat::Primitive {
                            primitive: ToolboxIdlTypePrimitive::U8
                        }
                    )
                ])
            }
        }
        .into()
    )
}
