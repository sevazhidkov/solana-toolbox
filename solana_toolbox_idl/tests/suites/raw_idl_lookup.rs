
use solana_toolbox_idl::ToolboxIdl;

#[tokio::test]
pub async fn run() {
    let idl = ToolboxIdl::try_from_value(&json!({

    })).unwrap();

        // Lookup instructions and print them
        for instruction in idl.lookup_instructions().unwrap() {
            instruction.print();
        }
    
}
