use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use spl_associated_token_account::get_associated_token_address;
use spl_associated_token_account::instruction::create_associated_token_account_idempotent;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_error::ToolboxEndpointError;

impl ToolboxEndpoint {
    pub const SPL_ASSOCIATED_TOKEN_PROGRAM_ID: Pubkey =
        spl_associated_token_account::ID;

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
            &ToolboxEndpoint::SPL_TOKEN_PROGRAM_ID,
        );
        self.process_instruction(payer, instruction).await?;
        Ok(token_account)
    }

    pub fn find_spl_associated_token_account(
        owner: &Pubkey,
        mint: &Pubkey,
    ) -> Pubkey {
        get_associated_token_address(owner, mint)
    }
}
