use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signature;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction::create_account;
use spl_associated_token_account::get_associated_token_address;
use spl_associated_token_account::instruction::create_associated_token_account_idempotent;
use spl_token::instruction::burn;
use spl_token::instruction::freeze_account;
use spl_token::instruction::initialize_account;
use spl_token::instruction::initialize_mint;
use spl_token::instruction::mint_to;
use spl_token::instruction::set_authority;
use spl_token::instruction::thaw_account;
use spl_token::instruction::transfer;
use spl_token::instruction::AuthorityType;
use spl_token::state::Account;
use spl_token::state::Mint;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_error::ToolboxEndpointError;

impl ToolboxEndpoint {
    pub async fn process_spl_token_mint_new(
        &mut self,
        payer: &Keypair,
        mint_authority: &Pubkey,
        mint_freeze_authority: Option<&Pubkey>,
        mint_decimals: u8,
    ) -> Result<Pubkey, ToolboxEndpointError> {
        let mint = Keypair::new();
        self.process_spl_token_mint_init(
            payer,
            &mint,
            mint_authority,
            mint_freeze_authority,
            mint_decimals,
        )
        .await?;
        Ok(mint.pubkey())
    }

    pub async fn process_spl_token_mint_init(
        &mut self,
        payer: &Keypair,
        mint: &Keypair,
        mint_authority: &Pubkey,
        mint_freeze_authority: Option<&Pubkey>,
        mint_decimals: u8,
    ) -> Result<Signature, ToolboxEndpointError> {
        let rent_space = Mint::LEN;
        let rent_minimum_lamports =
            self.get_sysvar_rent().await?.minimum_balance(rent_space);
        let instruction_create = create_account(
            &payer.pubkey(),
            &mint.pubkey(),
            rent_minimum_lamports,
            u64::try_from(rent_space)
                .map_err(ToolboxEndpointError::TryFromInt)?,
            &spl_token::ID,
        );
        let instruction_init = initialize_mint(
            &spl_token::ID,
            &mint.pubkey(),
            mint_authority,
            mint_freeze_authority,
            mint_decimals,
        )?;
        self.process_instructions_with_signers(
            &[instruction_create, instruction_init],
            payer,
            &[mint],
        )
        .await
    }

    pub async fn process_spl_token_mint_set_authority(
        &mut self,
        payer: &Keypair,
        mint: &Pubkey,
        source_mint_authority: &Keypair,
        destination_mint_authority: Option<&Pubkey>,
    ) -> Result<Signature, ToolboxEndpointError> {
        let instruction = set_authority(
            &spl_token::ID,
            mint,
            destination_mint_authority,
            AuthorityType::MintTokens,
            &source_mint_authority.pubkey(),
            &[],
        )?;
        self.process_instruction_with_signers(
            instruction,
            payer,
            &[source_mint_authority],
        )
        .await
    }

    pub async fn process_spl_token_mint_to(
        &mut self,
        payer: &Keypair,
        mint: &Pubkey,
        mint_authority: &Keypair,
        destination_token_account: &Pubkey,
        amount: u64,
    ) -> Result<Signature, ToolboxEndpointError> {
        let instruction = mint_to(
            &spl_token::ID,
            mint,
            destination_token_account,
            &mint_authority.pubkey(),
            &[],
            amount,
        )?;
        self.process_instruction_with_signers(
            instruction,
            payer,
            &[mint_authority],
        )
        .await
    }

    pub async fn process_spl_token_mint_set_freeze_authority(
        &mut self,
        payer: &Keypair,
        mint: &Pubkey,
        source_mint_freeze_authority: &Keypair,
        destination_mint_freeze_authority: Option<&Pubkey>,
    ) -> Result<Signature, ToolboxEndpointError> {
        let instruction = set_authority(
            &spl_token::ID,
            mint,
            destination_mint_freeze_authority,
            AuthorityType::FreezeAccount,
            &source_mint_freeze_authority.pubkey(),
            &[],
        )?;
        self.process_instruction_with_signers(
            instruction,
            payer,
            &[source_mint_freeze_authority],
        )
        .await
    }

    pub async fn process_spl_token_freeze(
        &mut self,
        payer: &Keypair,
        mint: &Pubkey,
        mint_freeze_authority: &Keypair,
        token_account: &Pubkey,
    ) -> Result<Signature, ToolboxEndpointError> {
        let instruction = freeze_account(
            &spl_token::ID,
            token_account,
            mint,
            &mint_freeze_authority.pubkey(),
            &[],
        )?;
        self.process_instruction_with_signers(
            instruction,
            payer,
            &[mint_freeze_authority],
        )
        .await
    }

    pub async fn process_spl_token_thaw(
        &mut self,
        payer: &Keypair,
        mint: &Pubkey,
        mint_freeze_authority: &Keypair,
        token_account: &Pubkey,
    ) -> Result<Signature, ToolboxEndpointError> {
        let instruction = thaw_account(
            &spl_token::ID,
            token_account,
            mint,
            &mint_freeze_authority.pubkey(),
            &[],
        )?;
        self.process_instruction_with_signers(
            instruction,
            payer,
            &[mint_freeze_authority],
        )
        .await
    }

    pub async fn process_spl_token_transfer(
        &mut self,
        payer: &Keypair,
        owner: &Keypair,
        source_token_account: &Pubkey,
        destination_token_account: &Pubkey,
        amount: u64,
    ) -> Result<Signature, ToolboxEndpointError> {
        let instruction = transfer(
            &spl_token::ID,
            source_token_account,
            destination_token_account,
            &owner.pubkey(),
            &[],
            amount,
        )?;
        self.process_instruction_with_signers(instruction, payer, &[owner])
            .await
    }

    pub async fn process_spl_token_burn(
        &mut self,
        payer: &Keypair,
        owner: &Keypair,
        source_token_account: &Pubkey,
        mint: &Pubkey,
        amount: u64,
    ) -> Result<Signature, ToolboxEndpointError> {
        let instruction = burn(
            &spl_token::ID,
            source_token_account,
            mint,
            &owner.pubkey(),
            &[],
            amount,
        )?;
        self.process_instruction_with_signers(instruction, payer, &[owner])
            .await
    }

    pub async fn process_spl_token_account_new(
        &mut self,
        payer: &Keypair,
        owner: &Pubkey,
        mint: &Pubkey,
    ) -> Result<Pubkey, ToolboxEndpointError> {
        let rent_space = Account::LEN;
        let rent_minimum_lamports =
            self.get_sysvar_rent().await?.minimum_balance(rent_space);
        let account = Keypair::new();
        let instruction_create = create_account(
            &payer.pubkey(),
            &account.pubkey(),
            rent_minimum_lamports,
            u64::try_from(rent_space)
                .map_err(ToolboxEndpointError::TryFromInt)?,
            &spl_token::ID,
        );
        let instruction_init = initialize_account(
            &spl_token::id(),
            &account.pubkey(),
            mint,
            owner,
        )?;
        self.process_instructions_with_signers(
            &[instruction_create, instruction_init],
            payer,
            &[&account],
        )
        .await?;
        Ok(account.pubkey())
    }

    pub async fn process_spl_associated_token_account_get_or_init(
        &mut self,
        payer: &Keypair,
        owner: &Pubkey,
        mint: &Pubkey,
    ) -> Result<Pubkey, ToolboxEndpointError> {
        let token_account =
            ToolboxEndpoint::find_spl_associated_token_account(owner, mint);
        if self.get_spl_token_account(&token_account).await?.is_some() {
            return Ok(token_account);
        }
        let instruction = create_associated_token_account_idempotent(
            &payer.pubkey(),
            owner,
            mint,
            &spl_token::id(),
        );
        self.process_instruction(instruction, payer).await?;
        Ok(token_account)
    }

    pub async fn get_spl_token_mint(
        &mut self,
        mint: &Pubkey,
    ) -> Result<Option<Mint>, ToolboxEndpointError> {
        self.get_account_data_unpacked::<Mint>(mint).await
    }

    pub async fn get_spl_token_account(
        &mut self,
        token_account: &Pubkey,
    ) -> Result<Option<Account>, ToolboxEndpointError> {
        self.get_account_data_unpacked::<Account>(token_account).await
    }

    pub fn find_spl_associated_token_account(
        owner: &Pubkey,
        mint: &Pubkey,
    ) -> Pubkey {
        get_associated_token_address(owner, mint)
    }

    pub fn convert_spl_token_amount_to_ui_amount(
        token_amount: u64,
        mint_decimals: u8,
    ) -> f64 {
        (token_amount as f64) / 10f64.powi(mint_decimals.into())
    }

    pub fn convert_ui_amount_to_spl_token_amount(
        ui_amount: f64,
        mint_decimals: u8,
    ) -> u64 {
        (ui_amount * 10f64.powi(mint_decimals.into())) as u64
    }
}
