use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;

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

    pub fn preload_program(
        &mut self,
        program_id: &Pubkey,
        idl_program: Option<Arc<ToolboxIdlProgram>>,
    ) {
        self.cached_programs.insert(*program_id, idl_program);
    }

    pub async fn resolve_program(
        &mut self,
        endpoint: &mut ToolboxEndpoint,
        program_id: &Pubkey,
    ) -> Result<Option<Arc<ToolboxIdlProgram>>> {
        if let Some(idl_program) = self.cached_programs.get(program_id) {
            return Ok(idl_program.clone());
        }
        let idl_program = {
            if let Some(idl_program) = ToolboxIdlProgram::from_lib(program_id) {
                Some(Arc::new(idl_program))
            } else {
                let mut source_account = None;
                if let Some(anchor_account) = endpoint
                    .get_account(&ToolboxIdlProgram::find_anchor(program_id)?)
                    .await?
                {
                    source_account = Some(anchor_account);
                } else if let Some(shank_account) = endpoint
                    .get_account(&ToolboxIdlProgram::find_shank(program_id)?)
                    .await?
                {
                    source_account = Some(shank_account);
                }
                source_account
                    .map(|source_account| {
                        ToolboxIdlProgram::try_parse_from_account_data(
                            &source_account.data,
                        )
                    })
                    .transpose()?
                    .map(Arc::new)
            }
        };
        self.cached_programs
            .insert(*program_id, idl_program.clone());
        Ok(idl_program)
    }
}
