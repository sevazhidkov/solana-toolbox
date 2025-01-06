use solana_sdk::account_info::AccountInfo;
use solana_sdk::instruction::Instruction;
use solana_sdk::program_error::ProgramError;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::toolbox_endpoint_program_test_builtin_program;
use solana_toolbox_endpoint::ToolboxEndpoint;

#[tokio::test]
pub async fn toolbox_endpoint_program_test_builtin_programs() {
    // Define dummy builtin program #1
    let builtin1_program_id = Keypair::new();
    fn builtin1_program_entry(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _data: &[u8],
    ) -> Result<(), ProgramError> {
        Ok(())
    }
    // Define dummy builtin program #2
    let builtin2_program_id = Keypair::new();
    fn builtin2_program_entry(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _data: &[u8],
    ) -> Result<(), ProgramError> {
        Ok(())
    }
    // Initialize the endpoint
    let mut toolbox_endpoint =
        ToolboxEndpoint::new_program_test_with_builtin_and_preloaded_programs(
            &[
                toolbox_endpoint_program_test_builtin_program!(
                    builtin1_program_id.pubkey(),
                    builtin1_program_entry
                ),
                toolbox_endpoint_program_test_builtin_program!(
                    builtin2_program_id.pubkey(),
                    builtin2_program_entry
                ),
            ],
            &[],
        )
        .await;
    // Prepare a payer
    let payer = Keypair::new();
    toolbox_endpoint
        .process_airdrop(&payer.pubkey(), 1_000_000_000)
        .await
        .unwrap();
    // Check that the builtin program #1 works
    toolbox_endpoint
        .process_instruction(
            Instruction {
                program_id: builtin1_program_id.pubkey(),
                accounts: vec![],
                data: vec![],
            },
            &payer,
        )
        .await
        .unwrap();
    // Check that the builtin program #2 works
    toolbox_endpoint
        .process_instruction(
            Instruction {
                program_id: builtin2_program_id.pubkey(),
                accounts: vec![],
                data: vec![],
            },
            &payer,
        )
        .await
        .unwrap();
}
