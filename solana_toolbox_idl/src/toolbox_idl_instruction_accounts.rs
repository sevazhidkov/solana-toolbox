use std::collections::HashMap;
use std::vec;

use convert_case::Case;
use convert_case::Casing;
use serde_json::Map;
use serde_json::Value;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_account::ToolboxIdlAccount;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_context::ToolboxIdlContext;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_instruction::ToolboxIdlInstruction;
use crate::toolbox_idl_program_instruction_account::ToolboxIdlProgramInstructionAccountResolve;
use crate::toolbox_idl_program_instruction_account::ToolboxIdlProgramInstructionAccountResolvePdaBlob;
use crate::toolbox_idl_program_typedef::ToolboxIdlProgramTypedef;
use crate::toolbox_idl_program_typedef_primitive::ToolboxIdlProgramTypedefPrimitiveKind;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;
use crate::toolbox_idl_utils::idl_ok_or_else;

impl ToolboxIdl {
    pub fn list_instruction_accounts_names(
        &self,
        instruction_name: &str,
    ) -> Result<Vec<String>, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let mut instruction_accounts_names = vec![];
        for program_instruction_account in &idl_map_get_key_or_else(
            &self.program_instructions,
            instruction_name,
            &breadcrumbs.as_idl("$program_instructions"),
        )?
        .accounts
        {
            instruction_accounts_names
                .push(program_instruction_account.name.to_string());
        }
        Ok(instruction_accounts_names)
    }

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
        let instruction_accounts_names =
            self.list_instruction_accounts_names(&instruction.name)?;
        loop {
            let mut made_progress = false;
            for instruction_account_name in &instruction_accounts_names {
                if instruction_accounts_addresses
                    .contains_key(instruction_account_name)
                {
                    continue;
                }
                if let Ok(instruction_account_address) = self
                    .find_instruction_account_address(
                        instruction_account_name,
                        instruction,
                        &instruction_accounts,
                    )
                {
                    made_progress = true;
                    instruction_accounts_addresses.insert(
                        instruction_account_name.to_string(),
                        instruction_account_address,
                    );
                    if let Ok(Some(instruction_account)) = self
                        .get_account(endpoint, &instruction_account_address)
                        .await
                    {
                        instruction_accounts.insert(
                            instruction_account_name.to_string(),
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
        let mut instruction_accounts_addresses =
            instruction.accounts_addresses.clone();
        for instruction_account_name in
            self.list_instruction_accounts_names(&instruction.name)?
        {
            if !instruction_accounts_addresses
                .contains_key(&instruction_account_name)
            {
                instruction_accounts_addresses.insert(
                    instruction_account_name.to_string(),
                    self.find_instruction_account_address(
                        &instruction_account_name,
                        instruction,
                        instruction_accounts,
                    )?,
                );
            }
        }
        Ok(instruction_accounts_addresses)
    }

    pub fn find_instruction_account_address(
        &self,
        account_name: &str,
        instruction: &ToolboxIdlInstruction,
        instruction_accounts: &HashMap<String, ToolboxIdlAccount>,
    ) -> Result<Pubkey, ToolboxIdlError> {
        idl_instruction_account_address_resolve(
            self,
            account_name,
            instruction,
            instruction_accounts,
            &ToolboxIdlBreadcrumbs::default(),
        )
    }

    pub fn compile_instruction_accounts(
        &self,
        instruction_name: &str,
        instruction_accounts_addresses: &HashMap<String, Pubkey>,
    ) -> Result<Vec<AccountMeta>, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let mut account_metas = vec![];
        for program_instruction_account in &idl_map_get_key_or_else(
            &self.program_instructions,
            instruction_name,
            &breadcrumbs.as_idl("$program_instructions"),
        )?
        .accounts
        {
            let instruction_account_address = *idl_map_get_key_or_else(
                instruction_accounts_addresses,
                &program_instruction_account.name,
                &breadcrumbs.as_val("instruction_accounts_addresses"),
            )?;
            if program_instruction_account.is_writable {
                account_metas.push(AccountMeta::new(
                    instruction_account_address,
                    program_instruction_account.is_signer,
                ));
            } else {
                account_metas.push(AccountMeta::new_readonly(
                    instruction_account_address,
                    program_instruction_account.is_signer,
                ));
            }
        }
        Ok(account_metas)
    }

    pub fn decompile_instruction_accounts(
        &self,
        instruction_name: &str,
        instruction_accounts: &[AccountMeta],
    ) -> Result<HashMap<String, Pubkey>, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let instruction_accounts_names =
            self.list_instruction_accounts_names(instruction_name)?;
        if instruction_accounts_names.len() != instruction_accounts.len() {
            return idl_err(
                "Invalid instruction accounts length",
                &breadcrumbs.val(),
            );
        }
        let mut instruction_accounts_addresses = HashMap::new();
        for (instruction_account_name, instruction_account_meta) in
            instruction_accounts_names
                .into_iter()
                .zip(instruction_accounts.iter())
        {
            instruction_accounts_addresses.insert(
                instruction_account_name,
                instruction_account_meta.pubkey,
            );
        }
        Ok(instruction_accounts_addresses)
    }
}

// TODO - fix naming
fn idl_instruction_account_address_resolve(
    idl: &ToolboxIdl,
    account_name: &str,
    instruction: &ToolboxIdlInstruction,
    instruction_accounts: &HashMap<String, ToolboxIdlAccount>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<Pubkey, ToolboxIdlError> {
    for program_instruction_account in &idl_map_get_key_or_else(
        &idl.program_instructions,
        &instruction.name,
        &breadcrumbs.as_idl("$program_instructions"),
    )?
    .accounts
    {
        if program_instruction_account.name.to_case(Case::Snake)
            == account_name.to_case(Case::Snake)
        {
            if let Some(instruction_accounts_address) = instruction
                .accounts_addresses
                .get(&program_instruction_account.name)
            {
                return Ok(*instruction_accounts_address);
            }
            return idl_instruction_account_object_resolve(
                idl,
                &program_instruction_account.resolve,
                instruction,
                instruction_accounts,
                &breadcrumbs.with_idl("resolve"),
            );
        }
    }
    idl_err("Unknown account name", &breadcrumbs.as_val(account_name))
}

// TODO - naming fix
fn idl_instruction_account_object_resolve(
    idl: &ToolboxIdl,
    idl_instruction_account_resolve: &ToolboxIdlProgramInstructionAccountResolve,
    instruction: &ToolboxIdlInstruction,
    instruction_accounts: &HashMap<String, ToolboxIdlAccount>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<Pubkey, ToolboxIdlError> {
    match idl_instruction_account_resolve {
        ToolboxIdlProgramInstructionAccountResolve::Address(pubkey) => {
            Ok(*pubkey)
        },
        ToolboxIdlProgramInstructionAccountResolve::Pda { seeds, program } => {
            let mut pda_seeds_bytes = vec![];
            for pda_seed_blob in seeds {
                pda_seeds_bytes.push(idl_blob_bytes(
                    idl,
                    pda_seed_blob,
                    instruction,
                    instruction_accounts,
                    breadcrumbs,
                )?);
            }
            let pda_program_id = if let Some(pda_program_blob) = program {
                let pda_program_id_bytes = idl_blob_bytes(
                    idl,
                    pda_program_blob,
                    instruction,
                    instruction_accounts,
                    &breadcrumbs.with_idl("program"),
                )?;
                Pubkey::new_from_array(
                    pda_program_id_bytes.try_into().map_err(|err| {
                        ToolboxIdlError::Custom {
                            failure: format!("value:{:?}", err), // TODO - better error handling and breadcrumbs
                            context: breadcrumbs.as_idl("program_id"),
                        }
                    })?,
                )
            } else {
                instruction.program_id
            };
            let mut pda_seeds_slices = vec![];
            for pda_seed_bytes in pda_seeds_bytes.iter() {
                pda_seeds_slices.push(&pda_seed_bytes[..]);
            }
            Ok(Pubkey::find_program_address(&pda_seeds_slices, &pda_program_id)
                .0)
        },
        ToolboxIdlProgramInstructionAccountResolve::Unresolvable => {
            idl_err("Unresolvable account", &breadcrumbs.as_idl("@"))
        },
    }
}

// TODO - naming fix
fn idl_blob_bytes(
    idl: &ToolboxIdl,
    idl_pda_blob: &ToolboxIdlProgramInstructionAccountResolvePdaBlob,
    instruction: &ToolboxIdlInstruction,
    instruction_accounts: &HashMap<String, ToolboxIdlAccount>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<Vec<u8>, ToolboxIdlError> {
    match idl_pda_blob {
        ToolboxIdlProgramInstructionAccountResolvePdaBlob::Const { bytes } => {
            Ok(bytes.clone())
        },
        ToolboxIdlProgramInstructionAccountResolvePdaBlob::Account { path } => {
            let idl_blob_parts = Vec::from_iter(path.split("."));
            if idl_blob_parts.len() == 1 {
                return idl_instruction_account_address_resolve(
                    idl,
                    path,
                    instruction,
                    instruction_accounts,
                    breadcrumbs,
                )
                .map(|address| address.to_bytes().into());
            }
            let account_name = idl_blob_parts[0];
            let account = idl_ok_or_else(
                instruction_accounts.get(account_name).or_else(|| {
                    instruction_accounts.get(&account_name.to_case(Case::Camel))
                }),
                "Missing account value",
                &breadcrumbs.as_val(account_name),
            )?;
            let account_object = idl_as_object_or_else(
                &account.value,
                &breadcrumbs.as_val(account_name),
            )?;
            let program_account = idl_map_get_key_or_else(
                &idl.program_accounts,
                &account.name,
                &breadcrumbs.as_idl("$program_accounts"),
            )?;
            let program_typedef_struct_fields =
                idl_typedef_as_struct_fields_or_else(
                    &program_account.typedef,
                    &breadcrumbs.as_idl(&account.name),
                )?;
            idl_parts_to_bytes(
                idl,
                program_typedef_struct_fields,
                &idl_blob_parts[1..],
                account_object,
                &breadcrumbs.with_idl("account"),
            )
        },
        ToolboxIdlProgramInstructionAccountResolvePdaBlob::Arg { path } => {
            let idl_blob_parts = Vec::from_iter(path.split("."));
            let program_instruction_args = &idl_map_get_key_or_else(
                &idl.program_instructions,
                &instruction.name,
                &breadcrumbs.as_idl("$program_instructions"),
            )?
            .args;
            idl_parts_to_bytes(
                idl,
                program_instruction_args,
                &idl_blob_parts,
                &instruction.args,
                &breadcrumbs.with_idl("arg"),
            )
        },
    }
}

fn idl_parts_to_bytes(
    idl: &ToolboxIdl,
    program_fields: &[(String, ToolboxIdlProgramTypedef)],
    parts: &[&str],
    object: &Map<String, Value>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<Vec<u8>, ToolboxIdlError> {
    let field_name = parts[0];
    for (program_field_name, program_field_typedef) in program_fields {
        let breadcrumbs = &breadcrumbs.with_idl(program_field_name);
        if program_field_name.to_case(Case::Snake)
            == field_name.to_case(Case::Snake)
        {
            let value = idl_object_get_key_or_else(
                object,
                program_field_name,
                &breadcrumbs.val(),
            )?;
            if parts.len() == 1 {
                let mut bytes = vec![];
                program_field_typedef.try_serialize(
                    idl,
                    value,
                    &mut bytes,
                    &breadcrumbs.with_val(program_field_name),
                )?;
                if let Some(kind) = program_field_typedef.as_primitive_kind() {
                    if kind == &ToolboxIdlProgramTypedefPrimitiveKind::String {
                        bytes.drain(0..4);
                    }
                }
                return Ok(bytes);
            } else {
                return idl_parts_to_bytes_recurse(
                    idl,
                    program_field_typedef,
                    parts,
                    &value,
                    &breadcrumbs.with_val("*"),
                );
            }
        }
    }
    idl_err("Unknown value field", &breadcrumbs.as_val(field_name))
}

fn idl_parts_to_bytes_recurse(
    idl: &ToolboxIdl,
    idl_type: &ToolboxIdlProgramTypedef,
    parts: &[&str],
    value: &&Value,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<Vec<u8>, ToolboxIdlError> {
    match idl_type {
        ToolboxIdlProgramTypedef::Defined { name } => {
            let program_typedef = idl_map_get_key_or_else(
                &idl.program_typedefs,
                name,
                &breadcrumbs.as_idl("$program_typedefs"),
            )?;
            // TODO - what if the lookup points to an enum or vec/array ?
            let program_typedef_struct_fields =
                idl_typedef_as_struct_fields_or_else(
                    program_typedef,
                    &breadcrumbs.as_idl(name),
                )?;
            let object = idl_as_object_or_else(value, &breadcrumbs.val())?;
            idl_parts_to_bytes(
                idl,
                program_typedef_struct_fields,
                &parts[1..],
                object,
                &breadcrumbs.with_idl("*"),
            )
        },
        _ => {
            idl_err(
                "doesnt support 2+ split path (unless nested structs)",
                &breadcrumbs.as_idl(&parts.join(".")),
            )
        },
    }
}

fn idl_typedef_as_struct_fields_or_else<'a>(
    idl_type: &'a ToolboxIdlProgramTypedef,
    context: &ToolboxIdlContext,
) -> Result<&'a [(String, ToolboxIdlProgramTypedef)], ToolboxIdlError> {
    let program_fields = idl_ok_or_else(
        idl_type.as_struct_fields(),
        "Type was expected to be a struct",
        context,
    )?;
    Ok(program_fields)
}
