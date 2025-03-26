use std::sync::Arc;

use serde_json::Value;
use solana_sdk::account::Account;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_idl_account::ToolboxIdlAccount;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program::ToolboxIdlProgram;
use crate::toolbox_idl_service::ToolboxIdlService;

pub struct ToolboxIdlServiceAccountDecoded {
    pub program: Arc<ToolboxIdlProgram>,
    pub account: Arc<ToolboxIdlAccount>,
    pub state: Value,
}

impl ToolboxIdlService {
    pub async fn get_and_decode_account(
        &mut self,
        endpoint: &mut ToolboxEndpoint,
        address: &Pubkey,
    ) -> Result<ToolboxIdlServiceAccountDecoded, ToolboxIdlError> {
        let account = endpoint.get_account(address).await?.unwrap_or_default();
        self.decode_account(endpoint, &account).await
    }

    pub async fn decode_account(
        &mut self,
        endpoint: &mut ToolboxEndpoint,
        account: &Account,
    ) -> Result<ToolboxIdlServiceAccountDecoded, ToolboxIdlError> {
        let idl_program = self
            .resolve_program(endpoint, &account.owner)
            .await?
            .unwrap_or_default();
        let idl_account =
            idl_program.guess_account(&account.data).unwrap_or_default();
        let account_state = idl_account.decompile(&account.data)?;
        Ok(ToolboxIdlServiceAccountDecoded {
            program: idl_program,
            account: idl_account,
            state: account_state,
        })
    }
}
