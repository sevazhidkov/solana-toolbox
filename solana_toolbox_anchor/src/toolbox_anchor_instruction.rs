use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signature;

use crate::toolbox_anchor_endpoint::ToolboxAnchorEndpoint;
use crate::toolbox_anchor_error::ToolboxAnchorError;

impl ToolboxAnchorEndpoint {
    pub async fn process_anchor_instruction<
        Accounts: ToAccountMetas,
        Payload: InstructionData,
    >(
        &mut self,
        program_id: Pubkey,
        accounts: Accounts,
        payload: Payload,
        payer: &Keypair,
    ) -> Result<Signature, ToolboxAnchorError> {
        let instruction = Instruction {
            program_id,
            accounts: accounts.to_account_metas(None),
            data: payload.data(),
        };
        let signature = self.process_instruction(instruction, payer).await?;
        Ok(signature)
    }

    pub async fn process_anchor_instruction_with_signers<
        Accounts: ToAccountMetas,
        Payload: InstructionData,
    >(
        &mut self,
        program_id: Pubkey,
        accounts: Accounts,
        payload: Payload,
        payer: &Keypair,
        signers: &[&Keypair],
    ) -> Result<Signature, ToolboxAnchorError> {
        let instruction = Instruction {
            program_id,
            accounts: accounts.to_account_metas(None),
            data: payload.data(),
        };
        let signature = self
            .process_instruction_with_signers(instruction, payer, signers)
            .await?;
        Ok(signature)
    }
}
