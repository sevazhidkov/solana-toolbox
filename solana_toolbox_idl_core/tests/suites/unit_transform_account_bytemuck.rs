use bytemuck::bytes_of;
use bytemuck::try_from_bytes;
use bytemuck::Pod;
use bytemuck::Zeroable;
use serde_json::json;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl_core::toolbox_idl_encoding as encoding;
use solana_toolbox_idl_core::ToolboxIdlAccount;
use solana_toolbox_idl_core::ToolboxIdlProgram;

#[derive(Clone, Copy, Debug, Zeroable, PartialEq)]
#[repr(C)]
struct BytemuckContainer {
    pub bytemuck_struct_c: BytemuckStructC,
    pub bytemuck_enum_c: BytemuckEnumC,
    pub bytemuck_enum_u8: BytemuckEnumU8,
    pub bytemuck_discriminant_c: BytemuckDiscriminantC,
    pub bytemuck_discriminant_u8: BytemuckDiscriminantU8,
    pub bytemuck_field: u8,
}

#[derive(Clone, Copy, Debug, Zeroable, PartialEq)]
#[repr(C)]
struct BytemuckStructC {
    pub field1: u16,
    pub field2: Pubkey,
    pub field3: u64,
    pub field4: u8,
    pub field5: (u8, u32),
}

#[derive(Clone, Copy, Debug, Zeroable, PartialEq)]
#[repr(C)]
enum BytemuckEnumC {
    Case0,
    Case1(u16),
    Case2(Pubkey),
    Case3(u64),
    Case4(u8),
    Case5(u32),
}

#[derive(Clone, Copy, Debug, Zeroable, PartialEq)]
#[repr(u8)]
enum BytemuckEnumU8 {
    Case0,
    Case1(u16),
    Case2(Pubkey),
    Case3(u64),
    Case4(u8),
    Case5(u32),
}

#[derive(Clone, Copy, Debug, Zeroable, PartialEq)]
#[repr(C)]
enum BytemuckDiscriminantC {
    CaseA,
    CaseB,
    CaseC,
}

#[derive(Clone, Copy, Debug, Zeroable, PartialEq)]
#[repr(u8)]
enum BytemuckDiscriminantU8 {
    CaseA,
    CaseB,
    CaseC,
}

unsafe impl Pod for BytemuckContainer {}

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program = ToolboxIdlProgram::try_parse(&json!({
        "accounts": {
            "BytemuckAccount": {
                "discriminator": [],
                "type": "BytemuckContainer",
            }
        },
        "types": {
            "BytemuckContainer": {
                "serialization": "bytemuck",
                "repr": "c",
                "fields": [
                    { "name": "bytemuck_struct_c", "type": "BytemuckStructC" },
                    { "name": "bytemuck_enum_c", "type": "BytemuckEnumC" },
                    { "name": "bytemuck_enum_u8", "type": "BytemuckEnumU8" },
                    { "name": "bytemuck_discriminant_c", "type": "BytemuckDiscriminantC" },
                    { "name": "bytemuck_discriminant_u8", "type": "BytemuckDiscriminantU8" },
                    { "name": "bytemuck_field", "type": "u8" },
                ]
            },
            "BytemuckStructC": {
                "repr": "c",
                "fields": [
                    { "name": "field1", "type": "u16" },
                    { "name": "field2", "type": "pubkey" },
                    { "name": "field3", "type": "u64" },
                    { "name": "field4", "type": "u8" },
                    { "name": "field5", "fields": ["u8", "u32"] },
                ],
            },
            "BytemuckEnumC": {
                "repr": "c",
                "variants": [
                    { "name": "Case0", "fields": [] },
                    { "name": "Case1", "fields": ["u16"] },
                    { "name": "Case2", "fields": ["pubkey"] },
                    { "name": "Case3", "fields": ["u64"] },
                    { "name": "Case4", "fields": ["u8"] },
                    { "name": "Case5", "fields": ["u32"] },
                ],
            },
            "BytemuckEnumU8": {
                "repr": "rust",
                "variants": [
                    { "name": "Case0", "fields": [] },
                    { "name": "Case1", "fields": ["u16"] },
                    { "name": "Case2", "fields": ["pubkey"] },
                    { "name": "Case3", "fields": ["u64"] },
                    { "name": "Case4", "fields": ["u8"] },
                    { "name": "Case5", "fields": ["u32"] },
                ],
            },
            "BytemuckDiscriminantC": {
                "repr": "c",
                "variants": [
                    { "name": "CaseA", "fields": [] },
                    { "name": "CaseB", "fields": [] },
                    { "name": "CaseC", "fields": [] },
                ],
            },
            "BytemuckDiscriminantU8": {
                "repr": "rust",
                "variants": [
                    { "name": "CaseA", "fields": [] },
                    { "name": "CaseB", "fields": [] },
                    { "name": "CaseC", "fields": [] },
                ],
            },
        }
    }))
    .unwrap();
    // Choose the instruction
    let idl_account = idl_program.accounts.get("BytemuckAccount").unwrap();
    // Bytemuck constants
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
    // Define dummy raw data
    let raw_bytemuck_struct_c = BytemuckStructC {
        field1: 0xF1F1u16,
        field2: key_f2,
        field3: 0xF3F3F3F3F3F3F3F3u64,
        field4: 0xF4u8,
        field5: (0xF5u8, 0xF5F5F5F5u32),
    };
    // Define dummy JSON data
    let json_bytemuck_struct = json!({
        "field1": 0xF1F1u16,
        "field2": key_f2.to_string(),
        "field3": 0xF3F3F3F3F3F3F3F3u64,
        "field4": 0xF4u8,
        "field5": [0xF5u8, 0xF5F5F5F5u32],
    });
    // Generate cases datas
    let cases = vec![
        (
            "Case0",
            BytemuckContainer {
                bytemuck_struct_c: raw_bytemuck_struct_c,
                bytemuck_enum_c: BytemuckEnumC::Case0,
                bytemuck_enum_u8: BytemuckEnumU8::Case0,
                bytemuck_discriminant_c: BytemuckDiscriminantC::CaseA,
                bytemuck_discriminant_u8: BytemuckDiscriminantU8::CaseA,
                bytemuck_field: 0xD0,
            },
            json!({
                "bytemuck_struct_c": json_bytemuck_struct,
                "bytemuck_enum_c": "Case0",
                "bytemuck_enum_u8": "Case0",
                "bytemuck_discriminant_c": "CaseA",
                "bytemuck_discriminant_u8": "CaseA",
                "bytemuck_field": 0xD0,
            }),
        ),
        (
            "Case1",
            BytemuckContainer {
                bytemuck_struct_c: raw_bytemuck_struct_c,
                bytemuck_enum_c: BytemuckEnumC::Case1(0xC1C1u16),
                bytemuck_enum_u8: BytemuckEnumU8::Case1(0xC1C1u16),
                bytemuck_discriminant_c: BytemuckDiscriminantC::CaseB,
                bytemuck_discriminant_u8: BytemuckDiscriminantU8::CaseB,
                bytemuck_field: 0xD1,
            },
            json!({
                "bytemuck_struct_c": json_bytemuck_struct,
                "bytemuck_enum_c": { "Case1": [0xC1C1u16] },
                "bytemuck_enum_u8": { "Case1": [0xC1C1u16] },
                "bytemuck_discriminant_c": "CaseB",
                "bytemuck_discriminant_u8": "CaseB",
                "bytemuck_field": 0xD1,
            }),
        ),
        (
            "Case2",
            BytemuckContainer {
                bytemuck_struct_c: raw_bytemuck_struct_c,
                bytemuck_enum_c: BytemuckEnumC::Case2(key_c2),
                bytemuck_enum_u8: BytemuckEnumU8::Case2(key_c2),
                bytemuck_discriminant_c: BytemuckDiscriminantC::CaseC,
                bytemuck_discriminant_u8: BytemuckDiscriminantU8::CaseC,
                bytemuck_field: 0xD2,
            },
            json!({
                "bytemuck_struct_c": json_bytemuck_struct,
                "bytemuck_enum_c": { "Case2": [key_c2.to_string()] },
                "bytemuck_enum_u8": { "Case2": [key_c2.to_string()] },
                "bytemuck_discriminant_c": "CaseC",
                "bytemuck_discriminant_u8": "CaseC",
                "bytemuck_field": 0xD2,
            }),
        ),
        (
            "Case3",
            BytemuckContainer {
                bytemuck_struct_c: raw_bytemuck_struct_c,
                bytemuck_enum_c: BytemuckEnumC::Case3(0xC3C3C3C3C3C3C3C3u64),
                bytemuck_enum_u8: BytemuckEnumU8::Case3(0xC3C3C3C3C3C3C3C3u64),
                bytemuck_discriminant_c: BytemuckDiscriminantC::CaseA,
                bytemuck_discriminant_u8: BytemuckDiscriminantU8::CaseA,
                bytemuck_field: 0xD3,
            },
            json!({
                "bytemuck_struct_c": json_bytemuck_struct,
                "bytemuck_enum_c": { "Case3": [0xC3C3C3C3C3C3C3C3u64] },
                "bytemuck_enum_u8": { "Case3": [0xC3C3C3C3C3C3C3C3u64] },
                "bytemuck_discriminant_c": "CaseA",
                "bytemuck_discriminant_u8": "CaseA",
                "bytemuck_field": 0xD3,
            }),
        ),
        (
            "Case4",
            BytemuckContainer {
                bytemuck_struct_c: raw_bytemuck_struct_c,
                bytemuck_enum_c: BytemuckEnumC::Case4(0xC4u8),
                bytemuck_enum_u8: BytemuckEnumU8::Case4(0xC4u8),
                bytemuck_discriminant_c: BytemuckDiscriminantC::CaseB,
                bytemuck_discriminant_u8: BytemuckDiscriminantU8::CaseB,
                bytemuck_field: 0xD4,
            },
            json!({
                "bytemuck_struct_c": json_bytemuck_struct,
                "bytemuck_enum_c": { "Case4": [0xC4u8] },
                "bytemuck_enum_u8": { "Case4": [0xC4u8] },
                "bytemuck_discriminant_c": "CaseB",
                "bytemuck_discriminant_u8": "CaseB",
                "bytemuck_field": 0xD4,
            }),
        ),
        (
            "Case5",
            BytemuckContainer {
                bytemuck_struct_c: raw_bytemuck_struct_c,
                bytemuck_enum_c: BytemuckEnumC::Case5(0xC5C5C5C5u32),
                bytemuck_enum_u8: BytemuckEnumU8::Case5(0xC5C5C5C5u32),
                bytemuck_discriminant_c: BytemuckDiscriminantC::CaseC,
                bytemuck_discriminant_u8: BytemuckDiscriminantU8::CaseC,
                bytemuck_field: 0xD5,
            },
            json!({
                "bytemuck_struct_c": json_bytemuck_struct,
                "bytemuck_enum_c": { "Case5": [0xC5C5C5C5u32] },
                "bytemuck_enum_u8": { "Case5": [0xC5C5C5C5u32] },
                "bytemuck_discriminant_c": "CaseC",
                "bytemuck_discriminant_u8": "CaseC",
                "bytemuck_field": 0xD5,
            }),
        ),
    ];
    // Print all the layouts for debugging
    for case in &cases {
        println!("-- {} --", case.0);
        case_print_layout(idl_account, &case.1, &case.2);
        println!();
    }
    // Actually assert the correctness of the results
    for case in &cases {
        println!("-- {} --", case.0);
        case_assert_round_trip(idl_account, &case.1, &case.2);
        println!();
    }
}

fn case_assert_round_trip(
    idl_account: &ToolboxIdlAccount,
    raw: &BytemuckContainer,
    json: &Value,
) {
    let expected = bytes_of(raw);
    let computed = idl_account.encode(json).unwrap();
    assert_eq!(try_from_bytes::<BytemuckContainer>(&computed).unwrap(), raw);
    assert_eq!(&idl_account.decode(expected).unwrap(), json);
    assert_eq!(&idl_account.decode(&computed).unwrap(), json);
    println!("> OK!");
}

fn case_print_layout(
    idl_account: &ToolboxIdlAccount,
    raw: &BytemuckContainer,
    json: &Value,
) {
    let expected = bytes_of(raw);
    let computed = idl_account.encode(json).unwrap();
    println!("> Expected (len: {}):", expected.len());
    print_pretty_bytes(expected);
    println!("> Computed (len: {}):", computed.len());
    print_pretty_bytes(&computed);
}

fn print_pretty_bytes(bytes: &[u8]) {
    for (index, chunk16_base16) in
        encoding::encode_base16_bytes(bytes).chunks(16).enumerate()
    {
        let mut words = vec![];
        for chunk4_base16 in chunk16_base16.chunks(4) {
            words.push(chunk4_base16.join(" "));
        }
        println!("{:04}:  {}", index * 16, words.join("  "));
    }
}
