use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_anchor::ToolboxAnchor;
use crate::toolbox_anchor_error::ToolboxAnchorError;

impl ToolboxAnchor {
    pub fn build_instruction<
        Accounts: ToAccountMetas,
        Payload: InstructionData,
    >(
        program_id: Pubkey,
        accounts: Accounts,
        payload: Payload,
    ) -> Instruction {
        Instruction {
            program_id,
            accounts: accounts.to_account_metas(None),
            data: payload.data(),
        }
    }

    pub async fn process_instruction<
        Accounts: ToAccountMetas,
        Payload: InstructionData,
    >(
        endpoint: &mut ToolboxEndpoint,
        program_id: Pubkey,
        accounts: Accounts,
        payload: Payload,
        payer: &Keypair,
    ) -> Result<(), ToolboxAnchorError> {
        endpoint
            .process_instruction(
                payer,
                ToolboxAnchor::build_instruction(program_id, accounts, payload),
            )
            .await?;
        Ok(())
    }

    pub async fn process_instruction_with_signers<
        Accounts: ToAccountMetas,
        Payload: InstructionData,
    >(
        endpoint: &mut ToolboxEndpoint,
        program_id: Pubkey,
        accounts: Accounts,
        payload: Payload,
        payer: &Keypair,
        signers: &[&Keypair],
    ) -> Result<(), ToolboxAnchorError> {
        endpoint
            .process_instruction_with_signers(
                payer,
                ToolboxAnchor::build_instruction(program_id, accounts, payload),
                signers,
            )
            .await?;
        Ok(())
    }
}
