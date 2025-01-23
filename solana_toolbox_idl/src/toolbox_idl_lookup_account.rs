use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_describe_type_of_object;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_object_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_scoped_named_object_array_or_else;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlLookupAccount {
    pub name: String,
    pub discriminator: Vec<u8>,
    pub fields: Vec<ToolboxIdlLookupAccountField>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlLookupAccountField {
    pub name: String,
    pub description: String,
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
