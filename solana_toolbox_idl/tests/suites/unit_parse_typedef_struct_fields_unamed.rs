use serde_json::json;
use solana_toolbox_idl::ToolboxIdlProgram;
use solana_toolbox_idl::ToolboxIdlTypeFlat;
use solana_toolbox_idl::ToolboxIdlTypeFlatFields;
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
                    ToolboxIdlTypeFlat::Option {
                        prefix_bytes: 1,
                        content: Box::new(ToolboxIdlTypeFlat::Primitive {
                            primitive: ToolboxIdlTypePrimitive::U8,
                        }),
                    },
                    ToolboxIdlTypeFlat::Option {
                        prefix_bytes: 4,
                        content: Box::new(ToolboxIdlTypeFlat::Primitive {
                            primitive: ToolboxIdlTypePrimitive::U8,
                        }),
                    },
                ])
            }
        }
        .into()
    )
}
