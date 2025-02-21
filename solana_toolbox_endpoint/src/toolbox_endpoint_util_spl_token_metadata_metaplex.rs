use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signature;
use solana_sdk::signer::Signer;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_error::ToolboxEndpointError;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
struct TokenMetadataMetaplexAccountSimplified {
    pub update_authority: Pubkey,
    pub mint: Pubkey,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
struct TokenMetadataMetaplexDataArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub creators: Option<()>,
    pub collection: Option<()>,
    pub uses: Option<()>,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
struct TokenMetadataMetaplexCreateArgsSimplified {
    pub data: TokenMetadataMetaplexDataArgs,
    pub is_mutable: bool,
    pub collection_details: Option<()>,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
struct UpdateMetadataAccountV2InstructionArgs {
    pub data: Option<TokenMetadataMetaplexDataArgs>,
    pub new_update_authority: Option<Pubkey>,
    pub primary_sale_happened: Option<bool>,
    pub is_mutable: Option<bool>,
}

// TODO - support for token metadata (metaplex/2022?)
impl ToolboxEndpoint {
    pub const SPL_TOKEN_METADATA_METAPLEX_PROGRAM_ID: Pubkey =
        pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

    pub fn find_spl_token_metadata_metaplex(mint: &Pubkey) -> Pubkey {
        Pubkey::find_program_address(
            &[
                b"metadata",
                ToolboxEndpoint::SPL_TOKEN_METADATA_METAPLEX_PROGRAM_ID
                    .as_ref(),
                mint.as_ref(),
            ],
            &ToolboxEndpoint::SPL_TOKEN_METADATA_METAPLEX_PROGRAM_ID,
        )
        .0
    }

    pub async fn get_spl_token_metadata_metaplex(
        &mut self,
        mint: &Pubkey,
    ) -> Result<Option<(Pubkey, String, String, String)>, ToolboxEndpointError>
    {
        self.get_account_data(
            &ToolboxEndpoint::find_spl_token_metadata_metaplex(mint),
        )
        .await?
        .map(|data| {
            let content =
                TokenMetadataMetaplexAccountSimplified::deserialize_reader(
                    &mut &data[1..],
                )
                .map_err(ToolboxEndpointError::Io)?;
            Ok((
                content.update_authority,
                content.symbol.trim_end_matches("\0").to_string(),
                content.name.trim_end_matches("\0").to_string(),
                content.uri.trim_end_matches("\0").to_string(),
            ))
        })
        .transpose()
    }

    pub async fn process_spl_token_metadata_metaplex_create(
        &mut self,
        payer: &Keypair,
        mint: &Pubkey,
        mint_authority: &Keypair,
        metadata_authority: &Pubkey,
        symbol: String,
        name: String,
        uri: String,
    ) -> Result<Signature, ToolboxEndpointError> {
        let mut accounts = vec![];
        accounts.push(AccountMeta::new(
            ToolboxEndpoint::find_spl_token_metadata_metaplex(mint),
            false,
        ));
        accounts.push(AccountMeta::new_readonly(*mint, false));
        accounts.push(AccountMeta::new_readonly(mint_authority.pubkey(), true));
        accounts.push(AccountMeta::new_readonly(payer.pubkey(), true));
        accounts.push(AccountMeta::new_readonly(*metadata_authority, false));
        accounts.push(AccountMeta::new_readonly(
            ToolboxEndpoint::SYSTEM_PROGRAM_ID,
            false,
        ));
        accounts.push(AccountMeta::new_readonly(
            ToolboxEndpoint::SYSVAR_RENT_ID,
            false,
        ));
        let mut data = vec![];
        data.push(33);
        TokenMetadataMetaplexCreateArgsSimplified {
            data: TokenMetadataMetaplexDataArgs {
                symbol,
                name,
                uri,
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            },
            is_mutable: true,
            collection_details: None,
        }
        .serialize(&mut data)
        .map_err(ToolboxEndpointError::Io)?;
        let instruction = Instruction {
            program_id: ToolboxEndpoint::SPL_TOKEN_METADATA_METAPLEX_PROGRAM_ID,
            accounts,
            data,
        };
        self.process_instruction_with_signers(
            instruction,
            payer,
            &[mint_authority],
        )
        .await
    }

    pub async fn process_spl_token_metadata_metaplex_update(
        &mut self,
        payer: &Keypair,
        mint: &Pubkey,
        metadata_authority: &Keypair,
        symbol: String,
        name: String,
        uri: String,
    ) -> Result<Signature, ToolboxEndpointError> {
        let mut accounts = vec![];
        accounts.push(AccountMeta::new(
            ToolboxEndpoint::find_spl_token_metadata_metaplex(mint),
            false,
        ));
        accounts
            .push(AccountMeta::new_readonly(metadata_authority.pubkey(), true));
        let mut data = vec![];
        data.push(15);
        UpdateMetadataAccountV2InstructionArgs {
            data: Some(TokenMetadataMetaplexDataArgs {
                symbol,
                name,
                uri,
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            }),
            is_mutable: None,
            new_update_authority: None,
            primary_sale_happened: None,
        }
        .serialize(&mut data)
        .map_err(ToolboxEndpointError::Io)?;
        let instruction = Instruction {
            program_id: ToolboxEndpoint::SPL_TOKEN_METADATA_METAPLEX_PROGRAM_ID,
            accounts,
            data,
        };
        self.process_instruction_with_signers(
            instruction,
            payer,
            &[metadata_authority],
        )
        .await
    }
}
