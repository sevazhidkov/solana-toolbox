use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Context;
use anyhow::Result;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_idl_program::ToolboxIdlProgram;

pub struct ToolboxIdlService {
    cached_programs: HashMap<Pubkey, Option<Arc<ToolboxIdlProgram>>>,
}

impl Default for ToolboxIdlService {
    fn default() -> ToolboxIdlService {
        ToolboxIdlService::new()
    }
}

impl ToolboxIdlService {
    pub fn new() -> ToolboxIdlService {
        ToolboxIdlService {
            cached_programs: Default::default(),
        }
    }

    pub fn set_program(
        &mut self,
        program_id: &Pubkey,
        idl_program: Option<Arc<ToolboxIdlProgram>>,
    ) {
        self.cached_programs.insert(*program_id, idl_program);
    }

    pub async fn get_or_resolve_program(
        &mut self,
        endpoint: &mut ToolboxEndpoint,
        program_id: &Pubkey,
    ) -> Result<Option<Arc<ToolboxIdlProgram>>> {
        if let Some(idl_program) = self.cached_programs.get(program_id) {
            return Ok(idl_program.clone());
        }
        let idl_program =
            ToolboxIdlService::resolve_program(endpoint, program_id).await?;
        self.cached_programs
            .insert(*program_id, idl_program.clone());
        Ok(idl_program)
    }

    async fn resolve_program(
        endpoint: &mut ToolboxEndpoint,
        program_id: &Pubkey,
    ) -> Result<Option<Arc<ToolboxIdlProgram>>> {
        if let Some(idl_program) = ToolboxIdlProgram::from_lib(program_id) {
            return Ok(Some(Arc::new(idl_program)));
        }
        Ok(endpoint
            .get_account(
                &ToolboxIdlProgram::find_anchor_address(program_id)
                    .context("Find Anchor Idl Account")?,
            )
            .await
            .context("Get Anchor Idl Account")?
            .map(|source_account| {
                ToolboxIdlProgram::try_parse_from_account_data(
                    &source_account.data,
                )
            })
            .transpose()
            .context("Parse IDL Account Data")?
            .map(Arc::new))
    }
}
