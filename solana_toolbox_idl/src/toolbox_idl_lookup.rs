use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_object_get_key_as_bool;
use crate::toolbox_idl_utils::idl_object_get_key_as_object_array_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_object_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_u64_or_else;

#[derive(Debug, Clone)]
pub struct ToolboxIdlLookupInstruction {
    pub name: String,
    pub accounts: Vec<ToolboxIdlLookupInstructionAccount>,
    // TODO - add args and associated types
}

#[derive(Debug, Clone)]
pub struct ToolboxIdlLookupInstructionAccount {
    pub name: String,
    pub resolvable: bool,
    pub writable: bool,
    pub signer: bool,
}

#[derive(Debug, Clone)]
pub struct ToolboxIdlLookupError {
    pub code: u64,
    pub name: String,
    pub msg: String,
}

// TODO - add lookups for accounts
impl ToolboxIdl {
    pub fn lookup_instructions(
        &self
    ) -> Result<Vec<ToolboxIdlLookupInstruction>, ToolboxIdlError> {
        let mut instructions = vec![];
        for idl_instruction_name in self.instructions_accounts.keys() {
            instructions.push(self.lookup_instruction(idl_instruction_name)?);
        }
        Ok(instructions)
    }

    pub fn lookup_instruction(
        &self,
        instruction_name: &str,
    ) -> Result<ToolboxIdlLookupInstruction, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let idl_accounts_objects = idl_object_get_key_as_object_array_or_else(
            &self.instructions_accounts,
            instruction_name,
            &breadcrumbs.as_idl("instruction_accounts"),
        )?;
        let mut lookup_accounts = vec![];
        for index in 0..idl_accounts_objects.len() {
            let idl_account_object = idl_accounts_objects.get(index).unwrap();
            let idl_account_name = idl_object_get_key_as_str_or_else(
                idl_account_object,
                "name",
                &breadcrumbs
                    .as_idl(&format!("instruction_accounts[{}]", index)),
            )?;
            let idl_account_is_resolvable = idl_account_object
                .contains_key("address")
                || idl_account_object.contains_key("pda");
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
            lookup_accounts.push(ToolboxIdlLookupInstructionAccount {
                name: idl_account_name.to_string(),
                resolvable: idl_account_is_resolvable,
                writable: idl_account_is_writable,
                signer: idl_account_is_signer,
            });
        }
        Ok(ToolboxIdlLookupInstruction {
            name: instruction_name.to_string(),
            accounts: lookup_accounts,
        })
    }

    pub fn lookup_errors(
        &self
    ) -> Result<Vec<ToolboxIdlLookupError>, ToolboxIdlError> {
        let mut errors = vec![];
        for idl_error_name in self.errors.keys() {
            errors.push(self.lookup_error(idl_error_name)?);
        }
        Ok(errors)
    }

    pub fn lookup_error(
        &self,
        error_name: &str,
    ) -> Result<ToolboxIdlLookupError, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let idl_error_object = idl_object_get_key_as_object_or_else(
            &self.errors,
            error_name,
            &breadcrumbs.as_idl("errors"),
        )?;
        let idl_error_code = idl_object_get_key_as_u64_or_else(
            idl_error_object,
            "code",
            &breadcrumbs.as_idl(&format!("error[{}]", error_name)),
        )?;
        let idl_error_msg = idl_object_get_key_as_str_or_else(
            idl_error_object,
            "msg",
            &breadcrumbs.as_idl(&format!("error[{}]", error_name)),
        )?;
        Ok(ToolboxIdlLookupError {
            code: idl_error_code,
            name: error_name.to_string(),
            msg: idl_error_msg.to_string(),
        })
    }

    pub fn lookup_error_by_code(
        &self,
        error_code: u64,
    ) -> Result<ToolboxIdlLookupError, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        for (idl_error_name, idl_error) in self.errors.iter() {
            if let Some(idl_error_object) = idl_error.as_object() {
                if let Some(idl_error_code) = idl_error_object
                    .get("code")
                    .and_then(|idl_error_code| idl_error_code.as_u64())
                {
                    if idl_error_code == error_code {
                        return self.lookup_error(idl_error_name);
                    }
                }
            }
        }
        idl_err(
            "Could not find error",
            &breadcrumbs.as_idl(&format!("error({})", error_code)),
        )
    }
}

impl ToolboxIdlLookupInstruction {
    pub fn print(&self) {
        println!("----");
        println!("instruction: {}", self.name);
        for index in 0..self.accounts.len() {
            let account = self.accounts.get(index).unwrap();
            println!("- accounts: #{:03}: {}", index + 1, account.describe(),)
        }
    }
}

impl ToolboxIdlLookupInstructionAccount {
    pub fn describe(&self) -> String {
        format!(
            "({}{}) {}{}",
            if self.writable { "W" } else { "R" },
            if self.signer { "S" } else { "." },
            self.name,
            if self.resolvable { " [RESOLVABLE]" } else { "" }
        )
    }
}
