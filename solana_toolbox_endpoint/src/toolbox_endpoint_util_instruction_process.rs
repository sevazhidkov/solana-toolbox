use solana_sdk::instruction::Instruction;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signature;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_error::ToolboxEndpointError;

impl ToolboxEndpoint {
    pub async fn process_instruction(
        &mut self,
        instruction: Instruction,
        payer: &Keypair,
    ) -> Result<Signature, ToolboxEndpointError> {
        self.process_instructions_with_signers_and_compute(
            &[instruction],
            payer,
            &[],
            None,
            None,
        )
        .await
    }

    pub async fn process_instruction_with_signers(
        &mut self,
        instruction: Instruction,
        payer: &Keypair,
        signers: &[&Keypair],
    ) -> Result<Signature, ToolboxEndpointError> {
        self.process_instructions_with_signers_and_compute(
            &[instruction],
            payer,
            signers,
            None,
            None,
        )
        .await
    }

    pub async fn process_instructions_with_signers(
        &mut self,
        instructions: &[Instruction],
        payer: &Keypair,
        signers: &[&Keypair],
    ) -> Result<Signature, ToolboxEndpointError> {
        self.process_instructions_with_signers_and_compute(
            instructions,
            payer,
            signers,
            None,
            None,
        )
        .await
    }

    pub async fn process_instructions_with_signers_and_compute(
        &mut self,
        instructions: &[Instruction],
        payer: &Keypair,
        signers: &[&Keypair],
        compute_budget_unit_limit_counter: Option<u32>,
        compute_budget_unit_price_micro_lamports: Option<u64>,
    ) -> Result<Signature, ToolboxEndpointError> {
        let transaction = self
            .build_transaction_from_instructions_with_signers_and_compute(
                instructions,
                payer,
                signers,
                compute_budget_unit_limit_counter,
                compute_budget_unit_price_micro_lamports,
            )
            .await?;
        self.process_transaction(&transaction).await
    }
}
