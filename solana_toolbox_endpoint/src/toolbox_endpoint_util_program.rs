use solana_sdk::bpf_loader_upgradeable;
use solana_sdk::bpf_loader_upgradeable::close_any;
use solana_sdk::bpf_loader_upgradeable::create_buffer;
use solana_sdk::bpf_loader_upgradeable::deploy_with_max_program_len;
use solana_sdk::bpf_loader_upgradeable::set_buffer_authority;
use solana_sdk::bpf_loader_upgradeable::upgrade;
use solana_sdk::bpf_loader_upgradeable::write;
use solana_sdk::bpf_loader_upgradeable::UpgradeableLoaderState;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signature;
use solana_sdk::signer::Signer;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::ToolboxEndpointError;

impl ToolboxEndpoint {
    pub async fn get_program_bytecode_from_program_id(
        &mut self,
        program_id: &Pubkey,
    ) -> Result<Option<Vec<u8>>, ToolboxEndpointError> {
        let program_id_account = match self.get_account(program_id).await? {
            Some(account) => account,
            None => {
                return Ok(None);
            },
        };
        match program_id_account.owner {
            bpf_loader_upgradeable::ID => {
                let program_data =
                    ToolboxEndpoint::find_program_data_from_program_id(
                        program_id,
                    );
                let program_data_account = self
                    .get_account(&program_data)
                    .await?
                    .ok_or_else(|| {
                        ToolboxEndpointError::Custom(
                            "Could not fetch program data".to_string(),
                        )
                    })?;
                let program_data_bytecode_offset =
                    UpgradeableLoaderState::size_of_programdata_metadata();
                if program_data_account.data.len()
                    < program_data_bytecode_offset
                {
                    return Err(ToolboxEndpointError::Custom(
                        "Program data is malformed".to_string(),
                    ));
                }
                Ok(Some(
                    program_data_account.data[program_data_bytecode_offset..]
                        .to_vec(),
                ))
            },
            _ => Err(ToolboxEndpointError::Custom(
                "Unsupported program owner".to_string(),
            )),
        }
    }

    pub fn find_program_data_from_program_id(program_id: &Pubkey) -> Pubkey {
        Pubkey::find_program_address(
            &[program_id.as_ref()],
            &bpf_loader_upgradeable::ID,
        )
        .0
    }

    pub async fn process_program_buffer_new(
        &mut self,
        payer: &Keypair,
        program_bytecode: &[u8],
        program_authority: &Pubkey,
    ) -> Result<Pubkey, ToolboxEndpointError> {
        let program_buffer = Keypair::new();
        let program_buffer_authority = Keypair::new();
        let program_len = program_bytecode.len();
        let rent_space = UpgradeableLoaderState::size_of_buffer(program_len);
        let rent_minimum_lamports =
            self.get_sysvar_rent().await?.minimum_balance(rent_space);
        let instructions_create = create_buffer(
            &payer.pubkey(),
            &program_buffer.pubkey(),
            &program_buffer_authority.pubkey(),
            rent_minimum_lamports,
            program_len,
        )
        .unwrap();
        self.process_instructions_with_signers(
            &instructions_create,
            payer,
            &[&program_buffer],
        )
        .await?;
        let write_packing = 1216;
        let write_count = program_len.div_ceil(write_packing);
        for write_index in 0..write_count {
            let write_before = write_index * write_packing;
            let write_after = (write_before + write_packing).min(program_len);
            let instruction_write = write(
                &program_buffer.pubkey(),
                &program_buffer_authority.pubkey(),
                u32::try_from(write_before)
                    .map_err(ToolboxEndpointError::TryFromInt)?,
                program_bytecode[write_before..write_after].to_vec(),
            );
            self.process_instruction_with_signers(
                instruction_write,
                payer,
                &[&program_buffer_authority],
            )
            .await?;
        }
        let instruction_set_authority = set_buffer_authority(
            &program_buffer.pubkey(),
            &program_buffer_authority.pubkey(),
            program_authority,
        );
        self.process_instruction_with_signers(
            instruction_set_authority,
            payer,
            &[&program_buffer_authority],
        )
        .await?;
        Ok(program_buffer.pubkey())
    }

    pub async fn process_program_deploy(
        &mut self,
        payer: &Keypair,
        program_id: &Keypair,
        program_buffer: &Pubkey,
        program_authority: &Keypair,
        program_len: usize,
    ) -> Result<Signature, ToolboxEndpointError> {
        let rent_space = UpgradeableLoaderState::size_of_program();
        let rent_minimum_lamports =
            self.get_sysvar_rent().await?.minimum_balance(rent_space);
        let instruction_deploy = deploy_with_max_program_len(
            &payer.pubkey(),
            &program_id.pubkey(),
            program_buffer,
            &program_authority.pubkey(),
            rent_minimum_lamports,
            program_len,
        )
        .unwrap();
        self.process_instructions_with_signers(
            &instruction_deploy,
            payer,
            &[&program_id, &program_authority],
        )
        .await
    }

    pub async fn process_program_upgrade(
        &mut self,
        payer: &Keypair,
        program_id: &Pubkey,
        program_buffer: &Pubkey,
        program_authority: &Keypair,
        spill: &Pubkey,
    ) -> Result<Signature, ToolboxEndpointError> {
        let instruction_upgrade = upgrade(
            program_id,
            program_buffer,
            &program_authority.pubkey(),
            spill,
        );
        self.process_instruction_with_signers(
            instruction_upgrade,
            payer,
            &[&program_authority],
        )
        .await
    }

    pub async fn process_program_buffer_close(
        &mut self,
        payer: &Keypair,
        program_buffer: &Pubkey,
        program_authority: &Keypair,
        spill: &Pubkey,
    ) -> Result<Signature, ToolboxEndpointError> {
        let program_authority_address = &program_authority.pubkey();
        let instruction_close = close_any(
            program_buffer,
            spill,
            Some(program_authority_address),
            None,
        );
        self.process_instruction_with_signers(
            instruction_close,
            payer,
            &[&program_authority],
        )
        .await
    }
}
