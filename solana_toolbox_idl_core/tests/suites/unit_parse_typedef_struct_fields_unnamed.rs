use serde_json::json;
use solana_toolbox_idl_core::ToolboxIdlProgram;
use solana_toolbox_idl_core::ToolboxIdlTypeFlat;
use solana_toolbox_idl_core::ToolboxIdlTypeFlatFieldUnnamed;
use solana_toolbox_idl_core::ToolboxIdlTypeFlatFields;
use solana_toolbox_idl_core::ToolboxIdlTypePrefix;
use solana_toolbox_idl_core::ToolboxIdlTypePrimitive;
use solana_toolbox_idl_core::ToolboxIdlTypedef;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl_program1 = ToolboxIdlProgram::try_parse(&json!({
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
    let idl_program2 = ToolboxIdlProgram::try_parse(&json!({
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
            serialization: None,
            repr: None,
            generics: vec![],
            type_flat: ToolboxIdlTypeFlat::Struct {
                fields: ToolboxIdlTypeFlatFields::Unnamed(vec![
                    ToolboxIdlTypeFlatFieldUnnamed {
                        docs: None,
                        content: ToolboxIdlTypePrimitive::U8.into()
                    },
                    ToolboxIdlTypeFlatFieldUnnamed {
                        docs: None,
                        content: ToolboxIdlTypePrimitive::U64.into()
                    },
                    ToolboxIdlTypeFlatFieldUnnamed {
                        docs: None,
                        content: ToolboxIdlTypeFlat::String {
                            prefix: ToolboxIdlTypePrefix::U32
                        }
                    },
                    ToolboxIdlTypeFlatFieldUnnamed {
                        docs: None,
                        content: ToolboxIdlTypeFlat::Vec {
                            prefix: ToolboxIdlTypePrefix::U32,
                            items: Box::new(ToolboxIdlTypePrimitive::U8.into()),
                        }
                    },
                    ToolboxIdlTypeFlatFieldUnnamed {
                        docs: None,
                        content: ToolboxIdlTypeFlat::Vec {
                            prefix: ToolboxIdlTypePrefix::U32,
                            items: Box::new(ToolboxIdlTypePrimitive::U8.into()),
                        }
                    },
                    ToolboxIdlTypeFlatFieldUnnamed {
                        docs: None,
                        content: ToolboxIdlTypeFlat::Array {
                            items: Box::new(
                                ToolboxIdlTypePrimitive::U32.into()
                            ),
                            length: Box::new(ToolboxIdlTypeFlat::Const {
                                literal: 4
                            }),
                        }
                    },
                    ToolboxIdlTypeFlatFieldUnnamed {
                        docs: None,
                        content: ToolboxIdlTypeFlat::Array {
                            items: Box::new(
                                ToolboxIdlTypePrimitive::U32.into()
                            ),
                            length: Box::new(ToolboxIdlTypeFlat::Const {
                                literal: 4
                            }),
                        }
                    },
                    ToolboxIdlTypeFlatFieldUnnamed {
                        docs: None,
                        content: ToolboxIdlTypeFlat::Struct {
                            fields: ToolboxIdlTypeFlatFields::nothing()
                        }
                    },
                    ToolboxIdlTypeFlatFieldUnnamed {
                        docs: None,
                        content: ToolboxIdlTypeFlat::Enum {
                            prefix: ToolboxIdlTypePrefix::U8,
                            variants: vec![]
                        }
                    },
                    ToolboxIdlTypeFlatFieldUnnamed {
                        docs: None,
                        content: ToolboxIdlTypeFlat::Defined {
                            name: "Other".to_string(),
                            generics: vec![]
                        }
                    },
                    ToolboxIdlTypeFlatFieldUnnamed {
                        docs: None,
                        content: ToolboxIdlTypeFlat::Defined {
                            name: "Other".to_string(),
                            generics: vec![]
                        }
                    },
                    ToolboxIdlTypeFlatFieldUnnamed {
                        docs: None,
                        content: ToolboxIdlTypeFlat::Defined {
                            name: "Other".to_string(),
                            generics: vec![]
                        }
                    },
                    ToolboxIdlTypeFlatFieldUnnamed {
                        docs: None,
                        content: ToolboxIdlTypeFlat::Generic {
                            symbol: "G".to_string()
                        }
                    },
                    ToolboxIdlTypeFlatFieldUnnamed {
                        docs: None,
                        content: ToolboxIdlTypeFlat::Option {
                            prefix: ToolboxIdlTypePrefix::U8,
                            content: Box::new(
                                ToolboxIdlTypePrimitive::U8.into()
                            ),
                        }
                    },
                    ToolboxIdlTypeFlatFieldUnnamed {
                        docs: None,
                        content: ToolboxIdlTypeFlat::Option {
                            prefix: ToolboxIdlTypePrefix::U32,
                            content: Box::new(
                                ToolboxIdlTypePrimitive::U8.into()
                            ),
                        }
                    },
                    ToolboxIdlTypeFlatFieldUnnamed {
                        docs: Some(json!(["Hello"])),
                        content: ToolboxIdlTypeFlat::Struct {
                            fields: ToolboxIdlTypeFlatFields::nothing()
                        }
                    },
                ]),
            }
        }
        .into()
    )
}
