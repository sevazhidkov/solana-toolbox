use anyhow::Result;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::toolbox_endpoint::ToolboxEndpoint;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
struct TokenMetaplexMetadataAccountSimplified {
    pub update_authority: Pubkey,
    pub mint: Pubkey,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
struct TokenMetaplexMetadataDataArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub creators: Option<()>,
    pub collection: Option<()>,
    pub uses: Option<()>,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
struct TokenMetaplexMetadataCreateArgsSimplified {
    pub data: TokenMetaplexMetadataDataArgs,
    pub is_mutable: bool,
    pub collection_details: Option<()>,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
struct TokenMetaplexMetadataUpdateArgsSimplified {
    pub data: Option<TokenMetaplexMetadataDataArgs>,
    pub new_update_authority: Option<Pubkey>,
    pub primary_sale_happened: Option<bool>,
    pub is_mutable: Option<bool>,
}

impl ToolboxEndpoint {
    pub const SPL_TOKEN_METAPLEX_METADATA_PROGRAM_ID: Pubkey =
        pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

    pub fn find_spl_token_metaplex_metadata(mint: &Pubkey) -> Pubkey {
        Pubkey::find_program_address(
            &[
                b"metadata",
                ToolboxEndpoint::SPL_TOKEN_METAPLEX_METADATA_PROGRAM_ID
                    .as_ref(),
                mint.as_ref(),
            ],
            &ToolboxEndpoint::SPL_TOKEN_METAPLEX_METADATA_PROGRAM_ID,
        )
        .0
    }

    pub async fn get_spl_token_metaplex_metadata(
        &mut self,
        mint: &Pubkey,
    ) -> Result<Option<(Pubkey, String, String, String)>> {
        self.get_account_data(
            &ToolboxEndpoint::find_spl_token_metaplex_metadata(mint),
        )
        .await?
        .map(|data| {
            let content =
                TokenMetaplexMetadataAccountSimplified::deserialize_reader(
                    &mut &data[1..],
                )?;
            Ok((
                content.update_authority,
                content.symbol.trim_end_matches("\0").to_string(),
                content.name.trim_end_matches("\0").to_string(),
                content.uri.trim_end_matches("\0").to_string(),
            ))
        })
        .transpose()
    }

    pub async fn process_spl_token_metaplex_metadata_create(
        &mut self,
        payer: &Keypair,
        mint: &Pubkey,
        mint_authority: &Keypair,
        metadata: (Pubkey, String, String, String),
    ) -> Result<()> {
        let accounts = vec![
            AccountMeta::new(
                ToolboxEndpoint::find_spl_token_metaplex_metadata(mint),
                false,
            ),
            AccountMeta::new_readonly(*mint, false),
            AccountMeta::new_readonly(mint_authority.pubkey(), true),
            AccountMeta::new_readonly(payer.pubkey(), true),
            AccountMeta::new_readonly(metadata.0, false),
            AccountMeta::new_readonly(
                ToolboxEndpoint::SYSTEM_PROGRAM_ID,
                false,
            ),
            AccountMeta::new_readonly(ToolboxEndpoint::SYSVAR_RENT_ID, false),
        ];
        let mut data = vec![];
        data.push(33);
        TokenMetaplexMetadataCreateArgsSimplified {
            data: TokenMetaplexMetadataDataArgs {
                symbol: metadata.1,
                name: metadata.2,
                uri: metadata.3,
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            },
            is_mutable: true,
            collection_details: None,
        }
        .serialize(&mut data)?;
        let instruction = Instruction {
            program_id: ToolboxEndpoint::SPL_TOKEN_METAPLEX_METADATA_PROGRAM_ID,
            accounts,
            data,
        };
        self.process_instruction_with_signers(
            payer,
            instruction,
            &[mint_authority],
        )
        .await?;
        Ok(())
    }

    pub async fn process_spl_token_metaplex_metadata_update(
        &mut self,
        payer: &Keypair,
        mint: &Pubkey,
        metadata_authority: &Keypair,
        metadata: (Pubkey, String, String, String),
    ) -> Result<()> {
        let accounts = vec![
            AccountMeta::new(
                ToolboxEndpoint::find_spl_token_metaplex_metadata(mint),
                false,
            ),
            AccountMeta::new_readonly(metadata_authority.pubkey(), true),
        ];
        let mut data = vec![];
        data.push(15);
        TokenMetaplexMetadataUpdateArgsSimplified {
            data: Some(TokenMetaplexMetadataDataArgs {
                symbol: metadata.1,
                name: metadata.2,
                uri: metadata.3,
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            }),
            new_update_authority: Some(metadata.0),
            primary_sale_happened: None,
            is_mutable: None,
        }
        .serialize(&mut data)?;
        let instruction = Instruction {
            program_id: ToolboxEndpoint::SPL_TOKEN_METAPLEX_METADATA_PROGRAM_ID,
            accounts,
            data,
        };
        self.process_instruction_with_signers(
            payer,
            instruction,
            &[metadata_authority],
        )
        .await?;
        Ok(())
    }
}
