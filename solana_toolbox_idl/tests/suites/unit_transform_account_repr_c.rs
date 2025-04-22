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
    Case1(u16),
    Case2(Pubkey),
    Case3(u32),
    Case4(u8),
}

#[derive(Clone, Copy, Debug, Zeroable, PartialEq)]
#[repr(C)]
struct DummyContainerReprC {
    pub struct_repr_c: DummyStructReprC,
    pub enum_repr_c: DummyEnumReprC,
    pub footer: u64,
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
                    { "name": "Case1", "fields": ["u16"] },
                    { "name": "Case2", "fields": ["pubkey"] },
                    { "name": "Case3", "fields": ["u32"] },
                    { "name": "Case4", "fields": ["u8"] },
                ],
            },
            "DummyContainerReprC": {
                "repr": "c",
                "fields": [
                    { "name": "struct_repr_c", "type": "DummyStructReprC" },
                    { "name": "enum_repr_c", "type": "DummyEnumReprC" },
                    { "name": "footer", "type": "u64" },
                ]
            },
        }
    }))
    .unwrap();
    // Choose the instruction
    let idl_account = idl_program.accounts.get("MyAccount").unwrap();
    // Dummy constants
    let f2_key = Pubkey::new_from_array([
        0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2,
        0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2,
        0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2, 0xF2,
    ]);
    let c2_key = Pubkey::new_from_array([
        0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2,
        0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2,
        0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2, 0xC2,
    ]);
    // Define dummy JSON data
    let json_struct_repr_c = json!({
        "field1": 0xF1F1u16,
        "field2": f2_key.to_string(),
        "field3": 0xF3F3F3F3u32,
        "field4": 0xF4,
    });
    let json_enum_repr_c_1 = json!({ "Case1": [0xC1C1u16] });
    let json_enum_repr_c_2 = json!({ "Case2": [c2_key.to_string()] });
    let json_enum_repr_c_3 = json!({ "Case3": [0xC3C3C3C3u32] });
    let json_enum_repr_c_4 = json!({ "Case4": [0xC4] });
    // Generate account JSON combos
    let json_container_repr_c_1 = json!({
        "struct_repr_c": json_struct_repr_c,
        "enum_repr_c": json_enum_repr_c_1,
        "footer": 0xAAAAAAAAAAAAAAAAu64,
    });
    let json_container_repr_c_2 = json!({
        "struct_repr_c": json_struct_repr_c,
        "enum_repr_c": json_enum_repr_c_2,
        "footer": 0xAA,
    });
    let json_container_repr_c_3 = json!({
        "struct_repr_c": json_struct_repr_c,
        "enum_repr_c": json_enum_repr_c_3,
        "footer": 0xAA,
    });
    let json_container_repr_c_4 = json!({
        "struct_repr_c": json_struct_repr_c,
        "enum_repr_c": json_enum_repr_c_4,
        "footer": 0xAA,
    });
    // Define dummy raw data
    let raw_struct_repr_c = DummyStructReprC {
        field1: 0xF1F1u16,
        field2: f2_key,
        field3: 0xF3F3F3F3u32,
        field4: 0xF4,
    };
    let raw_enum_repr_c_1 = DummyEnumReprC::Case1(0xC1C1u16);
    let raw_enum_repr_c_2 = DummyEnumReprC::Case2(c2_key);
    let raw_enum_repr_c_3 = DummyEnumReprC::Case3(0xC3C3C3C3u32);
    let raw_enum_repr_c_4 = DummyEnumReprC::Case4(0xC4);
    // Generate container raw combos
    let raw_container_repr_c_1 = DummyContainerReprC {
        struct_repr_c: raw_struct_repr_c,
        enum_repr_c: raw_enum_repr_c_1,
        footer: 0xAAAAAAAAAAAAAAAAu64,
    };
    let raw_container_repr_c_2 = DummyContainerReprC {
        struct_repr_c: raw_struct_repr_c,
        enum_repr_c: raw_enum_repr_c_2,
        footer: 0xAAAAAAAAAAAAAAAAu64,
    };
    let raw_container_repr_c_3 = DummyContainerReprC {
        struct_repr_c: raw_struct_repr_c,
        enum_repr_c: raw_enum_repr_c_3,
        footer: 0xAAAAAAAAAAAAAAAAu64,
    };
    let raw_container_repr_c_4 = DummyContainerReprC {
        struct_repr_c: raw_struct_repr_c,
        enum_repr_c: raw_enum_repr_c_4,
        footer: 0xAAAAAAAAAAAAAAAAu64,
    };
    // Compute the expected bytes
    let bytes_expected_container_repr_c_1 =
        bytemuck::bytes_of(&raw_container_repr_c_1);
    let bytes_expected_container_repr_c_2 =
        bytemuck::bytes_of(&raw_container_repr_c_2);
    let bytes_expected_container_repr_c_3 =
        bytemuck::bytes_of(&raw_container_repr_c_3);
    let bytes_expected_container_repr_c_4 =
        bytemuck::bytes_of(&raw_container_repr_c_4);
    // Compute the found bytes
    let bytes_found_container_repr_c_1 =
        &idl_account.encode(&json_container_repr_c_1).unwrap();
    let bytes_found_container_repr_c_2 =
        &idl_account.encode(&json_container_repr_c_2).unwrap();
    let bytes_found_container_repr_c_3 =
        &idl_account.encode(&json_container_repr_c_3).unwrap();
    let bytes_found_container_repr_c_4 =
        &idl_account.encode(&json_container_repr_c_4).unwrap();
    // Compare results
    assert_eq!(
        ToolboxEndpoint::encode_base16_bytes(bytes_expected_container_repr_c_1)
            .join(" "),
        ToolboxEndpoint::encode_base16_bytes(bytes_found_container_repr_c_1)
            .join(" "),
    );
    assert_eq!(
        ToolboxEndpoint::encode_base16_bytes(bytes_expected_container_repr_c_2)
            .join(" "),
        ToolboxEndpoint::encode_base16_bytes(bytes_found_container_repr_c_2)
            .join(" "),
    );
    assert_eq!(
        ToolboxEndpoint::encode_base16_bytes(bytes_expected_container_repr_c_3)
            .join(" "),
        ToolboxEndpoint::encode_base16_bytes(bytes_found_container_repr_c_3)
            .join(" "),
    );
    assert_eq!(
        ToolboxEndpoint::encode_base16_bytes(bytes_expected_container_repr_c_4)
            .join(" "),
        ToolboxEndpoint::encode_base16_bytes(bytes_found_container_repr_c_4)
            .join(" "),
    );
}
