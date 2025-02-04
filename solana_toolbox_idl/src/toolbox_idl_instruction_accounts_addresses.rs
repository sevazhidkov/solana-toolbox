use std::collections::HashMap;
use std::vec;

use convert_case::Case;
use convert_case::Casing;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_account::ToolboxIdlAccount;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_context::ToolboxIdlContext;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_instruction::ToolboxIdlInstruction;
use crate::toolbox_idl_primitive::ToolboxIdlPrimitive;
use crate::toolbox_idl_program_instruction_account_pda::ToolboxIdlProgramInstructionAccountPda;
use crate::toolbox_idl_program_instruction_account_pda::ToolboxIdlProgramInstructionAccountPdaBlob;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;
use crate::toolbox_idl_utils::idl_ok_or_else;

impl ToolboxIdl {
    pub async fn resolve_instruction_accounts_addresses(
        &self,
        endpoint: &mut ToolboxEndpoint,
        instruction: &ToolboxIdlInstruction,
    ) -> Result<HashMap<String, Pubkey>, ToolboxIdlError> {
        let mut instruction_accounts_addresses =
            instruction.accounts_addresses.clone();
        let mut instruction_accounts = self
            .get_accounts_by_name(endpoint, &instruction_accounts_addresses)
            .await?;
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let program_instruction = idl_map_get_key_or_else(
            &self.program_instructions,
            &instruction.name,
            &breadcrumbs.as_idl("$program_instructions"),
        )?;
        loop {
            let mut made_progress = false;
            for program_instruction_account in &program_instruction.accounts {
                if instruction_accounts_addresses
                    .contains_key(&program_instruction_account.name)
                {
                    continue;
                }
                if let Ok(instruction_account_address) = self
                    .find_instruction_account_address(
                        instruction,
                        &instruction_accounts,
                        &program_instruction_account.name,
                    )
                {
                    made_progress = true;
                    instruction_accounts_addresses.insert(
                        program_instruction_account.name.to_string(),
                        instruction_account_address,
                    );
                    if let Ok(Some(instruction_account)) = self
                        .get_account(endpoint, &instruction_account_address)
                        .await
                    {
                        instruction_accounts.insert(
                            program_instruction_account.name.to_string(),
                            instruction_account,
                        );
                    }
                }
            }
            if !made_progress {
                break;
            }
        }
        Ok(instruction_accounts_addresses)
    }

    pub fn find_instruction_accounts_addresses(
        &self,
        instruction: &ToolboxIdlInstruction,
        instruction_accounts: &HashMap<String, ToolboxIdlAccount>,
    ) -> Result<HashMap<String, Pubkey>, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let program_instruction = idl_map_get_key_or_else(
            &self.program_instructions,
            &instruction.name,
            &breadcrumbs.as_idl("$program_instructions"),
        )?;
        let mut instruction_accounts_addresses =
            instruction.accounts_addresses.clone();
        for program_instruction_account in &program_instruction.accounts {
            if !instruction_accounts_addresses
                .contains_key(&program_instruction_account.name)
            {
                instruction_accounts_addresses.insert(
                    program_instruction_account.name.to_string(),
                    self.find_instruction_account_address(
                        instruction,
                        instruction_accounts,
                        &program_instruction_account.name,
                    )?,
                );
            }
        }
        Ok(instruction_accounts_addresses)
    }

    pub fn find_instruction_account_address(
        &self,
        instruction: &ToolboxIdlInstruction,
        instruction_accounts: &HashMap<String, ToolboxIdlAccount>,
        instruction_account_name: &str,
    ) -> Result<Pubkey, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        idl_instruction_account_address_resolve(
            self,
            instruction,
            instruction_accounts,
            instruction_account_name,
            breadcrumbs,
        )
    }
}

// TODO - naming fix
fn idl_instruction_account_address_resolve(
    idl: &ToolboxIdl,
    instruction: &ToolboxIdlInstruction,
    instruction_accounts: &HashMap<String, ToolboxIdlAccount>,
    instruction_account_name: &str,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<Pubkey, ToolboxIdlError> {
    let program_instruction = idl_map_get_key_or_else(
        &idl.program_instructions,
        &instruction.name,
        &breadcrumbs.as_idl("$program_instructions"),
    )?;
    for program_instruction_account in &program_instruction.accounts {
        if program_instruction_account.name.to_case(Case::Snake)
            == instruction_account_name.to_case(Case::Snake)
        {
            if let Some(instruction_account_address) = instruction
                .accounts_addresses
                .get(&program_instruction_account.name)
            {
                return Ok(*instruction_account_address);
            }
            if let Some(program_instruction_account_address) =
                &program_instruction_account.address
            {
                return Ok(*program_instruction_account_address);
            }
            if let Some(program_instruction_account_pda) =
                &program_instruction_account.pda
            {
                return idl_instruction_account_pda_resolve(
                    idl,
                    program_instruction_account_pda,
                    instruction,
                    instruction_accounts,
                    &breadcrumbs.with_idl("pda"),
                );
            }
            return idl_err("Unresolvable account", &breadcrumbs.as_idl("@"));
        }
    }
    idl_err(
        "Unknown instruction account name",
        &breadcrumbs.as_val(instruction_account_name),
    )
}

// TODO - naming fix
fn idl_instruction_account_pda_resolve(
    idl: &ToolboxIdl,
    program_instruction_account_pda: &ToolboxIdlProgramInstructionAccountPda,
    instruction: &ToolboxIdlInstruction,
    instruction_accounts: &HashMap<String, ToolboxIdlAccount>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<Pubkey, ToolboxIdlError> {
    let mut pda_seeds_bytes = vec![];
    for pda_seed_blob in &program_instruction_account_pda.seeds {
        pda_seeds_bytes.push(idl_instruction_account_pda_blob_resolve(
            idl,
            pda_seed_blob,
            instruction,
            instruction_accounts,
            breadcrumbs,
        )?);
    }
    let pda_program_id = if let Some(pda_program_blob) =
        &program_instruction_account_pda.program
    {
        let pda_program_id_bytes = idl_instruction_account_pda_blob_resolve(
            idl,
            pda_program_blob,
            instruction,
            instruction_accounts,
            &breadcrumbs.with_idl("program"),
        )?;
        Pubkey::new_from_array(pda_program_id_bytes.try_into().map_err(
            |err| {
                ToolboxIdlError::Custom {
                    failure: format!("value:{:?}", err), // TODO - better error handling and breadcrumbs
                    context: breadcrumbs.as_idl("program_id"),
                }
            },
        )?)
    } else {
        instruction.program_id
    };
    let mut pda_seeds_slices = vec![];
    for pda_seed_bytes in pda_seeds_bytes.iter() {
        pda_seeds_slices.push(&pda_seed_bytes[..]);
    }
    Ok(Pubkey::find_program_address(&pda_seeds_slices, &pda_program_id).0)
}

// TODO - naming fix
fn idl_instruction_account_pda_blob_resolve(
    idl: &ToolboxIdl,
    program_account_pda_blob: &ToolboxIdlProgramInstructionAccountPdaBlob,
    instruction: &ToolboxIdlInstruction,
    instruction_accounts: &HashMap<String, ToolboxIdlAccount>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<Vec<u8>, ToolboxIdlError> {
    match program_account_pda_blob {
        ToolboxIdlProgramInstructionAccountPdaBlob::Const { bytes } => {
            Ok(bytes.clone())
        },
        ToolboxIdlProgramInstructionAccountPdaBlob::Account { path } => {
            let idl_blob_parts = Vec::from_iter(path.split("."));
            if idl_blob_parts.len() == 1 {
                return idl_instruction_account_address_resolve(
                    idl,
                    instruction,
                    instruction_accounts,
                    path,
                    breadcrumbs,
                )
                .map(|address| address.to_bytes().to_vec());
            }
            let account_name = idl_blob_parts[0];
            let account = idl_ok_or_else(
                instruction_accounts.get(account_name).or_else(|| {
                    instruction_accounts.get(&account_name.to_case(Case::Camel))
                }),
                "Missing account value",
                &breadcrumbs.as_val(account_name),
            )?;
            let program_account = idl_map_get_key_or_else(
                &idl.program_accounts,
                &account.name,
                &breadcrumbs.as_idl("$program_accounts"),
            )?;
            idl_instruction_account_pda_path_resolve(
                idl,
                &program_account.data_type_full,
                &account.state,
                &idl_blob_parts[1..],
                &breadcrumbs.with_idl(&account.name).with_val(account_name),
            )
        },
        ToolboxIdlProgramInstructionAccountPdaBlob::Arg { path } => {
            let idl_blob_parts = Vec::from_iter(path.split("."));
            let program_instruction = &idl_map_get_key_or_else(
                &idl.program_instructions,
                &instruction.name,
                &breadcrumbs.as_idl("$program_instructions"),
            )?;
            idl_instruction_account_pda_path_resolve(
                idl,
                &program_instruction.data_type_full,
                &instruction.args,
                &idl_blob_parts,
                &breadcrumbs.with_idl(&instruction.name).with_idl("args"),
            )
        },
    }
}

// TODO - naming fix
fn idl_instruction_account_pda_path_resolve(
    idl: &ToolboxIdl,
    type_full: &ToolboxIdlTypeFull,
    value: &Value,
    parts: &[&str],
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<Vec<u8>, ToolboxIdlError> {
    let current = parts[0];
    // TODO - support unamed structs as arg ?
    let value_object = idl_as_object_or_else(value, &breadcrumbs.val())?;
    let named_fields =
        idl_type_full_to_named_fields_or_else(type_full, &breadcrumbs.idl())?;
    for (field_name, field_type_full) in named_fields {
        let breadcrumbs = &breadcrumbs.with_idl(field_name);
        if field_name.to_case(Case::Snake) == current.to_case(Case::Snake) {
            let value_field = idl_object_get_key_or_else(
                value_object,
                field_name,
                &breadcrumbs.val(),
            )?;
            if parts.len() == 1 {
                let mut bytes = vec![];
                field_type_full.try_serialize(
                    value_field,
                    &mut bytes,
                    &breadcrumbs.with_val(field_name),
                )?;
                // TODO - generalize this case to vec and nested structs fields
                if field_type_full.as_vec().is_some() {
                    bytes.drain(0..4);
                }
                if let Some(primitive) = field_type_full.as_primitive() {
                    if primitive == &ToolboxIdlPrimitive::String {
                        bytes.drain(0..4);
                    }
                }
                return Ok(bytes);
            }
            return idl_instruction_account_pda_path_resolve(
                idl,
                &field_type_full,
                value_field,
                &parts[1..],
                &breadcrumbs.with_idl("*"),
            );
        }
    }
    idl_err("Unknown value field", &breadcrumbs.as_val(current))
}

fn idl_type_full_to_named_fields_or_else<'a>(
    type_full: &'a ToolboxIdlTypeFull,
    context: &ToolboxIdlContext,
) -> Result<&'a Vec<(String, ToolboxIdlTypeFull)>, ToolboxIdlError> {
    match type_full {
        ToolboxIdlTypeFull::Struct {
            fields: ToolboxIdlTypeFullFields::Named(fields),
        } => Ok(fields),
        _ => return idl_err("Expected fields", context),
    }
}
