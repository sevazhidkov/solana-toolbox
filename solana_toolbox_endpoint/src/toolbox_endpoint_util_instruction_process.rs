use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signature;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_error::ToolboxEndpointError;

impl ToolboxEndpoint {
    pub async fn process_instruction(
        &mut self,
        payer: &Keypair,
        instruction: Instruction,
    ) -> Result<Signature, ToolboxEndpointError> {
        self.process_instructions_with_signers_and_compute_and_lookup_table(
            payer,
            &[instruction],
            &[],
            None,
            None,
            &[],
        )
        .await
    }

    pub async fn process_instruction_with_signers(
        &mut self,
        payer: &Keypair,
        instruction: Instruction,
        signers: &[&Keypair],
    ) -> Result<Signature, ToolboxEndpointError> {
        self.process_instructions_with_signers_and_compute_and_lookup_table(
            payer,
            &[instruction],
            signers,
            None,
            None,
            &[],
        )
        .await
    }

    pub async fn process_instructions_with_signers(
        &mut self,
        payer: &Keypair,
        instructions: &[Instruction],
        signers: &[&Keypair],
    ) -> Result<Signature, ToolboxEndpointError> {
        self.process_instructions_with_signers_and_compute_and_lookup_table(
            payer,
            instructions,
            signers,
            None,
            None,
            &[],
        )
        .await
    }

    pub async fn process_instructions_with_signers_and_compute(
        &mut self,
        payer: &Keypair,
        instructions: &[Instruction],
        signers: &[&Keypair],
        compute_budget_unit_limit_counter: Option<u32>,
        compute_budget_unit_price_micro_lamports: Option<u64>,
    ) -> Result<Signature, ToolboxEndpointError> {
        self.process_instructions_with_signers_and_compute_and_lookup_table(
            payer,
            instructions,
            signers,
            compute_budget_unit_limit_counter,
            compute_budget_unit_price_micro_lamports,
            &[],
        )
        .await
    }

    pub async fn process_instructions_with_signers_and_compute_and_lookup_table(
        &mut self,
        payer: &Keypair,
        instructions: &[Instruction],
        signers: &[&Keypair],
        compute_budget_unit_limit_counter: Option<u32>,
        compute_budget_unit_price_micro_lamports: Option<u64>,
        address_lookup_tables: &[(Pubkey, Vec<Pubkey>)],
    ) -> Result<Signature, ToolboxEndpointError> {
        let versioned_transaction =
            ToolboxEndpoint::compile_versioned_transaction(
                payer,
                &ToolboxEndpoint::generate_instructions_with_compute_budget(
                    instructions,
                    compute_budget_unit_limit_counter,
                    compute_budget_unit_price_micro_lamports,
                ),
                signers,
                address_lookup_tables,
                self.get_latest_blockhash().await?,
            )
            .await?;
        self.process_versioned_transaction(versioned_transaction).await
    }
}
