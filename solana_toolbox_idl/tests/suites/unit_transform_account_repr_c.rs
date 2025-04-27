use bytemuck::Pod;
use bytemuck::Zeroable;
use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;
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
#[repr(C)]
enum DummyEnumReprC {
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
    pub enum_repr_c: DummyEnumReprC,
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
            "DummyEnumReprC": {
                "repr": "c",
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
                    { "name": "enum_repr_c", "type": "DummyEnumReprC" },
                ]
            },
        }
    }))
    .unwrap();
    // Choose the instruction
    let idl_account = idl_program.accounts.get("MyAccount").unwrap();
    eprintln!("idl_account: {:#?}", idl_account.content_type_full);
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
    let json_enum_repr_c_0 = json!({ "Case0": [] });
    let json_enum_repr_c_1 = json!({ "Case1": [0xC1C1u16] });
    let json_enum_repr_c_2 = json!({ "Case2": [key_c2.to_string()] });
    let json_enum_repr_c_3 = json!({ "Case3": [0xC3C3C3C3u32] });
    let json_enum_repr_c_4 =
        json!({ "Case4": [0xC4u8, 0xC5C5C5C5C5C5C5C5u64] });
    // Generate account JSON combos
    let json_container_repr_c_0 = json!({
        "struct_repr_c": json_struct_repr_c,
        "enum_repr_c": json_enum_repr_c_0,
    });
    let json_container_repr_c_1 = json!({
        "struct_repr_c": json_struct_repr_c,
        "enum_repr_c": json_enum_repr_c_1,
    });
    let json_container_repr_c_2 = json!({
        "struct_repr_c": json_struct_repr_c,
        "enum_repr_c": json_enum_repr_c_2,
    });
    let json_container_repr_c_3 = json!({
        "struct_repr_c": json_struct_repr_c,
        "enum_repr_c": json_enum_repr_c_3,
    });
    let json_container_repr_c_4 = json!({
        "struct_repr_c": json_struct_repr_c,
        "enum_repr_c": json_enum_repr_c_4,
    });
    // Define dummy raw data
    let raw_struct_repr_c = DummyStructReprC {
        field1: 0xF1F1u16,
        field2: key_f2,
        field3: 0xF3F3F3F3u32,
        field4: 0xF4u8,
    };
    let raw_enum_repr_c_0 = DummyEnumReprC::Case0;
    let raw_enum_repr_c_1 = DummyEnumReprC::Case1(0xC1C1u16);
    let raw_enum_repr_c_2 = DummyEnumReprC::Case2(key_c2);
    let raw_enum_repr_c_3 = DummyEnumReprC::Case3(0xC3C3C3C3u32);
    let raw_enum_repr_c_4 =
        DummyEnumReprC::Case4(0xC4u8, 0xC5C5C5C5C5C5C5C5u64);
    // Generate container raw combos
    let raw_container_repr_c_0 = DummyContainerReprC {
        struct_repr_c: raw_struct_repr_c,
        enum_repr_c: raw_enum_repr_c_0,
    };
    let raw_container_repr_c_1 = DummyContainerReprC {
        struct_repr_c: raw_struct_repr_c,
        enum_repr_c: raw_enum_repr_c_1,
    };
    let raw_container_repr_c_2 = DummyContainerReprC {
        struct_repr_c: raw_struct_repr_c,
        enum_repr_c: raw_enum_repr_c_2,
    };
    let raw_container_repr_c_3 = DummyContainerReprC {
        struct_repr_c: raw_struct_repr_c,
        enum_repr_c: raw_enum_repr_c_3,
    };
    let raw_container_repr_c_4 = DummyContainerReprC {
        struct_repr_c: raw_struct_repr_c,
        enum_repr_c: raw_enum_repr_c_4,
    };
    // Compute the expected bytes
    let bytes_expected_container_repr_c_0 =
        bytemuck::bytes_of(&raw_container_repr_c_0);
    let bytes_expected_container_repr_c_1 =
        bytemuck::bytes_of(&raw_container_repr_c_1);
    let bytes_expected_container_repr_c_2 =
        bytemuck::bytes_of(&raw_container_repr_c_2);
    let bytes_expected_container_repr_c_3 =
        bytemuck::bytes_of(&raw_container_repr_c_3);
    let bytes_expected_container_repr_c_4 =
        bytemuck::bytes_of(&raw_container_repr_c_4);
    // Compute the found bytes
    let bytes_found_container_repr_c_0 =
        &idl_account.encode(&json_container_repr_c_0).unwrap();
    let bytes_found_container_repr_c_1 =
        &idl_account.encode(&json_container_repr_c_1).unwrap();
    let bytes_found_container_repr_c_2 =
        &idl_account.encode(&json_container_repr_c_2).unwrap();
    let bytes_found_container_repr_c_3 =
        &idl_account.encode(&json_container_repr_c_3).unwrap();
    let bytes_found_container_repr_c_4 =
        &idl_account.encode(&json_container_repr_c_4).unwrap();
    // Compare and check results
    print_pretty_bytes_compare(
        "case0",
        bytes_expected_container_repr_c_0,
        bytes_found_container_repr_c_0,
    );
    print_pretty_bytes_compare(
        "case1",
        bytes_expected_container_repr_c_1,
        bytes_found_container_repr_c_1,
    );
    print_pretty_bytes_compare(
        "case2",
        bytes_expected_container_repr_c_2,
        bytes_found_container_repr_c_2,
    );
    print_pretty_bytes_compare(
        "case3",
        bytes_expected_container_repr_c_3,
        bytes_found_container_repr_c_3,
    );
    print_pretty_bytes_compare(
        "case4",
        bytes_expected_container_repr_c_4,
        bytes_found_container_repr_c_4,
    );
    // Assert everything is well
    assert_eq!(
        bytes_expected_container_repr_c_0,
        bytes_found_container_repr_c_0,
    );
    assert_eq!(
        bytes_expected_container_repr_c_1,
        bytes_found_container_repr_c_1,
    );
    assert_eq!(
        bytes_expected_container_repr_c_2,
        bytes_found_container_repr_c_2,
    );
    assert_eq!(
        bytes_expected_container_repr_c_3,
        bytes_found_container_repr_c_3,
    );
    assert_eq!(
        bytes_expected_container_repr_c_4,
        bytes_found_container_repr_c_4,
    );
}

fn print_pretty_bytes_compare(name: &str, expected: &[u8], found: &[u8]) {
    let expected_base16_array = ToolboxEndpoint::encode_base16_bytes(expected);
    let found_base16_array = ToolboxEndpoint::encode_base16_bytes(found);
    println!("-- {} --", name);
    println!("> expected:");
    print_pretty_bytes(&expected_base16_array);
    println!("> found:");
    print_pretty_bytes(&found_base16_array);
    println!();
}

fn print_pretty_bytes(bytes_base16_array: &[String]) {
    println!("len: {}", bytes_base16_array.len());
    for (index, bytes_16) in bytes_base16_array.chunks(16).enumerate() {
        let mut blobs = vec![];
        for byte_4 in bytes_16.chunks(4) {
            blobs.push(byte_4.join(" "));
        }
        println!("{:04X}:  {}", index * 16, blobs.join("  "));
    }
}
