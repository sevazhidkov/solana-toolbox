use std::fs::read_to_string;
use std::str::FromStr;

use serde_json::Value;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signature;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_idl::ToolboxIdlAccount;
use solana_toolbox_idl::ToolboxIdlInstruction;
use solana_toolbox_idl::ToolboxIdlProgram;
use solana_toolbox_idl::ToolboxIdlService;

use crate::toolbox_cli_error::ToolboxCliError;
use crate::toolbox_cli_key::ToolboxCliKey;

pub struct ToolboxCliContext {
    json_rpc_url: String,
    commitment: String,
    keypair_path: String,
    custom_idls: Vec<String>,
}

impl ToolboxCliContext {
    pub fn new(
        json_rpc_url: String,
        commitment: String,
        keypair_path: String,
        custom_idls: Vec<String>,
    ) -> ToolboxCliContext {
        ToolboxCliContext {
            json_rpc_url,
            commitment,
            keypair_path,
            custom_idls,
        }
    }

    pub async fn create_endpoint(
        &self,
    ) -> Result<ToolboxEndpoint, ToolboxCliError> {
        Ok(ToolboxEndpoint::new_rpc_with_url_or_moniker_and_commitment(
            &self.json_rpc_url,
            CommitmentConfig::from_str(&self.commitment)?,
        ))
    }

    pub async fn create_service(
        &self,
    ) -> Result<ToolboxIdlService, ToolboxCliError> {
        let mut idl_service = ToolboxIdlService::new();
        for custom_idl in &self.custom_idls {
            // TODO - allow "=" delimiter somehow ??
            if let Some((program_id, idl_file)) = custom_idl.split_once(":") {
                idl_service.preload_program(
                    &self.parse_key(program_id)?.address(),
                    Some(
                        ToolboxIdlProgram::try_parse_from_str(
                            &read_to_string(idl_file)?,
                        )?
                        .into(),
                    ),
                );
            } else {
                return Err(ToolboxCliError::Custom(
                    "Invalid idl, expected format: [ProgramId:IdlFilePath]"
                        .to_string(),
                ));
            }
        }
        Ok(idl_service)
    }

    pub fn parse_account(
        &self,
        account: &str,
    ) -> Result<(String, ToolboxCliKey), ToolboxCliError> {
        // TODO - figure out a better way for delimited split ?
        if let Some((name, key)) = account.split_once(":") {
            Ok((name.to_string(), self.parse_key(key)?))
        } else {
            // TODO - use custom clap parser instead
            Err(ToolboxCliError::Custom(
                "Invalid account, expected format: [Name:[Pubkey|KeypairFilePath]]".to_string(),
            ))
        }
    }

    pub fn parse_key(
        &self,
        key: &str,
    ) -> Result<ToolboxCliKey, ToolboxCliError> {
        if key.to_ascii_lowercase() == "keypair"
            || key.to_ascii_lowercase() == "wallet"
        {
            return Ok(ToolboxCliKey::Keypair(self.get_keypair()));
        }
        ToolboxCliKey::try_parse(key)
    }

    pub fn parse_signature(
        &self,
        value: &str,
    ) -> Result<Signature, ToolboxCliError> {
        Ok(Signature::from_str(value)?)
    }

    pub fn parse_hjson(&self, value: &str) -> Result<Value, ToolboxCliError> {
        Ok(serde_hjson::from_str::<Value>(value)?)
    }

    pub fn get_keypair(&self) -> Keypair {
        read_keypair_file(self.keypair_path.clone()).unwrap_or(Keypair::new())
    }

    pub fn compute_account_kind(
        &self,
        program_id: &Pubkey,
        program: &ToolboxIdlProgram,
        account: &ToolboxIdlAccount,
    ) -> String {
        if let Some(program_name) = &program.metadata.name {
            return format!("{}.{}.{}", program_id, program_name, account.name);
        }
        return format!("{}.{}", program_id, account.name);
    }

    pub fn compute_instruction_kind(
        &self,
        program_id: &Pubkey,
        program: &ToolboxIdlProgram,
        instruction: &ToolboxIdlInstruction,
    ) -> String {
        if let Some(program_name) = &program.metadata.name {
            return format!(
                "{}.{}.{}",
                program_id, program_name, instruction.name
            );
        }
        return format!("{}.{}", program_id, instruction.name);
    }

    pub fn compute_explorer_address_link(&self, address: &Pubkey) -> String {
        ToolboxEndpoint::compute_explorer_address_link(
            &self.json_rpc_url,
            address,
        )
    }

    pub fn compute_explorer_signature_link(
        &self,
        signature: &Signature,
    ) -> String {
        ToolboxEndpoint::compute_explorer_signature_link(
            &self.json_rpc_url,
            signature,
        )
    }

    pub fn compute_explorer_simulation_link(
        &self,
        transaction_signatures: &[Signature],
        transaction_message_serialized: &[u8],
    ) -> String {
        ToolboxEndpoint::compute_explorer_simulation_link(
            &self.json_rpc_url,
            transaction_signatures,
            transaction_message_serialized,
        )
    }
}
