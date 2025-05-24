use std::sync::Arc;

use anyhow::Context;
use anyhow::Result;
use serde_json::Value;
use solana_sdk::account::Account;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_idl_account::ToolboxIdlAccount;
use crate::toolbox_idl_program::ToolboxIdlProgram;
use crate::toolbox_idl_service::ToolboxIdlService;

pub struct ToolboxIdlServiceAccountDecoded {
    pub lamports: u64,
    pub owner: Pubkey,
    pub program: Arc<ToolboxIdlProgram>,
    pub account: Arc<ToolboxIdlAccount>,
    pub state: Value,
}

impl ToolboxIdlService {
    pub async fn get_and_decode_account(
        &mut self,
        endpoint: &mut ToolboxEndpoint,
        address: &Pubkey,
    ) -> Result<ToolboxIdlServiceAccountDecoded> {
        let account = endpoint
            .get_account(address)
            .await
            .context("Get Account")?
            .unwrap_or_default();
        self.decode_account(endpoint, &account).await
    }

    pub async fn decode_account(
        &mut self,
        endpoint: &mut ToolboxEndpoint,
        account: &Account,
    ) -> Result<ToolboxIdlServiceAccountDecoded> {
        let idl_program = self
            .load_program(endpoint, &account.owner)
            .await
            .context("Resolve Program")?
            .unwrap_or_default();
        let idl_account =
            idl_program.guess_account(&account.data).unwrap_or_default();
        let account_state = idl_account
            .decode(&account.data)
            .context("Decode Account State")?;
        Ok(ToolboxIdlServiceAccountDecoded {
            lamports: account.lamports,
            owner: account.owner,
            program: idl_program,
            account: idl_account,
            state: account_state,
        })
    }
}
