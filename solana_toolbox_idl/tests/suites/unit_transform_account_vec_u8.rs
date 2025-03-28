use serde_json::json;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program = ToolboxIdlProgram::try_parse_from_value(&json!({
        "accounts": {
            "MyAccount": {
                "fields": [
                    { "name": "bytes", "type": "bytes" },
                    { "name": "vec_u8_1", "type": {"vec": "u8"} },
                    { "name": "vec_u8_1", "type": ["u8"] },
                ]
            }
        },
    }))
    .unwrap();
    // Choose the instruction
    let idl_account = idl_program.accounts.get("MyAccount").unwrap();
    // Check that we can use the manual IDL to encode/decode our account in different ways
    let blob_coordinator_join_run = [
        67, 111, 111, 114, 100, 105, 110, 97, 116, 111, 114, 74, 111, 105, 110,
        82, 117, 110,
    ];
    let case1 = idl_account
        .encode(&json!({
            "bytes": blob_coordinator_join_run,
            "vec_u8_1": blob_coordinator_join_run,
            "vec_u8_2": blob_coordinator_join_run,
        }))
        .unwrap();
    let case2 = idl_account
        .encode(&json!({
            "bytes": b"CoordinatorJoinRun",
            "vec_u8_1": b"CoordinatorJoinRun",
            "vec_u8_2": b"CoordinatorJoinRun",
        }))
        .unwrap();
    let case3 = idl_account
        .encode(&json!({
            "bytes": {"utf8": "CoordinatorJoinRun"},
            "vec_u8_1": {"utf8": "CoordinatorJoinRun"},
            "vec_u8_2": {"utf8": "CoordinatorJoinRun"},
        }))
        .unwrap();
    let case4 = idl_account.encode(&json!({
        "bytes": {"hex": "43 6F 6F 72 64 69 6E 61 74 6F 72 4A 6F 69 6E 52 75 6E"},
        "vec_u8_1": {"hex": "436F6F7264696E61746F724A6F696E52756E"},
        "vec_u8_2": {"hex": "\"436F6F7264696E61746F724A6F696E52756E\""},
    })).unwrap();
    let case5 = idl_account
        .encode(&json!({
            "bytes": {"base58": "3oE - ADz TpG yQHQ ioFs uM8m zv Xf"},
            "vec_u8_1": {"base58": "3oEADzTpGyQHQioFsuM8mzvXf"},
            "vec_u8_2": {"base58": "\"3oEADzTpGyQHQioFsuM8mzvXf\""},
        }))
        .unwrap();
    let case6 = idl_account
        .encode(&json!({
            "bytes": {"base64": "Q29 v cm RpbmF0b3JKb2luUnVu"},
            "vec_u8_1": {"base64": "Q29vcmRpbmF0b3JKb2luUnVu"},
            "vec_u8_2": {"base64": "\"Q29vcmRpbmF0b3JKb2luUnVu\""},
        }))
        .unwrap();
    // Check that we got the correct results
    let mut expected = vec![];
    expected.extend_from_slice(&idl_account.discriminator);
    expected.extend_from_slice(&18u32.to_le_bytes());
    expected.extend_from_slice(b"CoordinatorJoinRun");
    expected.extend_from_slice(&18u32.to_le_bytes());
    expected.extend_from_slice(b"CoordinatorJoinRun");
    expected.extend_from_slice(&18u32.to_le_bytes());
    expected.extend_from_slice(b"CoordinatorJoinRun");
    assert_eq!(case1, expected);
    assert_eq!(case2, expected);
    assert_eq!(case3, expected);
    assert_eq!(case4, expected);
    assert_eq!(case5, expected);
    assert_eq!(case6, expected);
}
