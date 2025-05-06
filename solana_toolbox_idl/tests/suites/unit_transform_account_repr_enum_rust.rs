use bytemuck::Pod;
use bytemuck::Zeroable;
use serde_json::json;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_idl::ToolboxIdlAccount;
use solana_toolbox_idl::ToolboxIdlProgram;

#[derive(Clone, Copy, Debug, Zeroable, PartialEq)]
#[repr(C)]
struct DummyStructReprC {
    pub field1: u16,
    pub field2: Pubkey,
    pub field3: u32,
    pub field4: u8,
}

#[derive(Clone, Copy, Debug, Zeroable, PartialEq)]
#[repr(u8)]
enum DummyEnumReprU8 {
    Case0,
    Case1(u16),
    Case2(Pubkey),
    Case3(u32),
    Case4(u8, u64),
}

#[derive(Clone, Copy, Debug, Zeroable, PartialEq)]
#[repr(C)]
struct DummyContainerReprC {
    pub struct_repr_c: DummyStructReprC,
    pub enum_repr_u8: DummyEnumReprU8,
}

unsafe impl Pod for DummyContainerReprC {}

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program = ToolboxIdlProgram::try_parse_from_value(&json!({
        "accounts": {
            "MyAccount": {
                "discriminator": [],
                "type": "DummyContainerReprC",
            }
        },
        "types": {
            "DummyStructReprC": {
                "repr": "c",
                "fields": [
                    { "name": "field1", "type": "u16" },
                    { "name": "field2", "type": "pubkey" },
                    { "name": "field3", "type": "u32" },
                    { "name": "field4", "type": "u8" },
                ],
            },
            "DummyEnumReprU8": {
                "repr": "rust",
                "variants": [
                    "Case0",
                    { "name": "Case1", "fields": ["u16"] },
                    { "name": "Case2", "fields": ["pubkey"] },
                    { "name": "Case3", "fields": ["u32"] },
                    { "name": "Case4", "fields": ["u8", "u64"] },
                ],
            },
            "DummyContainerReprC": {
                "repr": "c",
                "fields": [
                    { "name": "struct_repr_c", "type": "DummyStructReprC" },
                    { "name": "enum_repr_u8", "type": "DummyEnumReprU8" },
                ]
            },
        }
    }))
    .unwrap();
    // Choose the instruction
    let idl_account = idl_program.accounts.get("MyAccount").unwrap();
    // Dummy constants
    let key_f2 = Pubkey::new_from_array([
        0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2,
        0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2,
        0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2,
    ]);
    let key_c2 = Pubkey::new_from_array([
        0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2,
        0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2,
        0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2,
    ]);
    // Define dummy JSON data
    let json_struct_repr_c = json!({
        "field1": 0xF1F1u16,
        "field2": key_f2.to_string(),
        "field3": 0xF3F3F3F3u32,
        "field4": 0xF4u8,
    });
    // Generate account JSON combos
    let json_container_repr_c_0 = json!({
        "struct_repr_c": json_struct_repr_c,
        "enum_repr_u8": "Case0",
    });
    let json_container_repr_c_1 = json!({
        "struct_repr_c": json_struct_repr_c,
        "enum_repr_u8": { "Case1": [0xC1C1u16] },
    });
    let json_container_repr_c_2 = json!({
        "struct_repr_c": json_struct_repr_c,
        "enum_repr_u8": { "Case2": [key_c2.to_string()] },
    });
    let json_container_repr_c_3 = json!({
        "struct_repr_c": json_struct_repr_c,
        "enum_repr_u8": { "Case3": [0xC3C3C3C3u32] },
    });
    let json_container_repr_c_4 = json!({
        "struct_repr_c": json_struct_repr_c,
        "enum_repr_u8": { "Case4": [0xC4u8, 0xC5C5C5C5C5C5C5C5u64] },
    });
    // Define dummy raw data
    let raw_struct_repr_c = DummyStructReprC {
        field1: 0xF1F1u16,
        field2: key_f2,
        field3: 0xF3F3F3F3u32,
        field4: 0xF4u8,
    };
    // Generate container raw combos
    let raw_container_repr_c_0 = DummyContainerReprC {
        struct_repr_c: raw_struct_repr_c,
        enum_repr_u8: DummyEnumReprU8::Case0,
    };
    let raw_container_repr_c_1 = DummyContainerReprC {
        struct_repr_c: raw_struct_repr_c,
        enum_repr_u8: DummyEnumReprU8::Case1(0xC1C1u16),
    };
    let raw_container_repr_c_2 = DummyContainerReprC {
        struct_repr_c: raw_struct_repr_c,
        enum_repr_u8: DummyEnumReprU8::Case2(key_c2),
    };
    let raw_container_repr_c_3 = DummyContainerReprC {
        struct_repr_c: raw_struct_repr_c,
        enum_repr_u8: DummyEnumReprU8::Case3(0xC3C3C3C3u32),
    };
    let raw_container_repr_c_4 = DummyContainerReprC {
        struct_repr_c: raw_struct_repr_c,
        enum_repr_u8: DummyEnumReprU8::Case4(0xC4u8, 0xC5C5C5C5C5C5C5C5u64),
    };
    // Compare and check results
    assert_case_round_trip(
        "case0",
        &idl_account,
        &json_container_repr_c_0,
        &raw_container_repr_c_0,
    );
    assert_case_round_trip(
        "case1",
        &idl_account,
        &json_container_repr_c_1,
        &raw_container_repr_c_1,
    );
    assert_case_round_trip(
        "case2",
        &idl_account,
        &json_container_repr_c_2,
        &raw_container_repr_c_2,
    );
    assert_case_round_trip(
        "case3",
        &idl_account,
        &json_container_repr_c_3,
        &raw_container_repr_c_3,
    );
    assert_case_round_trip(
        "case4",
        &idl_account,
        &json_container_repr_c_4,
        &raw_container_repr_c_4,
    );
    panic!("LOL");
}

fn assert_case_round_trip(
    name: &str,
    idl_account: &ToolboxIdlAccount,
    json: &Value,
    raw: &DummyContainerReprC,
) {
    let expected = bytemuck::bytes_of(raw);
    let found = idl_account.encode(json).unwrap();
    println!("-- {} --", name);
    println!("> expected:");
    print_pretty_bytes(&expected);
    println!("> found:");
    print_pretty_bytes(&found);
    println!();
    /*
    assert_eq!(
        bytemuck::try_from_bytes::<DummyContainerReprC>(&found).unwrap(),
        raw
    );
    assert_eq!(&idl_account.decode(expected).unwrap(), json);
    assert_eq!(&idl_account.decode(&found).unwrap(), json);
     */
}

fn print_pretty_bytes(bytes: &[u8]) {
    let bytes_base16 = ToolboxEndpoint::encode_base16_bytes(bytes);
    println!("len: {}", bytes_base16.len());
    for (index, chunk16_base16) in bytes_base16.chunks(16).enumerate() {
        let mut blobs = vec![];
        for chunk4_base16 in chunk16_base16.chunks(4) {
            blobs.push(chunk4_base16.join(" "));
        }
        println!("{:04X}:  {}", index * 16, blobs.join("  "));
    }
}
