use std::str::FromStr;

use serde_json::json;
use solana_sdk::signature::Signature;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointLoggerPrinter;
use solana_toolbox_idl::ToolboxIdlResolver;

#[tokio::test]
pub async fn run() {
    // Create the endpoint
    let mut endpoint = ToolboxEndpoint::new_mainnet().await;
    // Create a print logger
    endpoint.add_logger(Box::new(ToolboxEndpointLoggerPrinter::default()));

    let mut idl_resolver = ToolboxIdlResolver::new();

    let signature = Signature::from_str("44KbtWbYXCdn5vZwKoz8tfG2pLFT7TLqwVvDwVuFaMBMAaaENCVjN8DczQfRaAZvvSwpVkAUDWwSdfryrwU6fzNh").unwrap();
    let execution = endpoint.get_execution(&signature).await.unwrap();
    let mut json_instructions = vec![];
    for instruction in execution.instructions {
        let idl_program = idl_resolver
            .resolve_program(&mut endpoint, &instruction.program_id)
            .await
            .unwrap();
        //eprintln!("idl_program: {:#?}", idl_program);
        eprintln!("instruction: {:#?}", instruction);
        let idl_instruction = idl_program
            .guess_idl_instruction(&instruction.data)
            .unwrap(); // TODO - handle unwrap
        let (program_id, instruction_addresses, instruction_payload) =
            idl_instruction.decompile(&instruction).unwrap();
        let mut json_addresses = vec![];
        for instruction_address in instruction_addresses {
            json_addresses.push(json!([
                instruction_address.0,
                instruction_address.1.to_string()
            ]));
        }
        json_instructions.push(json!({
            "program_id": program_id.to_string(),
            "name": idl_instruction.name,
            "addresses": json_addresses,
            "payload": instruction_payload,
            "data": instruction.data,
        }));
    }
    let json = json!({
        "payer": execution.payer.to_string(),
        "instructions": json_instructions,
        "logs": execution.logs,
        "error": execution.error, // TODO - could parse the error using the code
        "return_data": execution.return_data,
        "units_consumed": execution.units_consumed,
    });
    println!("{}", serde_json::to_string_pretty(&json).unwrap());

    panic!("LOL");
}
