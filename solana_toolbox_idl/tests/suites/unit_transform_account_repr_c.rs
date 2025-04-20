use bytemuck::Pod;
use bytemuck::Zeroable;
use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_idl::ToolboxIdlProgram;

#[derive(Clone, Debug, Zeroable, Copy)]
#[repr(C)]
struct DummyStructReprC {
    pub field1: u16,
    pub field2: Pubkey,
    pub field3: u32,
    pub field4: u64,
}

#[derive(Clone, Debug, Zeroable, Copy)]
#[repr(C)]
enum DummyEnumReprC {
    A(u8),
    B(u64),
    C(u16),
}

#[derive(Clone, Debug, Zeroable, Copy)]
#[repr(C, u64)]
enum DummyEnumReprCU64 {
    A(u8),
    B(u64),
    C(u16),
}

#[derive(Clone, Debug, Zeroable, Copy)]
#[repr(C)]
struct DummyContainer {
    pub struct_repr_c: DummyStructReprC,
    pub enum_repr_c: DummyEnumReprC,
    pub enum_repr_c_u64: DummyEnumReprCU64,
}

unsafe impl Pod for DummyContainer {}

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program = ToolboxIdlProgram::try_parse_from_value(&json!({
        "accounts": {
            "MyAccount": {
                "discriminator": [],
                "fields": [
                ]
            }
        },
    }))
    .unwrap();
    // Choose the instruction
    let idl_account = idl_program.accounts.get("MyAccount").unwrap();
    // Dada
    let binding = DummyStructReprC {
        field1: 42,
        field2: Keypair::new().pubkey(),
        field3: 43,
        field4: 44,
    };
    //let dada = bytemuck::bytes_of(&binding);
    //eprintln!("dada: {:?}", dada);
    let binding_a = DummyEnumReprC::A(41);
    let binding_b = DummyEnumReprC::B(42);
    //eprintln!("binding_a: {:?}", bytemuck::bytes_of(&binding_a));
    //eprintln!("binding_b: {:?}", bytemuck::bytes_of(&binding_b));
    panic!("LOL");
    // Check that we can use the manual IDL to encode/decode our account in different ways
}
