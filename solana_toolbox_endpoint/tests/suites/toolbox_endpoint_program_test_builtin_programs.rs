use solana_sdk::{
    account_info::AccountInfo, instruction::Instruction,
    program_error::ProgramError, pubkey::Pubkey, signature::Keypair,
    signer::Signer,
};
use solana_toolbox_endpoint::{
    toolbox_endpoint_program_test_builtin_program, ToolboxEndpoint,
};

#[tokio::test]
pub async fn toolbox_endpoint_program_test_builtin_program() {
    // Make a builtin program
    let builtin1_program_id = Keypair::new();
    fn builtin1_program_entry(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _data: &[u8],
    ) -> Result<(), ProgramError> {
        Ok(())
    }
    let builtin1_program = toolbox_endpoint_program_test_builtin_program!(
        builtin1_program_id.pubkey(),
        builtin1_program_entry
    );
    // Make another builtin program
    let builtin2_program_id = Keypair::new();
    fn builtin2_program_entry(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _data: &[u8],
    ) -> Result<(), ProgramError> {
        Ok(())
    }
    let builtin2_program = toolbox_endpoint_program_test_builtin_program!(
        builtin2_program_id.pubkey(),
        builtin2_program_entry
    );
    // Initialize the endpoint
    let mut toolbox_endpoint =
        ToolboxEndpoint::new_program_test_with_builtin_and_preloaded_programs(
            &[builtin1_program, builtin2_program],
            &[],
        )
        .await;
    // Fund a payer
    let payer = Keypair::new();
    toolbox_endpoint
        .process_airdrop(&payer.pubkey(), 1_000_000_000)
        .await
        .unwrap();
    // Check that the builtin program #1 works
    let instruction = Instruction {
        program_id: builtin1_program_id.pubkey(),
        accounts: vec![],
        data: vec![],
    };
    toolbox_endpoint.process_instruction(instruction, &payer).await.unwrap();
    // Check that the builtin program #2 works
    let instruction = Instruction {
        program_id: builtin2_program_id.pubkey(),
        accounts: vec![],
        data: vec![],
    };
    toolbox_endpoint.process_instruction(instruction, &payer).await.unwrap();
}
