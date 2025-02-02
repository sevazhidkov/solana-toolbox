use serde_json::json;
use solana_toolbox_idl::ToolboxIdl;
use solana_toolbox_idl::ToolboxIdlPrimitive;
use solana_toolbox_idl::ToolboxIdlProgramType;
use solana_toolbox_idl::ToolboxIdlTypeFlat;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl1 = ToolboxIdl::try_from_value(&json!({
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
                ]
            },
        },
    }))
    .unwrap();
    let idl2 = ToolboxIdl::try_from_value(&json!({
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
                ]
            },
        },
    }))
    .unwrap();
    // Asser that the two notations are equivalent
    assert_eq!(idl1, idl2);
    // Assert that the content is correct
    assert_eq!(
        idl1.program_types.get("MyStruct").unwrap(),
        &ToolboxIdlProgramType {
            name: "MyStruct".to_string(),
            generics: vec![],
            type_flat: solana_toolbox_idl::ToolboxIdlTypeFlat::Struct {
                fields: vec![
                    (
                        "0".to_string(),
                        ToolboxIdlTypeFlat::Primitive {
                            primitive: ToolboxIdlPrimitive::U8
                        }
                    ),
                    (
                        "1".to_string(),
                        ToolboxIdlTypeFlat::Primitive {
                            primitive: ToolboxIdlPrimitive::U64
                        }
                    ),
                    (
                        "2".to_string(),
                        ToolboxIdlTypeFlat::Primitive {
                            primitive: ToolboxIdlPrimitive::String
                        }
                    ),
                    (
                        "3".to_string(),
                        ToolboxIdlTypeFlat::Vec {
                            items: Box::new(ToolboxIdlTypeFlat::Primitive {
                                primitive: ToolboxIdlPrimitive::U8,
                            }),
                        }
                    ),
                    (
                        "4".to_string(),
                        ToolboxIdlTypeFlat::Vec {
                            items: Box::new(ToolboxIdlTypeFlat::Primitive {
                                primitive: ToolboxIdlPrimitive::U8,
                            }),
                        }
                    ),
                    (
                        "5".to_string(),
                        ToolboxIdlTypeFlat::Array {
                            items: Box::new(ToolboxIdlTypeFlat::Primitive {
                                primitive: ToolboxIdlPrimitive::U32,
                            }),
                            length: Box::new(ToolboxIdlTypeFlat::Const {
                                literal: 4
                            }),
                        }
                    ),
                    (
                        "6".to_string(),
                        ToolboxIdlTypeFlat::Array {
                            items: Box::new(ToolboxIdlTypeFlat::Primitive {
                                primitive: ToolboxIdlPrimitive::U32,
                            }),
                            length: Box::new(ToolboxIdlTypeFlat::Const {
                                literal: 4
                            }),
                        }
                    ),
                    (
                        "7".to_string(),
                        ToolboxIdlTypeFlat::Struct { fields: vec![] },
                    ),
                    (
                        "8".to_string(),
                        ToolboxIdlTypeFlat::Enum { variants: vec![] },
                    ),
                    (
                        "9".to_string(),
                        ToolboxIdlTypeFlat::Defined {
                            name: "Other".to_string(),
                            generics: vec![]
                        },
                    ),
                    (
                        "10".to_string(),
                        ToolboxIdlTypeFlat::Defined {
                            name: "Other".to_string(),
                            generics: vec![]
                        },
                    ),
                    (
                        "11".to_string(),
                        ToolboxIdlTypeFlat::Defined {
                            name: "Other".to_string(),
                            generics: vec![]
                        },
                    ),
                    (
                        "12".to_string(),
                        ToolboxIdlTypeFlat::Generic { symbol: "G".to_string() },
                    ),
                ]
            }
        }
    )
}
