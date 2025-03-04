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
        self.simulate_instructions_with_options(
            payer,
            &[instruction],
            &[],
            None,
            None,
            &[],
        )
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
            None,
            None,
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
            None,
            None,
            &[],
        )
        .await
    }

    pub async fn simulate_instructions_with_options(
        &mut self,
        payer: &Keypair,
        instructions: &[Instruction],
        signers: &[&Keypair],
        compute_budget_unit_limit_counter: Option<u32>,
        compute_budget_unit_price_micro_lamports: Option<u64>,
        resolved_address_lookup_tables: &[(Pubkey, Vec<Pubkey>)],
    ) -> Result<ToolboxEndpointExecution, ToolboxEndpointError> {
        let versioned_transaction =
            ToolboxEndpoint::compile_versioned_transaction(
                payer,
                &ToolboxEndpoint::generate_instructions_with_compute_budget(
                    instructions,
                    compute_budget_unit_limit_counter,
                    compute_budget_unit_price_micro_lamports,
                ),
                signers,
                resolved_address_lookup_tables,
                self.get_latest_blockhash().await?,
            )?;
        self.simulate_versioned_transaction(versioned_transaction)
            .await
    }
}
