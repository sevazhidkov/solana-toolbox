use serde_json::json;
use solana_toolbox_idl::ToolboxIdl;
use solana_toolbox_idl::ToolboxIdlProgramTypedef;
use solana_toolbox_idl::ToolboxIdlProgramTypeFlat;
use solana_toolbox_idl::ToolboxIdlProgramTypeFlatFields;
use solana_toolbox_idl::ToolboxIdlProgramTypePrimitive;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl1 = ToolboxIdl::try_parse_from_value(&json!({
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
    let idl2 = ToolboxIdl::try_parse_from_value(&json!({
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
        idl1.program_typedefs.get("MyStruct").unwrap(),
        &ToolboxIdlProgramTypedef {
            name: "MyStruct".to_string(),
            generics: vec![],
            type_flat: ToolboxIdlProgramTypeFlat::Struct {
                fields: ToolboxIdlProgramTypeFlatFields::Unamed(vec![
                    ToolboxIdlProgramTypeFlat::Primitive {
                        primitive: ToolboxIdlProgramTypePrimitive::U8
                    },
                    ToolboxIdlProgramTypeFlat::Primitive {
                        primitive: ToolboxIdlProgramTypePrimitive::U64
                    },
                    ToolboxIdlProgramTypeFlat::Primitive {
                        primitive: ToolboxIdlProgramTypePrimitive::String
                    },
                    ToolboxIdlProgramTypeFlat::Vec {
                        items: Box::new(ToolboxIdlProgramTypeFlat::Primitive {
                            primitive: ToolboxIdlProgramTypePrimitive::U8,
                        }),
                    },
                    ToolboxIdlProgramTypeFlat::Vec {
                        items: Box::new(ToolboxIdlProgramTypeFlat::Primitive {
                            primitive: ToolboxIdlProgramTypePrimitive::U8,
                        }),
                    },
                    ToolboxIdlProgramTypeFlat::Array {
                        items: Box::new(ToolboxIdlProgramTypeFlat::Primitive {
                            primitive: ToolboxIdlProgramTypePrimitive::U32,
                        }),
                        length: Box::new(ToolboxIdlProgramTypeFlat::Const {
                            literal: 4
                        }),
                    },
                    ToolboxIdlProgramTypeFlat::Array {
                        items: Box::new(ToolboxIdlProgramTypeFlat::Primitive {
                            primitive: ToolboxIdlProgramTypePrimitive::U32,
                        }),
                        length: Box::new(ToolboxIdlProgramTypeFlat::Const {
                            literal: 4
                        }),
                    },
                    ToolboxIdlProgramTypeFlat::Struct {
                        fields: ToolboxIdlProgramTypeFlatFields::None
                    },
                    ToolboxIdlProgramTypeFlat::Enum { variants: vec![] },
                    ToolboxIdlProgramTypeFlat::Defined {
                        name: "Other".to_string(),
                        generics: vec![]
                    },
                    ToolboxIdlProgramTypeFlat::Defined {
                        name: "Other".to_string(),
                        generics: vec![]
                    },
                    ToolboxIdlProgramTypeFlat::Defined {
                        name: "Other".to_string(),
                        generics: vec![]
                    },
                    ToolboxIdlProgramTypeFlat::Generic {
                        symbol: "G".to_string()
                    },
                ])
            }
        }
    )
}
