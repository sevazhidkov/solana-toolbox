use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_describe_type;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_bool;
use crate::toolbox_idl_utils::idl_object_get_key_as_scoped_named_content_array_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_scoped_named_object_array_or_else;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlLookupInstruction {
    pub name: String,
    pub discriminator: Vec<u8>,
    pub accounts: Vec<ToolboxIdlLookupInstructionAccount>,
    pub args: Vec<ToolboxIdlLookupInstructionArg>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlLookupInstructionAccount {
    pub name: String,
    pub resolvable: bool,
    pub writable: bool,
    pub signer: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlLookupInstructionArg {
    pub name: String,
    pub description: String,
}

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
        let instruction_discriminator = idl_map_get_key_or_else(
            &self.instructions_discriminators,
            instruction_name,
            &breadcrumbs.as_idl("instructions_discriminators"),
        )?;
        let mut instruction_accounts = vec![];
        for (idl_account_name, idl_account_object, _) in
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
        for (idl_arg_name, idl_arg_type, breadcrumbs) in
            idl_object_get_key_as_scoped_named_content_array_or_else(
                &self.instructions_args,
                instruction_name,
                "type",
                &breadcrumbs.with_idl("instruction_args"),
            )?
        {
            instruction_args.push(ToolboxIdlLookupInstructionArg {
                name: idl_arg_name.to_string(),
                description: idl_describe_type(idl_arg_type, &breadcrumbs)?,
            });
        }
        Ok(ToolboxIdlLookupInstruction {
            name: instruction_name.to_string(),
            discriminator: instruction_discriminator.clone(),
            accounts: instruction_accounts,
            args: instruction_args,
        })
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
