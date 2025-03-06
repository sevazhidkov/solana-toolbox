use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_error::ToolboxEndpointError;
use crate::toolbox_endpoint_execution::ToolboxEndpointExecution;

impl ToolboxEndpoint {
    pub async fn simulate_instruction(
        &mut self,
        payer: &Keypair,
        instruction: Instruction,
    ) -> Result<ToolboxEndpointExecution, ToolboxEndpointError> {
        self.simulate_instructions_with_options(payer, &[instruction], &[], &[])
            .await
    }

    pub async fn simulate_instruction_with_signers(
        &mut self,
        payer: &Keypair,
        instruction: Instruction,
        signers: &[&Keypair],
    ) -> Result<ToolboxEndpointExecution, ToolboxEndpointError> {
        self.simulate_instructions_with_options(
            payer,
            &[instruction],
            signers,
            &[],
        )
        .await
    }

    pub async fn simulate_instructions_with_signers(
        &mut self,
        payer: &Keypair,
        instructions: &[Instruction],
        signers: &[&Keypair],
    ) -> Result<ToolboxEndpointExecution, ToolboxEndpointError> {
        self.simulate_instructions_with_options(
            payer,
            instructions,
            signers,
            &[],
        )
        .await
    }

    pub async fn simulate_instructions_with_options(
        &mut self,
        payer: &Keypair,
        instructions: &[Instruction],
        signers: &[&Keypair],
        resolved_address_lookup_tables: &[(Pubkey, Vec<Pubkey>)],
    ) -> Result<ToolboxEndpointExecution, ToolboxEndpointError> {
        let versioned_transaction =
            ToolboxEndpoint::compile_versioned_transaction(
                payer,
                instructions,
                signers,
                resolved_address_lookup_tables,
                self.get_latest_blockhash().await?,
            )?;
        self.simulate_versioned_transaction(versioned_transaction)
            .await
    }
}
