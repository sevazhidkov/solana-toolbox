use anyhow::Result;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_execution::ToolboxEndpointExecution;

impl ToolboxEndpoint {
    pub async fn simulate_instruction(
        &mut self,
        payer: &Keypair,
        instruction: Instruction,
    ) -> Result<ToolboxEndpointExecution> {
        self.simulate_instructions_with_options(
            payer,
            &[instruction],
            &[],
            &[],
            true,
        )
        .await
    }

    pub async fn simulate_instruction_with_signers(
        &mut self,
        payer: &Keypair,
        instruction: Instruction,
        signers: &[&Keypair],
    ) -> Result<ToolboxEndpointExecution> {
        self.simulate_instructions_with_options(
            payer,
            &[instruction],
            signers,
            &[],
            true,
        )
        .await
    }

    pub async fn simulate_instructions_with_signers(
        &mut self,
        payer: &Keypair,
        instructions: &[Instruction],
        signers: &[&Keypair],
    ) -> Result<ToolboxEndpointExecution> {
        self.simulate_instructions_with_options(
            payer,
            instructions,
            signers,
            &[],
            true,
        )
        .await
    }

    pub async fn simulate_instructions_with_options(
        &mut self,
        payer: &Keypair,
        instructions: &[Instruction],
        signers: &[&Keypair],
        resolved_address_lookup_tables: &[(Pubkey, Vec<Pubkey>)],
        verify_signatures: bool,
    ) -> Result<ToolboxEndpointExecution> {
        let versioned_transaction =
            ToolboxEndpoint::compile_versioned_transaction(
                payer,
                instructions,
                signers,
                resolved_address_lookup_tables,
                self.get_latest_blockhash().await?,
            )?;
        self.simulate_versioned_transaction(
            versioned_transaction,
            verify_signatures,
        )
        .await
    }
}
