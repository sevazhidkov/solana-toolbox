use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_idl_account::ToolboxIdlAccount;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program::ToolboxIdlProgram;

pub struct ToolboxIdlResolver {
    programs: HashMap<Pubkey, Arc<ToolboxIdlProgram>>,
}

impl ToolboxIdlResolver {
    pub fn new() -> ToolboxIdlResolver {
        ToolboxIdlResolver {
            programs: Default::default(),
        }
    }

    pub fn preload_idl_program(
        &mut self,
        program_id: &Pubkey,
        idl_program: ToolboxIdlProgram,
    ) {
        self.programs.insert(*program_id, idl_program.into());
    }

    pub async fn resolve_idl_program(
        &mut self,
        endpoint: &mut ToolboxEndpoint,
        program_id: &Pubkey,
    ) -> Result<Arc<ToolboxIdlProgram>, ToolboxIdlError> {
        // TODO - provide standard implementation for basic contracts such as spl_token and system, and compute_budget ?
        if !self.programs.contains_key(program_id) {
            self.programs.insert(
                *program_id,
                endpoint
                    .get_account(&ToolboxIdlProgram::find_anchor_pda(
                        program_id,
                    )?)
                    .await?
                    .map(|account| {
                        ToolboxIdlProgram::try_parse_from_account_data(
                            &account.data,
                        )
                    })
                    .transpose()?
                    .ok_or_else(|| ToolboxIdlError::CouldNotFindIdl {
                        program_id: *program_id,
                    })?
                    .into(),
            );
        }
        Ok(self.programs.get(program_id).unwrap().clone())
    }

    pub async fn resolve_account_details(
        &mut self,
        endpoint: &mut ToolboxEndpoint,
        address: &Pubkey,
    ) -> Result<Option<(Arc<ToolboxIdlAccount>, Value)>, ToolboxIdlError> {
        let account = match endpoint.get_account(address).await? {
            Some(account) => account,
            None => return Ok(None),
        };
        let account_owner = account.owner;
        let account_data = account.data;
        let idl_account = self
            .resolve_idl_program(endpoint, &account_owner)
            .await?
            .guess_idl_account(&account_data)
            .ok_or_else(|| ToolboxIdlError::CouldNotFindAccount {})?;
        let account_state = idl_account.decompile(&account_data)?;
        Ok(Some((idl_account, account_state)))
    }

    pub async fn resolve_instruction(
        &mut self,
        endpoint: &mut ToolboxEndpoint,
        program_id: &Pubkey,
        instruction_name: &str,
        instruction_addresses: &HashMap<String, Pubkey>,
        instruction_payload: &Value,
    ) -> Result<Instruction, ToolboxIdlError> {
        let instruction_addresses = self
            .resolve_instruction_addresses(
                endpoint,
                program_id,
                instruction_name,
                instruction_addresses,
                instruction_payload,
            )
            .await?;
        self.resolve_idl_program(endpoint, program_id)
            .await?
            .instructions
            .get(instruction_name)
            .ok_or_else(|| ToolboxIdlError::CouldNotFindInstruction {})?
            .compile(program_id, &instruction_addresses, instruction_payload)
    }

    pub async fn resolve_instruction_addresses(
        &mut self,
        endpoint: &mut ToolboxEndpoint,
        program_id: &Pubkey,
        instruction_name: &str,
        instruction_addresses: &HashMap<String, Pubkey>,
        instruction_payload: &Value,
    ) -> Result<HashMap<String, Pubkey>, ToolboxIdlError> {
        let idl_program =
            self.resolve_idl_program(endpoint, program_id).await?;
        let idl_instruction = idl_program
            .instructions
            .get(instruction_name)
            .ok_or_else(|| ToolboxIdlError::CouldNotFindInstruction {})?;
        let mut instruction_addresses = instruction_addresses.clone();
        let mut resolved_snapshots = HashMap::new();
        for (instruction_account_name, instruction_address) in
            &instruction_addresses
        {
            if let Ok(Some((idl_account, account_state))) = self
                .resolve_account_details(endpoint, instruction_address)
                .await
            {
                resolved_snapshots.insert(
                    instruction_account_name.to_string(),
                    (idl_account.content_type_full.clone(), account_state),
                );
            }
        }
        loop {
            let breadcrumbs = ToolboxIdlBreadcrumbs::default();
            let mut made_progress = false;
            for idl_instruction_account in &idl_instruction.accounts {
                if instruction_addresses
                    .contains_key(&idl_instruction_account.name)
                {
                    continue;
                }
                if let Ok(instruction_address) = idl_instruction_account
                    .try_compute(
                        program_id,
                        &instruction_addresses,
                        &resolved_snapshots,
                        &(
                            &idl_instruction.args_type_full_fields,
                            &instruction_payload,
                        ),
                        &breadcrumbs.with_idl(&idl_instruction.name),
                    )
                {
                    made_progress = true;
                    instruction_addresses.insert(
                        idl_instruction_account.name.to_string(),
                        instruction_address,
                    );
                    if let Ok(Some((idl_account, account_state))) = self
                        .resolve_account_details(endpoint, &instruction_address)
                        .await
                    {
                        resolved_snapshots.insert(
                            idl_instruction_account.name.to_string(),
                            (
                                idl_account.content_type_full.clone(),
                                account_state,
                            ),
                        );
                    }
                }
            }
            if !made_progress {
                break;
            }
        }
        Ok(instruction_addresses)
    }
}
