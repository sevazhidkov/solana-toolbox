use anyhow::Result;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction::allocate;
use solana_sdk::system_instruction::assign;
use solana_sdk::system_instruction::create_account;
use solana_sdk::system_instruction::transfer;
use solana_sdk::system_program;

use crate::toolbox_endpoint::ToolboxEndpoint;

impl ToolboxEndpoint {
    pub const SYSTEM_PROGRAM_ID: Pubkey = system_program::ID;

    pub async fn process_system_new(
        &mut self,
        payer: &Keypair,
        lamports: u64,
        space: usize,
        owner: &Pubkey,
    ) -> Result<Pubkey> {
        let account = Keypair::new();
        self.process_system_create(payer, &account, lamports, space, owner)
            .await?;
        Ok(account.pubkey())
    }

    pub async fn process_system_new_exempt(
        &mut self,
        payer: &Keypair,
        space: usize,
        owner: &Pubkey,
    ) -> Result<Pubkey> {
        let account = Keypair::new();
        self.process_system_create_exempt(payer, &account, space, owner)
            .await?;
        Ok(account.pubkey())
    }

    pub async fn process_system_create(
        &mut self,
        payer: &Keypair,
        account: &Keypair,
        lamports: u64,
        space: usize,
        owner: &Pubkey,
    ) -> Result<()> {
        let instruction = create_account(
            &payer.pubkey(),
            &account.pubkey(),
            lamports,
            u64::try_from(space)?,
            owner,
        );
        self.process_instruction_with_signers(payer, instruction, &[account])
            .await?;
        Ok(())
    }

    pub async fn process_system_create_exempt(
        &mut self,
        payer: &Keypair,
        account: &Keypair,
        space: usize,
        owner: &Pubkey,
    ) -> Result<()> {
        let lamports = self.get_sysvar_rent().await?.minimum_balance(space);
        let instruction = create_account(
            &payer.pubkey(),
            &account.pubkey(),
            lamports,
            u64::try_from(space)?,
            owner,
        );
        self.process_instruction_with_signers(payer, instruction, &[account])
            .await?;
        Ok(())
    }

    pub async fn process_system_transfer(
        &mut self,
        payer: &Keypair,
        source: &Keypair,
        destination: &Pubkey,
        lamports: u64,
    ) -> Result<()> {
        let instruction = transfer(&source.pubkey(), destination, lamports);
        self.process_instruction_with_signers(payer, instruction, &[source])
            .await?;
        Ok(())
    }

    pub async fn process_system_allocate(
        &mut self,
        payer: &Keypair,
        account: &Keypair,
        space: usize,
    ) -> Result<()> {
        let instruction = allocate(&account.pubkey(), u64::try_from(space)?);
        self.process_instruction_with_signers(payer, instruction, &[account])
            .await?;
        Ok(())
    }

    pub async fn process_system_assign(
        &mut self,
        payer: &Keypair,
        account: &Keypair,
        owner: &Pubkey,
    ) -> Result<()> {
        let instruction = assign(&account.pubkey(), owner);
        self.process_instruction_with_signers(payer, instruction, &[account])
            .await?;
        Ok(())
    }
}
