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
use crate::toolbox_idl_utils::idl_object_get_key_as_object_array_or_else;
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
        &breadcrumbs.as_idl("name"),
    )?;
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
        idl_as_object_or_else(idl_account_seed, &breadcrumbs.as_idl("seeds?"))?;
    let idl_account_seed_kind = idl_object_get_key_as_str_or_else(
        idl_account_seed_object,
        "kind",
        &breadcrumbs.as_idl("kind"),
    )?;
    match idl_account_seed_kind {
        "const" => {
            let idl_account_seed_array = idl_object_get_key_as_array_or_else(
                idl_account_seed_object,
                "value",
                &breadcrumbs.as_idl("?"),
            )?;
            // TODO - supported typed consts
            let mut account_seed = vec![];
            for index in 0..idl_account_seed_array.len() {
                let idl_account_seed_tag = &format!("[{}]", index);
                let idl_account_seed_byte =
                    idl_account_seed_array.get(index).unwrap();
                account_seed.push(
                    u8::try_from(idl_as_u128_or_else(
                        idl_account_seed_byte,
                        &&breadcrumbs.as_idl(idl_account_seed_tag),
                    )?)
                    .map_err(|err| {
                        ToolboxIdlError::InvalidInteger {
                            conversion: err,
                            context: breadcrumbs.as_idl(idl_account_seed_tag),
                        }
                    })?,
                );
            }
            Ok(account_seed)
        },
        "account" => {
            let idl_account_seed_path = idl_object_get_key_as_str_or_else(
                idl_account_seed_object,
                "path",
                &breadcrumbs.as_idl("seed"),
            )?;
            // TODO - Support typed accounts
            // TODO - don't use this utility function
            let account_address = idl_ok_or_else(
                account_addresses.get(idl_account_seed_path),
                "address not found",
                &breadcrumbs.as_idl(idl_account_seed_path),
            )?;
            Ok(account_address.to_bytes().into())
        },
        "arg" => {
            let idl_account_seed_path = idl_object_get_key_as_str_or_else(
                idl_account_seed_object,
                "path",
                &breadcrumbs.as_idl("seed"),
            )?;
            // TODO - proper arg parsing
            idl_err(
                "account seed arg not implemented yet",
                &breadcrumbs.as_idl(idl_account_seed_path),
            )
        },
        _ => idl_err(
            "unknown seed kind",
            &breadcrumbs.as_idl(idl_account_seed_kind),
        ),
    }
}
