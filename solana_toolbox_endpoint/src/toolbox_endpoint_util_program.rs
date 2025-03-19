use solana_sdk::account::Account;
use solana_sdk::bpf_loader_upgradeable;
use solana_sdk::bpf_loader_upgradeable::close_any;
use solana_sdk::bpf_loader_upgradeable::create_buffer;
use solana_sdk::bpf_loader_upgradeable::deploy_with_max_program_len;
use solana_sdk::bpf_loader_upgradeable::extend_program;
use solana_sdk::bpf_loader_upgradeable::set_buffer_authority;
use solana_sdk::bpf_loader_upgradeable::upgrade;
use solana_sdk::bpf_loader_upgradeable::write;
use solana_sdk::bpf_loader_upgradeable::UpgradeableLoaderState;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_error::ToolboxEndpointError;

impl ToolboxEndpoint {
    pub const BPF_LOADER_UPGRADEABLE_PROGRAM_ID: Pubkey =
        bpf_loader_upgradeable::ID;

    pub async fn get_program_meta(
        &mut self,
        program_id: &Pubkey,
    ) -> Result<Option<(u64, Option<Pubkey>)>, ToolboxEndpointError> {
        self.get_program_data_account(program_id)
            .await?
            .map(|program_data| {
                match bincode::deserialize::<UpgradeableLoaderState>(
                    &program_data.data,
                )
                .map_err(ToolboxEndpointError::Bincode)?
                {
                    UpgradeableLoaderState::ProgramData {
                        slot,
                        upgrade_authority_address,
                    } => Ok((slot, upgrade_authority_address)),
                    _ => Err(ToolboxEndpointError::Custom(
                        "Program data is malformed".to_string(),
                    )),
                }
            })
            .transpose()
    }

    pub async fn get_program_bytecode(
        &mut self,
        program_id: &Pubkey,
    ) -> Result<Option<Vec<u8>>, ToolboxEndpointError> {
        self.get_program_data_account(program_id)
            .await?
            .map(|program_data| {
                let program_data_bytecode_offset =
                    UpgradeableLoaderState::size_of_programdata_metadata();
                if program_data.data.len() < program_data_bytecode_offset {
                    return Err(ToolboxEndpointError::Custom(
                        "Program data is too small".to_string(),
                    ));
                }
                Ok(program_data.data[program_data_bytecode_offset..].to_vec())
            })
            .transpose()
    }

    async fn get_program_data_account(
        &mut self,
        program_id: &Pubkey,
    ) -> Result<Option<Account>, ToolboxEndpointError> {
        let program_id_account = match self.get_account(program_id).await? {
            Some(account) => account,
            None => {
                return Ok(None);
            },
        };
        if !program_id_account.executable {
            return Err(ToolboxEndpointError::Custom(
                "Program Id is not executable".to_string(),
            ));
        }
        if program_id_account.owner != bpf_loader_upgradeable::ID {
            return Err(ToolboxEndpointError::Custom(
                "Unsupported program owner".to_string(),
            ));
        }
        self.get_account(&ToolboxEndpoint::find_program_data_from_program_id(
            program_id,
        ))
        .await
    }

    fn find_program_data_from_program_id(program_id: &Pubkey) -> Pubkey {
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
        let program_bytecode_len = program_bytecode.len();
        let rent_space =
            UpgradeableLoaderState::size_of_buffer(program_bytecode_len);
        let rent_minimum_lamports =
            self.get_sysvar_rent().await?.minimum_balance(rent_space);
        let instructions_create = create_buffer(
            &payer.pubkey(),
            &program_buffer.pubkey(),
            &program_buffer_authority.pubkey(),
            rent_minimum_lamports,
            program_bytecode_len,
        )
        .unwrap();
        self.process_instructions_with_signers(
            payer,
            &instructions_create,
            &[&program_buffer],
        )
        .await?;
        let write_packing = 1216;
        let write_count = program_bytecode_len.div_ceil(write_packing);
        for write_index in 0..write_count {
            let write_before = write_index * write_packing;
            let write_after =
                (write_before + write_packing).min(program_bytecode_len);
            let instruction_write = write(
                &program_buffer.pubkey(),
                &program_buffer_authority.pubkey(),
                u32::try_from(write_before)
                    .map_err(ToolboxEndpointError::TryFromInt)?,
                program_bytecode[write_before..write_after].to_vec(),
            );
            self.process_instruction_with_signers(
                payer,
                instruction_write,
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
            payer,
            instruction_set_authority,
            &[&program_buffer_authority],
        )
        .await?;
        Ok(program_buffer.pubkey())
    }

    pub async fn process_program_buffer_deploy(
        &mut self,
        payer: &Keypair,
        program_id: &Keypair,
        program_buffer: &Pubkey,
        program_authority: &Keypair,
        program_bytecode_len: usize,
    ) -> Result<(), ToolboxEndpointError> {
        let rent_space = UpgradeableLoaderState::size_of_program();
        let rent_minimum_lamports =
            self.get_sysvar_rent().await?.minimum_balance(rent_space);
        let instruction_deploy = deploy_with_max_program_len(
            &payer.pubkey(),
            &program_id.pubkey(),
            program_buffer,
            &program_authority.pubkey(),
            rent_minimum_lamports,
            program_bytecode_len,
        )
        .unwrap();
        self.process_instructions_with_signers(
            payer,
            &instruction_deploy,
            &[program_id, program_authority],
        )
        .await?;
        Ok(())
    }

    pub async fn process_program_buffer_upgrade(
        &mut self,
        payer: &Keypair,
        program_id: &Pubkey,
        program_buffer: &Pubkey,
        program_authority: &Keypair,
        spill: &Pubkey,
    ) -> Result<(), ToolboxEndpointError> {
        let instruction_upgrade = upgrade(
            program_id,
            program_buffer,
            &program_authority.pubkey(),
            spill,
        );
        self.process_instruction_with_signers(
            payer,
            instruction_upgrade,
            &[program_authority],
        )
        .await?;
        Ok(())
    }

    pub async fn process_program_buffer_close(
        &mut self,
        payer: &Keypair,
        program_buffer: &Pubkey,
        program_authority: &Keypair,
        spill: &Pubkey,
    ) -> Result<(), ToolboxEndpointError> {
        let program_authority_address = &program_authority.pubkey();
        let instruction_close = close_any(
            program_buffer,
            spill,
            Some(program_authority_address),
            None,
        );
        self.process_instruction_with_signers(
            payer,
            instruction_close,
            &[program_authority],
        )
        .await?;
        Ok(())
    }

    pub async fn process_program_deploy(
        &mut self,
        payer: &Keypair,
        program_id: &Keypair,
        program_authority: &Keypair,
        program_bytecode: &[u8],
    ) -> Result<(), ToolboxEndpointError> {
        if self.get_account_exists(&program_id.pubkey()).await? {
            return Err(ToolboxEndpointError::Custom(
                "Cannot deploy on a program that already exist (need to upgrade)".to_string(),
            ));
        }
        let program_buffer = self
            .process_program_buffer_new(
                payer,
                program_bytecode,
                &program_authority.pubkey(),
            )
            .await?;
        self.process_program_buffer_deploy(
            payer,
            program_id,
            &program_buffer,
            program_authority,
            program_bytecode.len(),
        )
        .await?;
        Ok(())
    }

    pub async fn process_program_extend(
        &mut self,
        payer: &Keypair,
        program_id: &Pubkey,
        program_bytecode_len_added: usize,
    ) -> Result<(), ToolboxEndpointError> {
        let instruction_extend = extend_program(
            program_id,
            Some(&payer.pubkey()),
            u32::try_from(program_bytecode_len_added)
                .map_err(ToolboxEndpointError::TryFromInt)?,
        );
        self.process_instruction(payer, instruction_extend).await?;
        Ok(())
    }

    pub async fn process_program_upgrade(
        &mut self,
        payer: &Keypair,
        program_id: &Pubkey,
        program_authority: &Keypair,
        program_bytecode: &[u8],
        spill: &Pubkey,
    ) -> Result<(), ToolboxEndpointError> {
        let program_bytecode_len_before =
            match self.get_program_bytecode(program_id).await? {
                Some(program_bytecode) => program_bytecode.len(),
                None => {
                    return Err(ToolboxEndpointError::AccountDoesNotExist(
                        *program_id,
                        "Program Id".to_string(),
                    ))
                },
            };
        let program_bytecode_len_after = program_bytecode.len();
        if program_bytecode_len_after > program_bytecode_len_before {
            self.process_program_extend(
                payer,
                program_id,
                program_bytecode_len_after - program_bytecode_len_before,
            )
            .await?;
        }
        let program_buffer = self
            .process_program_buffer_new(
                payer,
                program_bytecode,
                &program_authority.pubkey(),
            )
            .await?;
        self.process_program_buffer_upgrade(
            payer,
            program_id,
            &program_buffer,
            program_authority,
            spill,
        )
        .await?;
        Ok(())
    }

    pub async fn process_program_close(
        &mut self,
        payer: &Keypair,
        program_id: &Pubkey,
        program_authority: &Keypair,
        spill: &Pubkey,
    ) -> Result<(), ToolboxEndpointError> {
        let program_data =
            &ToolboxEndpoint::find_program_data_from_program_id(program_id);
        let program_authority_address = &program_authority.pubkey();
        let instruction_close = close_any(
            program_data,
            spill,
            Some(program_authority_address),
            Some(program_id),
        );
        self.process_instruction_with_signers(
            payer,
            instruction_close,
            &[program_authority],
        )
        .await?;
        Ok(())
    }
}
