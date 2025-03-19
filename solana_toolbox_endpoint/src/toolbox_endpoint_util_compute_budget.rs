use solana_sdk::compute_budget;
use solana_sdk::compute_budget::ComputeBudgetInstruction;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_endpoint::ToolboxEndpoint;

impl ToolboxEndpoint {
    pub const COMPUTE_BUDGET_PROGRAM_ID: Pubkey = compute_budget::ID;

    pub fn generate_instructions_with_compute_budget(
        instructions: &[Instruction],
        paid_compute_units: Option<u32>,
        micro_lamport_price_per_unit: Option<u64>,
    ) -> Vec<Instruction> {
        let mut generated_instructions = vec![];
        if let Some(paid_compute_units) = paid_compute_units {
            generated_instructions.push(
                ComputeBudgetInstruction::set_compute_unit_limit(
                    paid_compute_units,
                ),
            );
        }
        if let Some(micro_lamport_price_per_unit) = micro_lamport_price_per_unit
        {
            generated_instructions.push(
                ComputeBudgetInstruction::set_compute_unit_price(
                    micro_lamport_price_per_unit,
                ),
            );
        }
        generated_instructions.extend_from_slice(instructions);
        generated_instructions
    }
}
