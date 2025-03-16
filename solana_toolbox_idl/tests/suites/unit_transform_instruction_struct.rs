use std::collections::HashMap;

use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program = ToolboxIdlProgram::try_parse_from_value(&json!({
        "instructions": {
            "my_ix": {
                "discriminator": [77, 78],
                "accounts": [{ "name": "addr", "signer": true }],
                "args": [
                    { "name": "arg1", "type": {"defined": "MyArg"} },
                    { "name": "arg2", "type": "i16" },
                ]
            }
        },
        "types": {
            "MyArg": {
                "fields": [
                    { "name": "id", "type": "u16" },
                    { "name": "data", "type": {"vec": "u8"} },
                ]
            }
        },
    }))
    .unwrap();
    // Choose the instruction
    let idl_instruction = idl_program.get_idl_instruction("my_ix").unwrap();
    // Check that we can use the manual IDL to compile/decompile our IX
    let program_id = Pubkey::new_unique();
    let instruction_addresses =
        HashMap::from_iter([("addr".to_string(), Pubkey::new_unique())]);
    let instruction_payload = json!({
        "arg1": {
            "id": 42,
            "data": [1, 2, 3]
        },
        "arg2": -2,
    });
    let instruction = idl_instruction
        .compile(&program_id, &instruction_addresses, &instruction_payload)
        .unwrap();
    assert_eq!(instruction.program_id, program_id);
    assert_eq!(
        instruction.accounts[0].pubkey,
        *instruction_addresses.get("addr").unwrap(),
    );
    assert_eq!(instruction.accounts[0].is_signer, true);
    assert_eq!(instruction.accounts[0].is_writable, false);
    assert_eq!(
        vec![77, 78, 42, 0, 3, 0, 0, 0, 1, 2, 3, 254, 255],
        instruction.data
    );
    assert_eq!(
        idl_instruction.decompile(&instruction).unwrap(),
        (program_id, instruction_addresses, instruction_payload),
    );
}
