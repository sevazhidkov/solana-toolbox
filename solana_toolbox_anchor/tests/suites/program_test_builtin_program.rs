use solana_sdk::account_info::AccountInfo;
use solana_sdk::instruction::Instruction;
use solana_sdk::program_error::ProgramError;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_anchor::toolbox_endpoint_program_test_builtin_program_anchor;
use solana_toolbox_anchor::ToolboxEndpoint;

#[tokio::test]
pub async fn program_test_builtin_programs() {
    // Define dummy builtin program with anchor lifetimes
    let builtin_program_id = Keypair::new();
    fn builtin_program_entry<'info>(
        _program_id: &Pubkey,
        _accounts: &'info [AccountInfo<'info>],
        _data: &[u8],
    ) -> Result<(), ProgramError> {
        Ok(())
    }
    // Initialize the endpoint
    let mut toolbox_endpoint =
        ToolboxEndpoint::new_program_test_with_builtin_programs(&[
            toolbox_endpoint_program_test_builtin_program_anchor!(
                builtin_program_id.pubkey(),
                builtin_program_entry
            ),
        ])
        .await;
    // Prepare a payer
    let payer = Keypair::new();
    toolbox_endpoint
        .process_airdrop(&payer.pubkey(), 1_000_000_000)
        .await
        .unwrap();
    // Check that the builtin program works
    toolbox_endpoint
        .process_instruction(
            Instruction {
                program_id: builtin_program_id.pubkey(),
                accounts: vec![],
                data: vec![],
            },
            &payer,
        )
        .await
        .unwrap();
}
