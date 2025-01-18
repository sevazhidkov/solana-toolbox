use std::collections::HashMap;
use std::str::FromStr;

use serde_json::Map;
use serde_json::Value;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_as_u128_or_else;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_array_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_bool;
use crate::toolbox_idl_utils::idl_object_get_key_as_object;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;
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
    instruction_args: &Map<String, Value>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<Vec<AccountMeta>, ToolboxIdlError> {
    let idl_accounts = idl_object_get_key_as_array_or_else(
        &idl.instructions_accounts,
        instruction_name,
        breadcrumbs,
    )?;
    let mut account_addresses = instruction_accounts.clone(); // TODO - remove this
    let mut account_metas = vec![];
    for idl_account in idl_accounts {
        let idl_account_object =
            idl_as_object_or_else(idl_account, breadcrumbs)?;
        let (account_name, account_address) = idl_account_object_resolve(
            idl_account_object,
            &account_addresses,
            program_id,
            breadcrumbs,
        )?;
        let account_address = *idl_ok_or_else(
            account_address.as_ref(),
            "could not be resolved",
            &account_name,
            breadcrumbs,
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
        breadcrumbs,
    )?;
    let mut account_address = account_addresses.get(idl_account_name).cloned();
    if account_address.is_none() {
        if let Some(idl_account_address) =
            idl_object_get_key_as_str(idl_account_object, "address")
        {
            account_address = Some(
                Pubkey::from_str(idl_account_address)
                    .map_err(ToolboxIdlError::ParsePubkey)?,
            );
        }
    }
    if account_address.is_none() {
        if let Some(idl_account_pda) =
            idl_object_get_key_as_object(idl_account_object, "pda")
        {
            if let Some(idl_account_seeds) =
                idl_object_get_key_as_array(idl_account_pda, "seeds")
            {
                let mut account_seeds = vec![];
                for idl_account_seed in idl_account_seeds {
                    let account_seed = idl_account_seed_serialized(
                        idl_account_seed,
                        account_addresses,
                        breadcrumbs,
                    )?;
                    account_seeds.push(account_seed);
                }
                let mut account_seeds_slices = vec![];
                for account_seed in &account_seeds {
                    account_seeds_slices.push(&account_seed[..]);
                }
                account_address = Some(
                    Pubkey::find_program_address(
                        &account_seeds_slices,
                        program_id,
                    )
                    .0,
                )
            }
        }
    }
    Ok((idl_account_name.to_owned(), account_address))
}

fn idl_account_seed_serialized(
    idl_account_seed: &Value,
    account_addresses: &HashMap<String, Pubkey>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<Vec<u8>, ToolboxIdlError> {
    let idl_account_seed_object =
        idl_as_object_or_else(idl_account_seed, breadcrumbs)?;
    let idl_account_seed_kind = idl_object_get_key_as_str_or_else(
        idl_account_seed_object,
        "kind",
        breadcrumbs,
    )?;
    match idl_account_seed_kind {
        "const" => {
            let idl_account_seed_value = idl_object_get_key_as_array_or_else(
                idl_account_seed_object,
                "value",
                breadcrumbs,
            )?;
            // TODO - supported typed consts
            let mut account_seed = vec![];
            for idl_account_seed_byte in idl_account_seed_value {
                account_seed.push(
                    u8::try_from(idl_as_u128_or_else(
                        idl_account_seed_byte,
                        breadcrumbs,
                    )?)
                    .map_err(ToolboxIdlError::TryFromInt)?,
                );
            }
            Ok(account_seed)
        },
        "account" => {
            let idl_account_seed_path = idl_object_get_key_as_str_or_else(
                idl_account_seed_object,
                "path",
                breadcrumbs,
            )?;
            // TODO - Support typed accounts
            // TODO - don't use this utility function
            let account_address = idl_ok_or_else(
                account_addresses.get(idl_account_seed_path),
                "path could not be looked up",
                idl_account_seed_path,
                breadcrumbs,
            )?;
            Ok(account_address.to_bytes().into())
        },
        "arg" => {
            let _idl_account_seed_path = idl_object_get_key_as_str_or_else(
                idl_account_seed_object,
                "path",
                breadcrumbs,
            )?;
            // TODO - proper arg parsing
            idl_err(
                format!(
                    "arg account seed not implemented yet: {:?}",
                    idl_account_seed_object,
                ),
                breadcrumbs,
            )
        },
        _ => idl_err(
            format!("account seed kind unknown: {}", idl_account_seed_kind),
            breadcrumbs,
        ),
    }
}
