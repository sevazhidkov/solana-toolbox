use std::collections::HashMap;

use serde_json::json;
use solana_toolbox_idl::ToolboxIdlPath;
use solana_toolbox_idl::ToolboxIdlProgram;
use solana_toolbox_idl::ToolboxIdlTypePrimitive;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program = ToolboxIdlProgram::try_parse(&json!({
        "accounts": {
            "MyAccount": {
                "fields": [
                    { "name": "field1", "type": "f32" },
                    { "name": "field2", "type": "MyStructNamed" },
                    { "name": "field3", "type": "MyStructUnnamed" },
                    { "name": "field4", "type": "MyEnum" },
                ]
            }
        },
        "types": {
            "MyStructNamed": {
                "fields": [
                    { "name": "field1", "type": "u8" },
                    { "name": "field2", "type": "u16" },
                    { "name": "field3", "type": ["u32"] },
                    { "name": "field4", "type": ["u64", 4] },
                ]
            },
            "MyStructUnnamed": {
                "fields": [
                   "u8",
                   "u16",
                   ["u32"],
                   ["u64", 4],
                ]
            },
            "MyEnum": {
                "variants": [
                    "Case0",
                    { "name": "Case1", "fields": ["bool", "pubkey"] },
                    {
                        "name": "Case2",
                        "fields": [
                            {"name": "field1", "type": "i8" },
                            {"name": "field2", "type": "i16" },
                        ]
                    },
                ]
            }
        }
    }))
    .unwrap();
    // Check that we can read the proper type using the proper paths
    assert_get(&idl_program, "field1", ToolboxIdlTypePrimitive::F32);
    // Named Struct Fields
    assert_get(&idl_program, "field2.field1", ToolboxIdlTypePrimitive::U8);
    assert_get(&idl_program, "field2.field2", ToolboxIdlTypePrimitive::U16);
    assert_get(&idl_program, "field2.field3.", ToolboxIdlTypePrimitive::U32);
    assert_get(&idl_program, "field2.field4.", ToolboxIdlTypePrimitive::U64);
    assert_get(
        &idl_program,
        "field2.field3.5",
        ToolboxIdlTypePrimitive::U32,
    );
    assert_get(
        &idl_program,
        "field2.field4.5",
        ToolboxIdlTypePrimitive::U64,
    );
    // Unnamed Struct Fields
    assert_get(&idl_program, "field3.0", ToolboxIdlTypePrimitive::U8);
    assert_get(&idl_program, "field3.1", ToolboxIdlTypePrimitive::U16);
    assert_get(&idl_program, "field3.2.", ToolboxIdlTypePrimitive::U32);
    assert_get(&idl_program, "field3.3.", ToolboxIdlTypePrimitive::U64);
    assert_get(&idl_program, "field3.2.5", ToolboxIdlTypePrimitive::U32);
    assert_get(&idl_program, "field3.3.5", ToolboxIdlTypePrimitive::U64);
    // Enum Fields
    assert_get(
        &idl_program,
        "field4.Case1.0",
        ToolboxIdlTypePrimitive::Boolean,
    );
    assert_get(
        &idl_program,
        "field4.Case1.1",
        ToolboxIdlTypePrimitive::PublicKey,
    );
    assert_get(
        &idl_program,
        "field4.Case2.field1",
        ToolboxIdlTypePrimitive::I8,
    );
    assert_get(
        &idl_program,
        "field4.Case2.field2",
        ToolboxIdlTypePrimitive::I16,
    );
}

fn assert_get(
    idl_program: &ToolboxIdlProgram,
    path: &str,
    expected: ToolboxIdlTypePrimitive,
) {
    assert_eq!(
        ToolboxIdlPath::try_parse(path)
            .unwrap()
            .try_get_type_flat(
                &idl_program
                    .accounts
                    .get("MyAccount")
                    .unwrap()
                    .content_type_flat,
                &HashMap::new(),
                &idl_program.typedefs
            )
            .unwrap(),
        expected.into()
    );
}
