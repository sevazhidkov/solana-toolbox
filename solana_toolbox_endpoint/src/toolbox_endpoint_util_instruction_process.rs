use anyhow::Result;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signature;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_execution::ToolboxEndpointExecution;

impl ToolboxEndpoint {
    pub async fn process_instruction(
        &mut self,
        payer: &Keypair,
        instruction: Instruction,
    ) -> Result<(Signature, ToolboxEndpointExecution)> {
        self.process_instructions_with_options(
            payer,
            &[instruction],
            &[],
            &[],
            true,
        )
        .await
    }

    pub async fn process_instruction_with_signers(
        &mut self,
        payer: &Keypair,
        instruction: Instruction,
        signers: &[&Keypair],
    ) -> Result<(Signature, ToolboxEndpointExecution)> {
        self.process_instructions_with_options(
            payer,
            &[instruction],
            signers,
            &[],
            true,
        )
        .await
    }

    pub async fn process_instructions_with_signers(
        &mut self,
        payer: &Keypair,
        instructions: &[Instruction],
        signers: &[&Keypair],
    ) -> Result<(Signature, ToolboxEndpointExecution)> {
        self.process_instructions_with_options(
            payer,
            instructions,
            signers,
            &[],
            true,
        )
        .await
    }

    pub async fn process_instructions_with_options(
        &mut self,
        payer: &Keypair,
        instructions: &[Instruction],
        signers: &[&Keypair],
        resolved_address_lookup_tables: &[(Pubkey, Vec<Pubkey>)],
        verify_prelight: bool,
    ) -> Result<(Signature, ToolboxEndpointExecution)> {
        let versioned_transaction =
            ToolboxEndpoint::compile_versioned_transaction(
                payer,
                instructions,
                signers,
                resolved_address_lookup_tables,
                self.get_latest_blockhash().await?,
            )?;
        self.process_versioned_transaction(
            versioned_transaction,
            verify_prelight,
        )
        .await
    }
}
