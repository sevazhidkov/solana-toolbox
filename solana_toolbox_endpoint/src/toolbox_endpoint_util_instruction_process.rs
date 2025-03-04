use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signature;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_error::ToolboxEndpointError;
use crate::ToolboxEndpointExecution;

impl ToolboxEndpoint {
    pub async fn process_instruction(
        &mut self,
        payer: &Keypair,
        instruction: Instruction,
    ) -> Result<(Signature, ToolboxEndpointExecution), ToolboxEndpointError>
    {
        self.process_instructions_with_options(
            payer,
            &[instruction],
            &[],
            None,
            None,
            &[],
            false,
        )
        .await
    }

    pub async fn process_instruction_with_signers(
        &mut self,
        payer: &Keypair,
        instruction: Instruction,
        signers: &[&Keypair],
    ) -> Result<(Signature, ToolboxEndpointExecution), ToolboxEndpointError>
    {
        self.process_instructions_with_options(
            payer,
            &[instruction],
            signers,
            None,
            None,
            &[],
            false,
        )
        .await
    }

    pub async fn process_instructions_with_signers(
        &mut self,
        payer: &Keypair,
        instructions: &[Instruction],
        signers: &[&Keypair],
    ) -> Result<(Signature, ToolboxEndpointExecution), ToolboxEndpointError>
    {
        self.process_instructions_with_options(
            payer,
            instructions,
            signers,
            None,
            None,
            &[],
            false,
        )
        .await
    }

    pub async fn process_instructions_with_options(
        &mut self,
        payer: &Keypair,
        instructions: &[Instruction],
        signers: &[&Keypair],
        compute_budget_unit_limit_counter: Option<u32>,
        compute_budget_unit_price_micro_lamports: Option<u64>,
        resolved_address_lookup_tables: &[(Pubkey, Vec<Pubkey>)],
        skip_preflight: bool,
    ) -> Result<(Signature, ToolboxEndpointExecution), ToolboxEndpointError>
    {
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
        self.process_versioned_transaction(
            versioned_transaction,
            skip_preflight,
        )
        .await
    }
}
