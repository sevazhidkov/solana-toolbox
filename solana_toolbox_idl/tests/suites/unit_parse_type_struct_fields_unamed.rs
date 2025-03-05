use serde_json::json;
use solana_toolbox_idl::ToolboxIdl;
use solana_toolbox_idl::ToolboxIdlProgramType;
use solana_toolbox_idl::ToolboxIdlTypeFlat;
use solana_toolbox_idl::ToolboxIdlTypeFlatFields;
use solana_toolbox_idl::ToolboxIdlTypePrimitive;

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
            type_flat: ToolboxIdlTypeFlat::Struct {
                fields: ToolboxIdlTypeFlatFields::Unamed(vec![
                    ToolboxIdlTypeFlat::Primitive {
                        primitive: ToolboxIdlTypePrimitive::U8
                    },
                    ToolboxIdlTypeFlat::Primitive {
                        primitive: ToolboxIdlTypePrimitive::U64
                    },
                    ToolboxIdlTypeFlat::Primitive {
                        primitive: ToolboxIdlTypePrimitive::String
                    },
                    ToolboxIdlTypeFlat::Vec {
                        items: Box::new(ToolboxIdlTypeFlat::Primitive {
                            primitive: ToolboxIdlTypePrimitive::U8,
                        }),
                    },
                    ToolboxIdlTypeFlat::Vec {
                        items: Box::new(ToolboxIdlTypeFlat::Primitive {
                            primitive: ToolboxIdlTypePrimitive::U8,
                        }),
                    },
                    ToolboxIdlTypeFlat::Array {
                        items: Box::new(ToolboxIdlTypeFlat::Primitive {
                            primitive: ToolboxIdlTypePrimitive::U32,
                        }),
                        length: Box::new(ToolboxIdlTypeFlat::Const {
                            literal: 4
                        }),
                    },
                    ToolboxIdlTypeFlat::Array {
                        items: Box::new(ToolboxIdlTypeFlat::Primitive {
                            primitive: ToolboxIdlTypePrimitive::U32,
                        }),
                        length: Box::new(ToolboxIdlTypeFlat::Const {
                            literal: 4
                        }),
                    },
                    ToolboxIdlTypeFlat::Struct {
                        fields: ToolboxIdlTypeFlatFields::None
                    },
                    ToolboxIdlTypeFlat::Enum { variants: vec![] },
                    ToolboxIdlTypeFlat::Defined {
                        name: "Other".to_string(),
                        generics: vec![]
                    },
                    ToolboxIdlTypeFlat::Defined {
                        name: "Other".to_string(),
                        generics: vec![]
                    },
                    ToolboxIdlTypeFlat::Defined {
                        name: "Other".to_string(),
                        generics: vec![]
                    },
                    ToolboxIdlTypeFlat::Generic {
                        symbol: "G".to_string()
                    },
                ])
            }
        }
    )
}
