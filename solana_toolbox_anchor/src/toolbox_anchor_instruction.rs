use anchor_lang::ToAccountMetas;
use anchor_lang::InstructionData;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signature;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_anchor_error::ToolboxAnchorError;

pub async fn process_anchor_instruction<
    Accounts: ToAccountMetas,
    Params: InstructionData,
>(
    toolbox_endpoint: &mut ToolboxEndpoint,
    program_id: Pubkey,
    accounts: Accounts,
    params: Params,
    payer: &Keypair,
) -> Result<Signature, ToolboxAnchorError> {
    let instruction = Instruction {
        program_id,
        accounts: accounts.to_account_metas(None),
        data: params.data(),
    };
    toolbox_endpoint
        .process_instruction(instruction, payer)
        .await
        .map_err(ToolboxAnchorError::ToolboxEndpoint)
}

pub async fn process_anchor_instruction_with_signers<
    Accounts: ToAccountMetas,
    Params: InstructionData,
>(
    toolbox_endpoint: &mut ToolboxEndpoint,
    program_id: Pubkey,
    accounts: Accounts,
    params: Params,
    payer: &Keypair,
    signers: &[&Keypair],
) -> Result<Signature, ToolboxAnchorError> {
    let instruction = Instruction {
        program_id,
        accounts: accounts.to_account_metas(None),
        data: params.data(),
    };
    toolbox_endpoint
        .process_instruction_with_signers(instruction, payer, signers)
        .await
        .map_err(ToolboxAnchorError::ToolboxEndpoint)
}
