use std::collections::HashMap;
use std::str::FromStr;
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
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_type::ToolboxIdlType;
use crate::toolbox_idl_instruction::ToolboxIdlInstruction;

use crate::toolbox_idl_utils::idl_as_bytes_or_else;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_bool;
use crate::toolbox_idl_utils::idl_object_get_key_as_object;
use crate::toolbox_idl_utils::idl_object_get_key_as_object_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_scoped_named_content_array_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_scoped_named_object_array_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_scoped_object_array_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;
use crate::toolbox_idl_utils::idl_ok_or_else;
use crate::toolbox_idl_utils::idl_value_as_str_or_object_with_name_as_str_or_else;

impl ToolboxIdl {
    pub fn list_instruction_accounts_names(
        &self,
        instruction_name: &str,
    ) -> Result<Vec<String>, ToolboxIdlError> {
        let mut instruction_accounts_names = vec![];
        for (idl_account_name, ..) in
            idl_object_get_key_as_scoped_named_object_array_or_else(
                &self.instructions_accounts,
                instruction_name,
                &ToolboxIdlBreadcrumbs::default()
                    .with_idl("instruction_accounts"),
            )?
        {
            instruction_accounts_names.push(idl_account_name.to_string());
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
        for (idl_account_name, idl_account_object, breadcrumbs) in
            idl_object_get_key_as_scoped_named_object_array_or_else(
                &self.instructions_accounts,
                instruction_name,
                &breadcrumbs.with_idl("instruction_accounts"),
            )?
        {
            let instruction_account_address = *idl_map_get_key_or_else(
                instruction_accounts_addresses,
                idl_account_name,
                &breadcrumbs.as_val("instruction_accounts_addresses"),
            )?;
            let idl_account_is_signer =
                idl_object_get_key_as_bool(idl_account_object, "signer")
                    .or(idl_object_get_key_as_bool(
                        idl_account_object,
                        "isSigner",
                    ))
                    .unwrap_or(false);
            let idl_account_is_writable =
                idl_object_get_key_as_bool(idl_account_object, "writable")
                    .or(idl_object_get_key_as_bool(idl_account_object, "isMut"))
                    .unwrap_or(false);
            if idl_account_is_writable {
                account_metas.push(AccountMeta::new(
                    instruction_account_address,
                    idl_account_is_signer,
                ));
            } else {
                account_metas.push(AccountMeta::new_readonly(
                    instruction_account_address,
                    idl_account_is_signer,
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

fn idl_instruction_account_address_resolve(
    idl: &ToolboxIdl,
    account_name: &str,
    instruction: &ToolboxIdlInstruction,
    instruction_accounts: &HashMap<String, ToolboxIdlAccount>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<Pubkey, ToolboxIdlError> {
    for (idl_account_name, idl_account_object, breadcrumbs) in
        idl_object_get_key_as_scoped_named_object_array_or_else(
            &idl.instructions_accounts,
            &instruction.name,
            &breadcrumbs.with_idl("instruction_accounts"),
        )?
    {
        if idl_account_name.to_case(Case::Snake)
            == account_name.to_case(Case::Snake)
        {
            if let Some(instruction_accounts_address) =
                instruction.accounts_addresses.get(idl_account_name)
            {
                return Ok(*instruction_accounts_address);
            }
            return idl_instruction_account_object_resolve(
                idl,
                idl_account_object,
                instruction,
                instruction_accounts,
                &breadcrumbs,
            );
        }
    }
    idl_err("Unknown account name", &breadcrumbs.as_val(account_name))
}

fn idl_instruction_account_object_resolve(
    idl: &ToolboxIdl,
    idl_account_object: &Map<String, Value>,
    instruction: &ToolboxIdlInstruction,
    instruction_accounts: &HashMap<String, ToolboxIdlAccount>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<Pubkey, ToolboxIdlError> {
    if let Some(idl_account_address) =
        idl_object_get_key_as_str(idl_account_object, "address")
    {
        return Pubkey::from_str(idl_account_address).map_err(|err| {
            ToolboxIdlError::InvalidPubkey {
                parsing: err,
                context: breadcrumbs.as_idl("address"),
            }
        });
    }
    if let Some(idl_account_pda) =
        idl_object_get_key_as_object(idl_account_object, "pda")
    {
        let mut pda_seeds_bytes = vec![];
        let mut pda_program_id = instruction.program_id;
        for (idl_account_seed_object, breadcrumbs) in
            idl_object_get_key_as_scoped_object_array_or_else(
                idl_account_pda,
                "seeds",
                &breadcrumbs.with_idl("pda"),
            )?
        {
            let pda_seed_bytes = idl_blob_bytes(
                idl,
                idl_account_seed_object,
                instruction,
                instruction_accounts,
                &breadcrumbs,
            )?;
            pda_seeds_bytes.push(pda_seed_bytes);
        }
        if let Some(idl_account_program_object) =
            idl_object_get_key_as_object(idl_account_pda, "program")
        {
            let program_id_bytes = idl_blob_bytes(
                idl,
                idl_account_program_object,
                instruction,
                instruction_accounts,
                &breadcrumbs.with_idl("program"),
            )?;
            pda_program_id = Pubkey::new_from_array(
                program_id_bytes.try_into().map_err(|err| {
                    ToolboxIdlError::Custom {
                        failure: format!("value:{:?}", err), // TODO - better error handling and breadcrumbs
                        context: breadcrumbs.as_idl("program_id"),
                    }
                })?,
            );
        }
        let mut pda_seeds_slices = vec![];
        for pda_seed_bytes in pda_seeds_bytes.iter() {
            pda_seeds_slices.push(&pda_seed_bytes[..]);
        }
        let pda_address =
            Pubkey::find_program_address(&pda_seeds_slices, &pda_program_id).0;
        return Ok(pda_address);
    }
    idl_err("Expected key: pda or address", &breadcrumbs.as_idl("@"))
}

fn idl_blob_bytes(
    idl: &ToolboxIdl,
    idl_blob_object: &Map<String, Value>,
    instruction: &ToolboxIdlInstruction,
    instruction_accounts: &HashMap<String, ToolboxIdlAccount>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<Vec<u8>, ToolboxIdlError> {
    let idl_blob_kind = idl_object_get_key_as_str_or_else(
        idl_blob_object,
        "kind",
        &breadcrumbs.as_idl("datadef"),
    )?;
    match idl_blob_kind {
        "const" => {
            let idl_blob_value = idl_object_get_key_or_else(
                idl_blob_object,
                "value",
                &breadcrumbs.as_idl("const"),
            )?;
            if let Some(idl_blob_value_string) = idl_blob_value.as_str() {
                return Ok(idl_blob_value_string.as_bytes().to_vec());
            }
            if idl_blob_value.is_array() {
                return idl_as_bytes_or_else(
                    idl_blob_value,
                    &breadcrumbs.as_idl("const(bytes)"),
                );
            }
            idl_err(
                "Expected an array of string as value",
                &breadcrumbs.as_idl("datadef(const)"),
            )
        },
        "account" => {
            let idl_blob_path = idl_object_get_key_as_str_or_else(
                idl_blob_object,
                "path",
                &breadcrumbs.as_idl("datadef(account)"),
            )?;
            let idl_blob_parts = Vec::from_iter(idl_blob_path.split("."));
            if idl_blob_parts.len() == 1 {
                return idl_instruction_account_address_resolve(
                    idl,
                    idl_blob_path,
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
            let idl_blob_type = idl_map_get_key_or_else(
                &idl.accounts_types,
                &account.name,
                &breadcrumbs.as_idl("$accounts_types"),
            )?;
            let idl_blob_fields = idl_type_as_struct_fields(
                idl,
                idl_blob_type,
                &breadcrumbs.as_idl(&account.name),
            )?;
            idl_parts_to_bytes(
                idl,
                idl_blob_fields,
                &idl_blob_parts[1..],
                account_object,
                &breadcrumbs.with_idl("account"),
            )
        },
        "arg" => {
            let idl_blob_path = idl_object_get_key_as_str_or_else(
                idl_blob_object,
                "path",
                &breadcrumbs.as_idl("arg"),
            )?;
            let idl_blob_parts = Vec::from_iter(idl_blob_path.split("."));
            let idl_blob_fields = idl_map_get_key_or_else(
                &idl.instructions_args,
                &instruction.name,
                &breadcrumbs.as_idl("$instructions_args"),
            )?;
            idl_parts_to_bytes(
                idl,
                idl_blob_fields,
                &idl_blob_parts,
                &instruction.args,
                &breadcrumbs.with_idl("arg"),
            )
        },
        _ => {
            idl_err(
                "Expected a 'kind' value of: const/account/arg",
                &breadcrumbs.as_idl(idl_blob_kind),
            )
        },
    }
}

fn idl_parts_to_bytes(
    idl: &ToolboxIdl,
    idl_fields: &[(String, ToolboxIdlType)],
    parts: &[&str],
    object: &Map<String, Value>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<Vec<u8>, ToolboxIdlError> {
    let field_name = parts[0];
    for (idl_field_name, idl_field_type) in idl_fields
    {
        let breadcrumbs = &breadcrumbs.with_idl(idl_field_name);
        if idl_field_name.to_case(Case::Snake)
            == field_name.to_case(Case::Snake)
        {
            let value = idl_object_get_key_or_else(
                object,
                idl_field_name,
                &breadcrumbs.val(),
            )?;
            if parts.len() == 1 {
                let mut bytes = vec![];
                idl.type_serialize(
                    idl_field_type,
                    value,
                    &mut bytes,
                    &breadcrumbs.with_val(idl_field_name),
                )?;
                if idl_field_type.as_str() == Some("string") {
                    bytes.drain(0..4);
                }
                return Ok(bytes);
            } else {
                return idl_parts_to_bytes_recurse(
                    idl,
                    idl_field_type,
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
    idl_type: &ToolboxIdlType,
    parts: &[&str],
    value: &&Value,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<Vec<u8>, ToolboxIdlError> {
    match idl_type {
        ToolboxIdlType::Defined {
            name
        } => {
            let idl_type = idl_map_get_key_or_else(
                &idl.program_types,
                name,
                &breadcrumbs.as_idl("$program_types"),
            )?;
            // TODO - what if the lookup points to an enum or vec/array ?
            let idl_type_fields = idl_type_as_struct_fields(
                idl,
                idl_type,
                &breadcrumbs.as_idl(name)
            )?;
            let object = idl_as_object_or_else(value, &breadcrumbs.val())?;
            return idl_parts_to_bytes(
                idl,
                idl_type_fields,
                &parts[1..],
                object,
                &breadcrumbs.with_idl("*"),
            );
        }
        _ => 
        idl_err(
            "doesnt support 2+ split path (unless nested structs)",
            &breadcrumbs.as_idl(&parts.join(".")),
        )
    }
}

fn idl_type_as_struct_fields<'a>(
    idl: &ToolboxIdl,
    idl_type: &'a ToolboxIdlType,
    context: &ToolboxIdlContext,
) -> Result<&'a [(String, ToolboxIdlType)], ToolboxIdlError> {
    let idl_struct = idl_ok_or_else(
        idl_type.as_struct(),
        "Type was expected to be a struct",
        context
    )?;
    Ok(idl_struct.fields)
}
