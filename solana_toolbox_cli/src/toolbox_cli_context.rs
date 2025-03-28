use std::collections::HashMap;
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
                    "Invalid idl, expected format: [ProgramId:IdlFile]"
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
        if let Some((name, key)) = account.split_once(":") {
            Ok((name.to_string(), self.parse_key(key)?))
        } else {
            Err(ToolboxCliError::Custom(
                "Invalid account, expected format: [Name:[Pubkey|KeypairFile|'KEYPAIR']]".to_string(),
            ))
        }
    }

    pub fn parse_key(
        &self,
        key: &str,
    ) -> Result<ToolboxCliKey, ToolboxCliError> {
        if key.to_ascii_uppercase() == "KEYPAIR"
            || key.to_ascii_uppercase() == "WALLET"
        {
            return Ok(ToolboxCliKey::Keypair(self.get_keypair()));
        }
        Ok(if let Ok(keypair) = read_keypair_file(key) {
            ToolboxCliKey::Keypair(keypair)
        } else {
            ToolboxCliKey::Pubkey(Pubkey::from_str(
                key.trim_matches(|c| !char::is_alphanumeric(c)),
            )?)
        })
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
        self.compute_explorer_link(
            "address",
            &address.to_string(),
            &HashMap::new(),
        )
    }

    pub fn compute_explorer_signature_link(
        &self,
        signature: &Signature,
    ) -> String {
        self.compute_explorer_link(
            "tx",
            &signature.to_string(),
            &HashMap::new(),
        )
    }

    pub fn compute_explorer_simulation_link(
        &self,
        transaction_signatures: &[Signature],
        transaction_message_serialized: &[u8],
    ) -> Result<String, ToolboxCliError> {
        let mut params = HashMap::new();
        params.insert(
            "signatures".to_string(),
            format!(
                "[{}]",
                transaction_signatures
                    .iter()
                    .map(|signature| format!("\"{}\"", signature.to_string()))
                    .collect::<Vec<_>>()
                    .join(","),
            ),
        );
        params.insert(
            "message".to_string(),
            ToolboxEndpoint::encode_base64(transaction_message_serialized)?,
        );
        Ok(self.compute_explorer_link("tx", "inspector", &params))
    }

    fn compute_explorer_link(
        &self,
        category: &str,
        payload: &str,
        params: &HashMap<String, String>,
    ) -> String {
        let mut args = vec![];
        for (param_name, param_content) in params {
            args.push(format!(
                "{}={}",
                urlencoding::encode(param_name),
                urlencoding::encode(param_content)
            ));
        }
        args.push("cluster=custom".to_string());
        args.push(format!(
            "customUrl={}",
            urlencoding::encode(&self.json_rpc_url)
        ));
        format!(
            "https://explorer.solana.com/{}/{}?{}",
            category,
            payload,
            args.join("&"),
        )
    }
}
