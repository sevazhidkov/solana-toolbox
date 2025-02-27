use solana_sdk::compute_budget::ComputeBudgetInstruction;
use solana_sdk::instruction::Instruction;

use crate::toolbox_endpoint::ToolboxEndpoint;

impl ToolboxEndpoint {
    pub fn generate_instructions_with_compute_budget(
        instructions: &[Instruction],
        compute_budget_unit_limit_counter: Option<u32>,
        compute_budget_unit_price_micro_lamports: Option<u64>,
    ) -> Vec<Instruction> {
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
        generated_instructions
    }
}
