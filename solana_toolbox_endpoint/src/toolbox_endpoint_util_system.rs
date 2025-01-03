use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signature;
use solana_sdk::signer::Signer;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_error::ToolboxEndpointError;

impl ToolboxEndpoint {
    pub async fn process_system_transfer(
        &mut self,
        payer: &Keypair,
        source: &Keypair,
        destination: &Pubkey,
        lamports: u64,
    ) -> Result<Signature, ToolboxEndpointError> {
        let instruction = solana_sdk::system_instruction::transfer(
            &source.pubkey(),
            destination,
            lamports,
        );
        self.process_instruction_with_signers(instruction, payer, &[source])
            .await
    }

    pub async fn process_system_allocate(
        &mut self,
        payer: &Keypair,
        account: &Keypair,
        space: u64,
    ) -> Result<Signature, ToolboxEndpointError> {
        let instruction =
            solana_sdk::system_instruction::allocate(&account.pubkey(), space);
        self.process_instruction_with_signers(instruction, payer, &[account])
            .await
    }

    pub async fn process_system_assign(
        &mut self,
        payer: &Keypair,
        account: &Keypair,
        owner: &Pubkey,
    ) -> Result<Signature, ToolboxEndpointError> {
        let instruction =
            solana_sdk::system_instruction::assign(&account.pubkey(), owner);
        self.process_instruction_with_signers(instruction, payer, &[account])
            .await
    }

    pub async fn process_system_create(
        &mut self,
        payer: &Keypair,
        account: &Keypair,
        lamports: u64,
        space: usize,
        owner: &Pubkey,
    ) -> Result<Signature, ToolboxEndpointError> {
        let instruction = solana_sdk::system_instruction::create_account(
            &payer.pubkey(),
            &account.pubkey(),
            lamports,
            space as u64,
            owner,
        );
        self.process_instruction_with_signers(instruction, payer, &[account])
            .await
    }

    pub async fn process_system_create_exempt(
        &mut self,
        payer: &Keypair,
        account: &Keypair,
        space: usize,
        owner: &Pubkey,
    ) -> Result<Signature, ToolboxEndpointError> {
        let lamports = self.get_sysvar_rent().await?.minimum_balance(space);
        let instruction = solana_sdk::system_instruction::create_account(
            &payer.pubkey(),
            &account.pubkey(),
            lamports,
            space as u64,
            owner,
        );
        self.process_instruction_with_signers(instruction, payer, &[account])
            .await
    }
}
