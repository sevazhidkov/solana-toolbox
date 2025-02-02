use std::collections::HashMap;

use serde_json::Value;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_ok_or_else;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlInstruction {
    pub program_id: Pubkey,
    pub name: String,
    pub accounts_addresses: HashMap<String, Pubkey>,
    pub args: Value,
}

impl ToolboxIdl {
    pub async fn resolve_instruction(
        &self,
        endpoint: &mut ToolboxEndpoint,
        instruction: &ToolboxIdlInstruction,
    ) -> Result<Instruction, ToolboxIdlError> {
        let instruction_accounts_addresses = self
            .resolve_instruction_accounts_addresses(endpoint, instruction)
            .await?;
        let instruction = ToolboxIdlInstruction {
            program_id: instruction.program_id,
            name: instruction.name.clone(),
            accounts_addresses: instruction_accounts_addresses,
            args: instruction.args.clone(),
        };
        Ok(Instruction {
            program_id: instruction.program_id,
            accounts: self.compile_instruction_accounts(
                &instruction.name,
                &instruction.accounts_addresses,
            )?,
            data: self.compile_instruction_data(
                &instruction.name,
                &instruction.args,
            )?,
        })
    }

    pub fn compile_instruction(
        &self,
        instruction: &ToolboxIdlInstruction,
    ) -> Result<Instruction, ToolboxIdlError> {
        Ok(Instruction {
            program_id: instruction.program_id,
            accounts: self.compile_instruction_accounts(
                &instruction.name,
                &instruction.accounts_addresses,
            )?,
            data: self.compile_instruction_data(
                &instruction.name,
                &instruction.args,
            )?,
        })
    }

    pub fn decompile_instruction(
        &self,
        instruction: &Instruction,
    ) -> Result<ToolboxIdlInstruction, ToolboxIdlError> {
        let instruction_name = idl_ok_or_else(
            self.guess_instruction_name(instruction),
            "Could not guess instruction name",
            &ToolboxIdlBreadcrumbs::default().as_val("instruction_name"),
        )?;
        Ok(ToolboxIdlInstruction {
            program_id: instruction.program_id,
            name: instruction_name.to_string(),
            accounts_addresses: self.decompile_instruction_accounts(
                instruction_name,
                &instruction.accounts,
            )?,
            args: self.decompile_instruction_data(
                instruction_name,
                &instruction.data,
            )?,
        })
    }

    pub fn guess_instruction_name(
        &self,
        instruction: &Instruction,
    ) -> Option<&str> {
        for (program_instruction_name, program_instruction) in
            &self.program_instructions
        {
            if instruction.data.starts_with(&program_instruction.discriminator)
            {
                return Some(program_instruction_name);
            }
        }
        None
    }
}
