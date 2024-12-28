use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_error::ToolboxEndpointError;

impl ToolboxEndpoint {
    pub async fn process_system_transfer(
        &mut self,
        payer: &Keypair,
        authority: &Keypair,
        destination: &Pubkey,
        lamports: u64,
    ) -> Result<(), ToolboxEndpointError> {
        let instruction = solana_sdk::system_instruction::transfer(
            &authority.pubkey(),
            destination,
            lamports,
        );
        self.process_instruction_with_signers(instruction, payer, &[authority])
            .await?;
        Ok(())
    }

    pub async fn process_system_create(
        &mut self,
        payer: &Keypair,
        account: &Keypair,
        lamports: u64,
        space: u64,
        owner: &Pubkey,
    ) -> Result<(), ToolboxEndpointError> {
        let instruction = solana_sdk::system_instruction::create_account(
            &payer.pubkey(),
            &account.pubkey(),
            lamports,
            space,
            owner,
        );
        self.process_instruction_with_signers(instruction, payer, &[account])
            .await?;
        Ok(())
    }
}
