use std::str::FromStr;

use anyhow::Result;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::TransactionError;

use crate::toolbox_endpoint::ToolboxEndpoint;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxEndpointExecution {
    pub slot: u64,
    pub payer: Pubkey,
    pub instructions: Vec<Instruction>,
    pub steps: Option<Vec<ToolboxEndpointExecutionStep>>,
    pub logs: Option<Vec<String>>,
    pub error: Option<TransactionError>,
    pub return_data: Option<Vec<u8>>,
    pub units_consumed: Option<u64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxEndpointExecutionStep {
    Unknown(String),
    Log(String),
    Data(Vec<u8>),
    Call(ToolboxEndpointExecutionStepCall),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxEndpointExecutionStepCall {
    pub program_id: Pubkey,
    pub steps: Vec<ToolboxEndpointExecutionStep>,
    pub consumed: Option<(u64, u64)>,
    pub returns: Option<Vec<u8>>,
    pub failure: Option<String>,
}

// RETURN: mainnet: 3TZeRWjoJ3W2tqTFB4QZRuwAEKCoYryCD2CLtwPXH4EdkaQHbpor7ndJ3FD9KYzb9ff66eKVRB1LeN4a9UzQVYRC
// FAILURE: devnet: 2BcPxAAz6myMLKUMbKgV1dfdzGgPTqFh6imVb3oX3M4gsQBEbhs6P5W466TaRViLcbACSaN7R5hLboBUeKXh9uUY

impl ToolboxEndpointExecution {
    pub fn try_parse_steps(
        logs: &[String],
    ) -> Result<Vec<ToolboxEndpointExecutionStep>> {
        let (_offset, root_step) =
            ToolboxEndpointExecution::try_parse_step_call(
                Pubkey::default(),
                0,
                logs,
            )?;
        // TODO - check warning for returns/failure/consumed/offset weirdness?
        Ok(root_step.steps)
    }

    fn try_parse_step_call(
        program_id: Pubkey,
        offset: usize,
        logs: &[String],
    ) -> Result<(usize, ToolboxEndpointExecutionStepCall)> {
        let mut offset = offset;
        let mut steps = vec![];
        let mut consumed = None;
        let mut returns = None;
        let mut failure = None;
        while offset < logs.len() {
            let log = &logs[offset];
            offset += 1;
            if let Some(log_message) = log.strip_prefix("Program log: ") {
                steps.push(ToolboxEndpointExecutionStep::Log(
                    log_message.to_string(),
                ));
                continue;
            }
            if let Some(log_base64) = log.strip_prefix("Program data: ") {
                steps.push(ToolboxEndpointExecutionStep::Data(
                    ToolboxEndpoint::sanitize_and_decode_base64(log_base64)?,
                ));
                continue;
            }
            if let Some(log_return) = log.strip_prefix("Program return: ") {
                if let Some((_, log_return_base64)) = log_return.split_once(" ")
                {
                    returns =
                        Some(ToolboxEndpoint::sanitize_and_decode_base64(
                            log_return_base64,
                        )?);
                    continue;
                }
            }
            if let Some(log_stack) = log.strip_prefix("Program ") {
                if let Some((log_program_id, log_info)) =
                    log_stack.split_once(" ")
                {
                    let program_id = Pubkey::from_str(log_program_id)?;
                    if let Some(_log_invoke_level) =
                        log_info.strip_prefix("invoke ")
                    {
                        let (new_offset, call) =
                            ToolboxEndpointExecution::try_parse_step_call(
                                program_id, offset, logs,
                            )?;
                        offset = new_offset;
                        steps.push(ToolboxEndpointExecutionStep::Call(call));
                        continue;
                    }
                    if let Some(log_consumed) =
                        log_info.strip_prefix("consumed ")
                    {
                        let log_consumed_parts =
                            log_consumed.split(" ").collect::<Vec<_>>();
                        if log_consumed_parts.len() == 5
                            && log_consumed_parts[1] == "of"
                            && log_consumed_parts[3] == "compute"
                            && log_consumed_parts[4] == "units"
                        {
                            consumed = Some((
                                log_consumed_parts[0].parse::<u64>()?,
                                log_consumed_parts[2].parse::<u64>()?,
                            ));
                        }
                        continue;
                    }
                    if let Some(log_failed_error) =
                        log_info.strip_prefix("failed: ")
                    {
                        failure = Some(log_failed_error.to_string());
                        break;
                    }
                    if let Some(_log_success_postfix) =
                        log_info.strip_prefix("success")
                    {
                        break;
                    }
                }
            }
            steps.push(ToolboxEndpointExecutionStep::Unknown(log.to_string()));
        }
        Ok((
            offset,
            ToolboxEndpointExecutionStepCall {
                program_id,
                steps,
                returns,
                failure,
                consumed,
            },
        ))
    }
}
