use solana_sdk::compute_budget::ComputeBudgetInstruction;
use solana_sdk::instruction::Instruction;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_error::ToolboxEndpointError;

impl ToolboxEndpoint {
    pub async fn build_transaction_from_instructions_with_signers_and_compute(
        &mut self,
        instructions: &[Instruction],
        payer: &Keypair,
        signers: &[&Keypair],
        compute_budget_unit_limit_counter: Option<u32>,
        compute_budget_unit_price_micro_lamports: Option<u64>,
    ) -> Result<Transaction, ToolboxEndpointError> {
        let mut generated_instructions = vec![];
        if let Some(compute_budget_unit_limit_counter) =
            compute_budget_unit_limit_counter
        {
            generated_instructions.push(
                ComputeBudgetInstruction::set_compute_unit_limit(
                    compute_budget_unit_limit_counter,
                ),
            );
        }
        if let Some(compute_budget_unit_price_micro_lamports) =
            compute_budget_unit_price_micro_lamports
        {
            generated_instructions.push(
                ComputeBudgetInstruction::set_compute_unit_price(
                    compute_budget_unit_price_micro_lamports,
                ),
            );
        }
        generated_instructions.extend_from_slice(instructions);
        let mut transaction = Transaction::new_with_payer(
            &generated_instructions,
            Some(&payer.pubkey()),
        );
        let mut keypairs = signers.to_owned();
        keypairs.push(payer);
        transaction.partial_sign(&keypairs, self.get_latest_blockhash().await?);
        Ok(transaction)
    }
}
