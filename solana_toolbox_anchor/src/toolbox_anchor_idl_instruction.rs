use std::collections::HashMap;
use std::str::FromStr;

use solana_sdk::instruction::AccountMeta;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_anchor_endpoint::ToolboxAnchorEndpoint;
use crate::toolbox_anchor_error::ToolboxAnchorError;
use crate::toolbox_anchor_idl::ToolboxAnchorIdl;
use crate::toolbox_anchor_idl_utils::json_object_get_key_as_object;
use crate::toolbox_anchor_idl_utils::{
    json_object_get_key_as_array, json_object_get_key_as_bool,
    json_object_get_key_as_str,
};

impl ToolboxAnchorEndpoint {
    pub async fn generate_anchor_idl_instruction(
        &mut self,
        idl: &ToolboxAnchorIdl,
        instruction_name: &str,
        known_accounts: &HashMap<&str, Pubkey>,
    ) -> Result<Instruction, ToolboxAnchorError> {
        let accounts = self
            .generate_anchor_idl_instruction_accounts(
                idl,
                instruction_name,
                known_accounts,
            )
            .await;
        let data =
            self.generate_anchor_idl_instruction_data(idl, instruction_name);
        eprintln!("accounts:{:#X?}", accounts); // TODO - cleanup prints
        eprintln!("data:{:#?}", data);
        Ok(Instruction {
            program_id: idl.program_id,
            accounts: accounts?,
            data: data?,
        })
    }

    pub fn generate_anchor_idl_instruction_data(
        &mut self,
        idl: &ToolboxAnchorIdl,
        instruction_name: &str,
    ) -> Result<Vec<u8>, ToolboxAnchorError> {
        let mut data = vec![];
        let discriminator =
            self.compute_anchor_instruction_discriminator(instruction_name);
        eprintln!("discriminator:{:#X?}", discriminator);
        data.extend_from_slice(&discriminator.to_le_bytes());
        let idl_args = json_object_get_key_as_array(
            &idl.instructions_args,
            instruction_name,
        )
        .ok_or_else(|| {
            ToolboxAnchorError::Custom(format!(
                "IDL doesn't have args for instruction: {}",
                instruction_name
            ))
        })?;

        for idl_arg in idl_args {
            println!("idl_arg:{:#?}", idl_arg);
        }

        // TODO - helper for generating an instruction's data
        Ok(data)
    }

    pub async fn generate_anchor_idl_instruction_accounts(
        &mut self,
        idl: &ToolboxAnchorIdl,
        instruction_name: &str,
        known_accounts: &HashMap<&str, Pubkey>,
    ) -> Result<Vec<AccountMeta>, ToolboxAnchorError> {
        let idl_accounts = json_object_get_key_as_array(
            &idl.instructions_accounts,
            instruction_name,
        )
        .ok_or_else(|| {
            ToolboxAnchorError::Custom(format!(
                "IDL doesn't have accounts for instruction: {}",
                instruction_name
            ))
        })?;
        let mut account_addresses = known_accounts.clone();
        let mut account_metas = vec![];
        for idl_account in idl_accounts {
            let idl_account_object =
                idl_account.as_object().ok_or_else(|| {
                    ToolboxAnchorError::Custom(format!(
                        "IDL instruction account is malformed: {}",
                        idl_account
                    ))
                })?;
            let idl_account_name =
                json_object_get_key_as_str(idl_account_object, "name")
                    .ok_or_else(|| {
                        ToolboxAnchorError::Custom(format!(
                            "IDL instruction account has no name: {}",
                            idl_account
                        ))
                    })?;
            let idl_account_is_signer =
                json_object_get_key_as_bool(idl_account_object, "signer")
                    .or(json_object_get_key_as_bool(
                        idl_account_object,
                        "isSigner",
                    ))
                    .unwrap_or(false);
            let idl_account_is_writable =
                json_object_get_key_as_bool(idl_account_object, "writable")
                    .or(json_object_get_key_as_bool(
                        idl_account_object,
                        "isMut",
                    ))
                    .unwrap_or(false);
            println!("idl_account:{:#?}", idl_account);
            let mut account_address =
                known_accounts.get(idl_account_name).cloned();
            if account_address.is_none() {
                if let Some(idl_account_address) =
                    json_object_get_key_as_str(idl_account_object, "address")
                {
                    account_address = Some(
                        Pubkey::from_str(idl_account_address)
                            .map_err(ToolboxAnchorError::ParsePubkey)?,
                    );
                }
            }
            if account_address.is_none() {
                if let Some(idl_account_pda) =
                    json_object_get_key_as_object(idl_account_object, "pda")
                {
                    if let Some(idl_account_seeds) =
                        json_object_get_key_as_array(idl_account_pda, "seeds")
                    {
                        let mut account_seeds = vec![];
                        for idl_account_seed in idl_account_seeds {
                            let idl_account_seed_object =
                            idl_account_seed.as_object().ok_or_else(|| {
                                ToolboxAnchorError::Custom(format!(
                                    "IDL instruction account seed is malformed: {}",
                                    idl_account_seed
                                ))
                            })?;
                            let mut account_seed = vec![];
                            let idl_account_seed_kind =
                                json_object_get_key_as_str(
                                    idl_account_seed_object,
                                    "kind",
                                ).ok_or_else(|| {
                                    ToolboxAnchorError::Custom(format!(
                                        "IDL instruction account seed has no kind: {}",
                                        idl_account_seed
                                    ))
                                })?;
                            match idl_account_seed_kind {
                                "const" => {
                                    let idl_account_seed_value =
                                        json_object_get_key_as_array(
                                            idl_account_seed_object,
                                            "value",
                                        ).ok_or_else(|| ToolboxAnchorError::Custom(
                                            format!(
                                                "IDL unknown account seed const has no value: {}",
                                                idl_account_seed
                                            ),
                                        ))?;
                                    for idl_account_seed_byte in
                                        idl_account_seed_value
                                    {
                                        account_seed.push(
                                            u8::try_from(
                                                idl_account_seed_byte.as_u64().ok_or_else(|| ToolboxAnchorError::Custom(
                                                    format!(
                                                        "IDL invalid account seed const value: {}",
                                                        idl_account_seed_byte
                                                    ),
                                                ))?
                                            ).map_err(ToolboxAnchorError::TryFromInt)?
                                        );
                                    }
                                },
                                "account" => {
                                    let idl_account_seed_path = json_object_get_key_as_str(idl_account_seed_object, "path").ok_or_else(|| ToolboxAnchorError::Custom(
                                        format!(
                                            "IDL account seed account has no path: {}",
                                            idl_account_seed
                                        ),
                                    ))?;
                                    let account_address = account_addresses.get(idl_account_seed_path).ok_or_else(|| ToolboxAnchorError::Custom(
                                        format!(
                                            "IDL unknown account seed account path: {}",
                                            idl_account_seed_path
                                        ),
                                    ))?;
                                    account_seed.extend_from_slice(
                                        &account_address.to_bytes(),
                                    );
                                },
                                _ => {
                                    return Err(ToolboxAnchorError::Custom(
                                        format!(
                                            "IDL unknown account seed kind: {}",
                                            idl_account_seed_kind
                                        ),
                                    ));
                                },
                            }
                            account_seeds.push(account_seed);
                        }
                        let mut account_seeds_slices = vec![];
                        for account_seed in &account_seeds {
                            account_seeds_slices.push(&account_seed[..]);
                        }
                        account_address = Some(
                            Pubkey::find_program_address(
                                &account_seeds_slices,
                                &idl.program_id,
                            )
                            .0,
                        )
                    }
                }
            }
            let account_address = account_address.ok_or_else(|| {
                ToolboxAnchorError::Custom(format!(
                    "Could not resolve account: {}",
                    idl_account_name
                ))
            })?;
            account_addresses.insert(idl_account_name, account_address);
            println!("account_address:{:?}", account_address);
            if idl_account_is_writable {
                account_metas.push(AccountMeta::new(
                    account_address,
                    idl_account_is_signer,
                ));
            } else {
                account_metas.push(AccountMeta::new_readonly(
                    account_address,
                    idl_account_is_signer,
                ));
            }
        }
        Ok(account_metas)
    }
}
