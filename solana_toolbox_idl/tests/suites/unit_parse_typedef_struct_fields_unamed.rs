use serde_json::json;
use solana_toolbox_idl::ToolboxIdlProgram;
use solana_toolbox_idl::ToolboxIdlTypeFlat;
use solana_toolbox_idl::ToolboxIdlTypeFlatFieldUnamed;
use solana_toolbox_idl::ToolboxIdlTypeFlatFields;
use solana_toolbox_idl::ToolboxIdlTypePrefix;
use solana_toolbox_idl::ToolboxIdlTypePrimitive;
use solana_toolbox_idl::ToolboxIdlTypedef;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl_program1 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "types": {
            "MyStruct": {
                "fields": [
                    { "type": "u8" },
                    { "type": "u64" },
                    { "type": "string" },
                    { "type": ["u8"] },
                    { "type": {"vec": "u8"} },
                    { "type": ["u32", 4] },
                    { "type": {"array": ["u32", 4]} },
                    { "type": {"fields": []} },
                    { "type": {"variants": []} },
                    { "type": "Other" },
                    { "type": {"defined": "Other"} },
                    { "type": {"defined": {"name": "Other"}} },
                    { "type": {"generic": "G"} },
                    { "type": {"option": "u8"} },
                    { "type": {"option32": "u8"} },
                    { "type": {"fields": []}, "docs": ["Hello"] },
                ]
            },
        },
    }))
    .unwrap();
    let idl_program2 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "types": {
            "MyStruct": {
                "fields": [
                    "u8",
                    "u64",
                    "string",
                    ["u8"],
                    {"vec": "u8"},
                    ["u32", 4],
                    {"array": ["u32", 4]},
                    {"fields": []},
                    {"variants": []},
                    "Other",
                    {"defined": "Other"},
                    {"defined": {"name": "Other"}},
                    {"generic": "G"},
                    {"option": "u8"},
                    {"option32": "u8"},
                    { "docs": ["Hello"], "fields": [] },
                ]
            },
        },
    }))
    .unwrap();
    // Asser that the two notations are equivalent
    assert_eq!(idl_program1, idl_program2);
    // Assert that the content is correct
    assert_eq!(
        *idl_program1.typedefs.get("MyStruct").unwrap(),
        ToolboxIdlTypedef {
            name: "MyStruct".to_string(),
            docs: None,
            repr: None,
            generics: vec![],
            type_flat: ToolboxIdlTypeFlat::Struct {
                fields: ToolboxIdlTypeFlatFields::Unnamed(vec![
                    ToolboxIdlTypeFlatFieldUnamed {
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Primitive {
                            primitive: ToolboxIdlTypePrimitive::U8
                        }
                    },
                    ToolboxIdlTypeFlatFieldUnamed {
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Primitive {
                            primitive: ToolboxIdlTypePrimitive::U64
                        }
                    },
                    ToolboxIdlTypeFlatFieldUnamed {
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Primitive {
                            primitive: ToolboxIdlTypePrimitive::String
                        }
                    },
                    ToolboxIdlTypeFlatFieldUnamed {
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Vec {
                            prefix: ToolboxIdlTypePrefix::U32,
                            items: Box::new(ToolboxIdlTypeFlat::Primitive {
                                primitive: ToolboxIdlTypePrimitive::U8,
                            }),
                        }
                    },
                    ToolboxIdlTypeFlatFieldUnamed {
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Vec {
                            prefix: ToolboxIdlTypePrefix::U32,
                            items: Box::new(ToolboxIdlTypeFlat::Primitive {
                                primitive: ToolboxIdlTypePrimitive::U8,
                            }),
                        }
                    },
                    ToolboxIdlTypeFlatFieldUnamed {
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
                    ToolboxIdlTypeFlatFieldUnamed {
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
                    ToolboxIdlTypeFlatFieldUnamed {
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Struct {
                            fields: ToolboxIdlTypeFlatFields::None
                        }
                    },
                    ToolboxIdlTypeFlatFieldUnamed {
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Enum {
                            prefix: ToolboxIdlTypePrefix::U8,
                            variants: vec![]
                        }
                    },
                    ToolboxIdlTypeFlatFieldUnamed {
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Defined {
                            name: "Other".to_string(),
                            generics: vec![]
                        }
                    },
                    ToolboxIdlTypeFlatFieldUnamed {
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Defined {
                            name: "Other".to_string(),
                            generics: vec![]
                        }
                    },
                    ToolboxIdlTypeFlatFieldUnamed {
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Defined {
                            name: "Other".to_string(),
                            generics: vec![]
                        }
                    },
                    ToolboxIdlTypeFlatFieldUnamed {
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Generic {
                            symbol: "G".to_string()
                        }
                    },
                    ToolboxIdlTypeFlatFieldUnamed {
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Option {
                            prefix: ToolboxIdlTypePrefix::U8,
                            content: Box::new(ToolboxIdlTypeFlat::Primitive {
                                primitive: ToolboxIdlTypePrimitive::U8,
                            }),
                        }
                    },
                    ToolboxIdlTypeFlatFieldUnamed {
                        docs: None,
                        type_flat: ToolboxIdlTypeFlat::Option {
                            prefix: ToolboxIdlTypePrefix::U32,
                            content: Box::new(ToolboxIdlTypeFlat::Primitive {
                                primitive: ToolboxIdlTypePrimitive::U8,
                            }),
                        }
                    },
                    ToolboxIdlTypeFlatFieldUnamed {
                        docs: Some(json!(["Hello"])),
                        type_flat: ToolboxIdlTypeFlat::Struct {
                            fields: ToolboxIdlTypeFlatFields::None
                        }
                    },
                ]),
            }
        }
        .into()
    )
}
