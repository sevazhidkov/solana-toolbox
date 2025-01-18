use std::collections::HashMap;
use std::str::FromStr;

use convert_case::Case;
use convert_case::Casing;
use serde_json::Map;
use serde_json::Value;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_as_u128_or_else;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_object_get_key_as_bool;
use crate::toolbox_idl_utils::idl_object_get_key_as_object;
use crate::toolbox_idl_utils::idl_object_get_key_as_object_array_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;
use crate::toolbox_idl_utils::idl_ok_or_else;

impl ToolboxIdl {
    pub fn generate_instruction_accounts(
        &self,
        program_id: &Pubkey,
        instruction_name: &str,
        instruction_accounts: &HashMap<String, Pubkey>,
        instruction_args: &Map<String, Value>,
    ) -> Result<Vec<AccountMeta>, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        generate_instruction_account_metas(
            self,
            program_id,
            instruction_name,
            instruction_accounts,
            instruction_args,
            breadcrumbs,
        )
    }
}

fn generate_instruction_account_metas(
    idl: &ToolboxIdl,
    program_id: &Pubkey,
    instruction_name: &str,
    instruction_accounts: &HashMap<String, Pubkey>, // TODO - this needs to be iteratively resolved
    instruction_args: &Map<String, Value>, // TODO - need to support args lookup
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<Vec<AccountMeta>, ToolboxIdlError> {
    let mut account_addresses = instruction_accounts.clone(); // TODO - remove this
    let mut account_metas = vec![];
    let idl_accounts_objects = idl_object_get_key_as_object_array_or_else(
        &idl.instructions_accounts,
        instruction_name,
        &breadcrumbs.as_idl("instruction_accounts"),
    )?;
    for index in 0..idl_accounts_objects.len() {
        let idl_account_object = idl_accounts_objects.get(index).unwrap();
        let (account_name, account_address) = idl_account_object_resolve(
            idl_account_object,
            &account_addresses,
            program_id,
            breadcrumbs,
        )?;
        let account_address = *idl_ok_or_else(
            account_address.as_ref(),
            "unresolved account",
            &breadcrumbs.as_idl(&account_name),
        )?;
        account_addresses.insert(account_name, account_address);
        let idl_account_is_signer =
            idl_object_get_key_as_bool(idl_account_object, "signer")
                .or(idl_object_get_key_as_bool(idl_account_object, "isSigner"))
                .unwrap_or(false);
        let idl_account_is_writable =
            idl_object_get_key_as_bool(idl_account_object, "writable")
                .or(idl_object_get_key_as_bool(idl_account_object, "isMut"))
                .unwrap_or(false);
        if idl_account_is_writable {
            account_metas
                .push(AccountMeta::new(account_address, idl_account_is_signer));
        } else {
            account_metas.push(AccountMeta::new_readonly(
                account_address,
                idl_account_is_signer,
            ));
        }
    }
    Ok(account_metas)
}

fn idl_account_object_resolve(
    idl_account_object: &Map<String, Value>,
    account_addresses: &HashMap<String, Pubkey>,
    program_id: &Pubkey,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(String, Option<Pubkey>), ToolboxIdlError> {
    let idl_account_name = idl_object_get_key_as_str_or_else(
        idl_account_object,
        "name",
        &breadcrumbs.as_idl("&"),
    )?;
    let breadcrumbs = &breadcrumbs.with_idl(idl_account_name);
    let mut account_address = account_addresses.get(idl_account_name).cloned();
    if account_address.is_none() {
        if let Some(idl_account_address) =
            idl_object_get_key_as_str(idl_account_object, "address")
        {
            account_address =
                Some(Pubkey::from_str(idl_account_address).map_err(|err| {
                    ToolboxIdlError::InvalidPubkey {
                        parsing: err,
                        context: breadcrumbs.as_val("address"),
                    }
                })?);
        }
    }
    if account_address.is_none() {
        if let Some(idl_account_pda) =
            idl_object_get_key_as_object(idl_account_object, "pda")
        {
            let mut pda_seeds_bytes = vec![];
            let mut pda_program_id = *program_id;
            let idl_account_seeds_objects =
                idl_object_get_key_as_object_array_or_else(
                    idl_account_pda,
                    "seeds",
                    &breadcrumbs.as_idl("pda"),
                )?;
            for index in 0..idl_account_seeds_objects.len() {
                let idl_account_seed_object =
                    idl_account_seeds_objects.get(index).unwrap();
                let pda_seed_bytes = idl_blob_bytes(
                    idl_account_seed_object,
                    account_addresses,
                    &breadcrumbs.with_idl(&format!("seed[{}]", index)),
                )?;
                pda_seeds_bytes.push(pda_seed_bytes);
            }
            if let Some(idl_account_program) =
                idl_object_get_key_as_object(idl_account_pda, "program")
            {
                let program_id_bytes = idl_blob_bytes(
                    idl_account_program,
                    account_addresses,
                    &breadcrumbs.with_idl("program"),
                )?;
                pda_program_id = Pubkey::new_from_array(
                    program_id_bytes.try_into().map_err(|err| {
                        ToolboxIdlError::Custom {
                            failure: format!("value:{:?}", err),
                            context: breadcrumbs.as_idl("program_id"),
                        }
                    })?,
                );
            }
            let mut pda_seeds_slices = vec![];
            for pda_seed_bytes in pda_seeds_bytes.iter() {
                pda_seeds_slices.push(&pda_seed_bytes[..]);
            }
            account_address = Some(
                Pubkey::find_program_address(
                    &pda_seeds_slices,
                    &pda_program_id,
                )
                .0,
            );
        }
    }
    Ok((idl_account_name.to_owned(), account_address))
}

fn idl_blob_bytes(
    idl_blob_object: &Map<String, Value>,
    account_addresses: &HashMap<String, Pubkey>,
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
            if let Some(idl_blob_value_array) = idl_blob_value.as_array() {
                let mut bytes = vec![];
                for index in 0..idl_blob_value_array.len() {
                    let idl_blob_value_byte =
                        idl_blob_value_array.get(index).unwrap();
                    let idl_blob_value_byte_tag = &format!("value[{}]", index);
                    let idl_blob_value_byte_integer = idl_as_u128_or_else(
                        idl_blob_value_byte,
                        &breadcrumbs.as_idl(idl_blob_value_byte_tag),
                    )?;
                    let idl_blob_value_byte_casted = u8::try_from(
                        idl_blob_value_byte_integer,
                    )
                    .map_err(|err| ToolboxIdlError::InvalidInteger {
                        conversion: err,
                        context: breadcrumbs.as_idl(idl_blob_value_byte_tag),
                    })?;
                    bytes.push(idl_blob_value_byte_casted);
                }
                return Ok(bytes);
            }
            idl_err("Unknown value", &breadcrumbs.as_idl("datadef(const)"))
        },
        "account" => {
            let idl_blob_path = idl_object_get_key_as_str_or_else(
                idl_blob_object,
                "path",
                &breadcrumbs.as_idl("datadef(account)"),
            )?;
            // TODO - Support typed accounts and account contents
            let account_address = match account_addresses
                .get(&idl_blob_path.to_case(Case::Camel))
            {
                Some(account_address_camel) => account_address_camel,
                None => idl_ok_or_else(
                    account_addresses.get(idl_blob_path),
                    "address not found",
                    &breadcrumbs.as_val(idl_blob_path),
                )?,
            };
            Ok(account_address.to_bytes().into())
        },
        "arg" => {
            let idl_blob_path = idl_object_get_key_as_str_or_else(
                idl_blob_object,
                "path",
                &breadcrumbs.as_idl("arg"),
            )?;
            // TODO - proper arg parsing and arg contents
            idl_err(
                "account seed arg not implemented yet",
                &breadcrumbs.as_idl(idl_blob_path),
            )
        },
        _ => idl_err(
            "Expected a 'kind' value of: const/account/arg",
            &breadcrumbs.as_idl(idl_blob_kind),
        ),
    }
}
