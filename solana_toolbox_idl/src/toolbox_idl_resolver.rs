use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_idl_account::ToolboxIdlAccount;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_instruction::ToolboxIdlInstruction;
use crate::toolbox_idl_program::ToolboxIdlProgram;

pub struct ToolboxIdlResolver {
    cached_programs: HashMap<Pubkey, Option<Arc<ToolboxIdlProgram>>>,
}

impl Default for ToolboxIdlResolver {
    fn default() -> ToolboxIdlResolver {
        ToolboxIdlResolver::new()
    }
}

impl ToolboxIdlResolver {
    pub fn new() -> ToolboxIdlResolver {
        ToolboxIdlResolver {
            cached_programs: Default::default(),
        }
    }

    pub fn preload_program(
        &mut self,
        program_id: &Pubkey,
        idl_program: Option<Arc<ToolboxIdlProgram>>,
    ) {
        self.cached_programs.insert(*program_id, idl_program);
    }

    async fn resolve_program(
        &mut self,
        endpoint: &mut ToolboxEndpoint,
        program_id: &Pubkey,
    ) -> Result<Option<Arc<ToolboxIdlProgram>>, ToolboxIdlError> {
        if let Some(idl_program) = self.cached_programs.get(program_id) {
            return Ok(idl_program.clone());
        }
        let idl_program = {
            if let Some(idl_program) = ToolboxIdlProgram::from_lib(program_id) {
                Some(Arc::new(idl_program))
            } else {
                let mut source_account = None;
                if let Some(anchor_account) = endpoint
                    .get_account(&ToolboxIdlProgram::find_anchor(program_id)?)
                    .await?
                {
                    source_account = Some(anchor_account);
                } else if let Some(shank_account) = endpoint
                    .get_account(&ToolboxIdlProgram::find_shank(program_id)?)
                    .await?
                {
                    source_account = Some(shank_account);
                }
                source_account
                    .map(|source_account| {
                        ToolboxIdlProgram::try_parse_from_account_data(
                            &source_account.data,
                        )
                    })
                    .transpose()?
                    .map(Arc::new)
            }
        };
        self.cached_programs
            .insert(*program_id, idl_program.clone());
        Ok(idl_program)
    }

    // TODO - support resolve_execution ?
    // TODO - resolve account datastructure would be better?

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
            .resolve_program(endpoint, &account_owner)
            .await?
            .unwrap_or_default()
            .guess_account(&account_data)
            .unwrap_or_default();
        let account_state = idl_account.decompile(&account_data)?;
        Ok(Some((idl_account, account_state)))
    }

    pub async fn resolve_instruction(
        &mut self,
        endpoint: &mut ToolboxEndpoint,
        program_id: &Pubkey,
        instruction_name: &str,
        instruction_payload: &Value,
        instruction_addresses: &HashMap<String, Pubkey>,
    ) -> Result<Instruction, ToolboxIdlError> {
        let idl_program = self
            .resolve_program(endpoint, program_id)
            .await?
            .unwrap_or_default();
        let idl_instruction = idl_program
            .instructions
            .get(instruction_name)
            .cloned()
            .unwrap_or_default();
        idl_instruction.compile(
            program_id,
            instruction_payload,
            &self
                .resolve_custom_instruction_addresses(
                    endpoint,
                    program_id,
                    &idl_instruction,
                    instruction_payload,
                    instruction_addresses,
                )
                .await?,
        )
    }

    pub async fn resolve_instruction_addresses(
        &mut self,
        endpoint: &mut ToolboxEndpoint,
        program_id: &Pubkey,
        instruction_name: &str,
        instruction_payload: &Value,
        instruction_addresses: &HashMap<String, Pubkey>,
    ) -> Result<HashMap<String, Pubkey>, ToolboxIdlError> {
        let idl_program = self
            .resolve_program(endpoint, program_id)
            .await?
            .unwrap_or_default();
        let idl_instruction = idl_program
            .instructions
            .get(instruction_name)
            .cloned()
            .unwrap_or_default();
        self.resolve_custom_instruction_addresses(
            endpoint,
            program_id,
            &idl_instruction,
            instruction_payload,
            instruction_addresses,
        )
        .await
    }

    pub async fn resolve_custom_instruction_addresses(
        &mut self,
        endpoint: &mut ToolboxEndpoint,
        program_id: &Pubkey,
        idl_instruction: &ToolboxIdlInstruction,
        instruction_payload: &Value,
        instruction_addresses: &HashMap<String, Pubkey>,
    ) -> Result<HashMap<String, Pubkey>, ToolboxIdlError> {
        let mut instruction_addresses = instruction_addresses.clone();
        let mut instruction_content_types_and_states = HashMap::new();
        for (instruction_account_name, instruction_address) in
            &instruction_addresses
        {
            if let Ok(Some((idl_account, account_state))) = self
                .resolve_account_details(endpoint, instruction_address)
                .await
            {
                instruction_content_types_and_states.insert(
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
                    .try_find(
                        program_id,
                        &idl_instruction.args_type_full_fields,
                        instruction_payload,
                        &instruction_addresses,
                        &instruction_content_types_and_states,
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
                        instruction_content_types_and_states.insert(
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
