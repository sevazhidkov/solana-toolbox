use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_bool;
use crate::toolbox_idl_utils::idl_object_get_key_as_object_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_scoped_named_object_array_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_u64_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;
use crate::toolbox_idl_utils::idl_value_as_str_or_object_with_name_as_str_or_else;

#[derive(Debug, Clone)]
pub struct ToolboxIdlLookupInstruction {
    pub name: String,
    pub discriminator: Vec<u8>,
    pub accounts: Vec<ToolboxIdlLookupInstructionAccount>,
    pub args: Vec<ToolboxIdlLookupInstructionArg>,
}

#[derive(Debug, Clone)]
pub struct ToolboxIdlLookupInstructionAccount {
    pub name: String,
    pub resolvable: bool,
    pub writable: bool,
    pub signer: bool,
}

#[derive(Debug, Clone)]
pub struct ToolboxIdlLookupInstructionArg {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ToolboxIdlLookupAccount {
    pub name: String,
    pub discriminator: Vec<u8>,
    pub fields: Vec<ToolboxIdlLookupAccountField>,
}

#[derive(Debug, Clone)]
pub struct ToolboxIdlLookupAccountField {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ToolboxIdlLookupType {
    pub name: String,
    pub kind: String,
    pub items: Vec<ToolboxIdlLookupTypeItem>,
}

#[derive(Debug, Clone)]
pub struct ToolboxIdlLookupTypeItem {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ToolboxIdlLookupError {
    pub code: u64,
    pub name: String,
    pub msg: String,
}

impl ToolboxIdl {
    pub fn lookup_accounts(
        &self
    ) -> Result<Vec<ToolboxIdlLookupAccount>, ToolboxIdlError> {
        let mut accounts = vec![];
        for idl_account_name in self.accounts_types.keys() {
            accounts.push(self.lookup_account(idl_account_name)?);
        }
        Ok(accounts)
    }

    pub fn lookup_account(
        &self,
        account_name: &str,
    ) -> Result<ToolboxIdlLookupAccount, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let account_discriminator = idl_map_get_key_or_else(
            &self.accounts_discriminators,
            account_name,
            &breadcrumbs.as_idl("accounts_discriminators"),
        )?;
        let idl_account_type_object = idl_object_get_key_as_object_or_else(
            &self.accounts_types,
            account_name,
            &breadcrumbs.as_idl("accounts_types"),
        )?;
        let mut account_fields = vec![];
        for (idl_field_object, idl_field_name, breadcrumbs) in
            idl_object_get_key_as_scoped_named_object_array_or_else(
                idl_account_type_object,
                "fields",
                &breadcrumbs.with_idl(account_name),
            )?
        {
            account_fields.push(ToolboxIdlLookupAccountField {
                name: idl_field_name.to_string(),
                description: idl_describe_type_of_object(
                    idl_field_object,
                    &breadcrumbs,
                )?,
            });
        }
        Ok(ToolboxIdlLookupAccount {
            name: account_name.to_string(),
            discriminator: account_discriminator.clone(),
            fields: account_fields,
        })
    }

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
        let instruction_discriminator = idl_map_get_key_or_else(
            &self.instructions_discriminators,
            instruction_name,
            &breadcrumbs.as_idl("instructions_discriminators"),
        )?;
        let mut instruction_accounts = vec![];
        for (idl_account_object, idl_account_name, _) in
            idl_object_get_key_as_scoped_named_object_array_or_else(
                &self.instructions_accounts,
                instruction_name,
                &breadcrumbs.with_idl("instruction_accounts"),
            )?
        {
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
            instruction_accounts.push(ToolboxIdlLookupInstructionAccount {
                name: idl_account_name.to_string(),
                resolvable: idl_account_is_resolvable,
                writable: idl_account_is_writable,
                signer: idl_account_is_signer,
            });
        }
        let mut instruction_args = vec![];
        for (idl_arg_object, idl_arg_name, breadcrumbs) in
            idl_object_get_key_as_scoped_named_object_array_or_else(
                &self.instructions_args,
                instruction_name,
                &breadcrumbs.with_idl("instruction_args"),
            )?
        {
            instruction_args.push(ToolboxIdlLookupInstructionArg {
                name: idl_arg_name.to_string(),
                description: idl_describe_type_of_object(
                    idl_arg_object,
                    &breadcrumbs,
                )?,
            });
        }
        Ok(ToolboxIdlLookupInstruction {
            name: instruction_name.to_string(),
            discriminator: instruction_discriminator.clone(),
            accounts: instruction_accounts,
            args: instruction_args,
        })
    }

    pub fn lookup_types(
        &self
    ) -> Result<Vec<ToolboxIdlLookupType>, ToolboxIdlError> {
        let mut types = vec![];
        for idl_type_name in self.types.keys() {
            types.push(self.lookup_type(idl_type_name)?);
        }
        Ok(types)
    }

    pub fn lookup_type(
        &self,
        type_name: &str,
    ) -> Result<ToolboxIdlLookupType, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let idl_type_object = idl_object_get_key_as_object_or_else(
            &self.types,
            type_name,
            &breadcrumbs.as_idl("types"),
        )?;
        if let Some(idl_type_kind) =
            idl_object_get_key_as_str(idl_type_object, "kind")
        {
            if idl_type_kind == "struct" {
                let mut type_fields = vec![];
                for (idl_field_object, idl_field_name, breadcrumbs) in
                    idl_object_get_key_as_scoped_named_object_array_or_else(
                        idl_type_object,
                        "fields",
                        &breadcrumbs.with_idl(type_name),
                    )?
                {
                    type_fields.push(ToolboxIdlLookupTypeItem {
                        name: idl_field_name.to_string(),
                        description: idl_describe_type_of_object(
                            idl_field_object,
                            &breadcrumbs,
                        )?,
                    });
                }
                return Ok(ToolboxIdlLookupType {
                    name: type_name.to_string(),
                    kind: "struct".to_string(),
                    items: type_fields,
                });
            }
            if idl_type_kind == "enum" {
                let mut type_variants = vec![];
                for (index, (_, idl_variant_name, _)) in
                    idl_object_get_key_as_scoped_named_object_array_or_else(
                        idl_type_object,
                        "variants",
                        &breadcrumbs.with_idl(type_name),
                    )?
                    .into_iter()
                    .enumerate()
                {
                    type_variants.push(ToolboxIdlLookupTypeItem {
                        name: index.to_string(),
                        description: idl_variant_name.to_string(),
                    });
                }
                return Ok(ToolboxIdlLookupType {
                    name: type_name.to_string(),
                    kind: "enum".to_string(),
                    items: type_variants,
                });
            }
        }
        Ok(ToolboxIdlLookupType {
            name: type_name.to_string(),
            kind: "unparsable".to_string(),
            items: vec![],
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
        println!("instruction.name: {:?}", self.name);
        println!("instruction.discriminator: {:?}", self.discriminator);
        for index in 0..self.accounts.len() {
            let account = self.accounts.get(index).unwrap();
            println!(
                "instruction.accounts: #{:03}: {}",
                index + 1,
                account.describe()
            );
        }
        for arg in &self.args {
            println!("instruction.arg: {}: {}", arg.name, arg.description);
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

impl ToolboxIdlLookupAccount {
    pub fn print(&self) {
        println!("----");
        println!("account.name: {:?}", self.name);
        println!("account.discriminator: {:?}", self.discriminator);
        for field in &self.fields {
            println!("account.data: {}: {}", field.name, field.description);
        }
    }
}

impl ToolboxIdlLookupType {
    pub fn print(&self) {
        println!("----");
        println!("{}.name: {:?}", self.kind, self.name);
        for item in &self.items {
            println!("{}.item: {}: {}", self.kind, item.name, item.description);
        }
    }
}

impl ToolboxIdlLookupError {
    pub fn print(&self) {
        println!("----");
        println!("error.code: {:?}", self.code);
        println!("error.name: {:?}", self.name);
        println!("error.msg: {:?}", self.msg);
    }
}
fn idl_describe_type_of_object(
    idl_object: &Map<String, Value>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<String, ToolboxIdlError> {
    let idl_type =
        idl_object_get_key_or_else(idl_object, "type", &breadcrumbs.idl())?;
    idl_describe_type(idl_type, breadcrumbs)
}

fn idl_describe_type(
    idl_type: &Value,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<String, ToolboxIdlError> {
    if let Some(idl_type_object) = idl_type.as_object() {
        if let Some(idl_type_defined) = idl_type_object.get("defined") {
            return Ok(idl_value_as_str_or_object_with_name_as_str_or_else(
                idl_type_defined,
                &breadcrumbs.as_idl("defined"),
            )?
            .to_string());
        }
        if let Some(idl_type_option) = idl_type_object.get("option") {
            return Ok(format!(
                "Option<{}>",
                idl_describe_type(
                    idl_type_option,
                    &breadcrumbs.with_idl("Option"),
                )?
            ));
        }
        if let Some(idl_type_kind) =
            idl_object_get_key_as_str(idl_type_object, "kind")
        {
            if idl_type_kind == "struct" {
                return Ok("Struct".to_string());
            }
            if idl_type_kind == "enum" {
                return Ok("Enum".to_string());
            }
        }
        if let Some(idl_type_array) =
            idl_object_get_key_as_array(idl_type_object, "array")
        {
            if idl_type_array.len() != 2 {
                return Ok("unparsable array".to_string());
            }
            return Ok(format!(
                "[{}; {}]",
                idl_describe_type(
                    &idl_type_array[0],
                    &breadcrumbs.with_idl("Array")
                )?,
                idl_type_array[1]
            ));
        }
        if let Some(idl_type_vec) = idl_type_object.get("vec") {
            return Ok(format!(
                "Vec<{}>",
                idl_describe_type(idl_type_vec, &breadcrumbs.with_idl("Vec"))?
            ));
        }
    }
    if let Some(idl_type_leaf) = idl_type.as_str() {
        return Ok(idl_type_leaf.to_string());
    }
    Ok("unparsable type".to_string())
}
