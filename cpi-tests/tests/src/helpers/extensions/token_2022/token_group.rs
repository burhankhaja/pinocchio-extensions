use {
    crate::helpers::suite::{
        core::{extension::send_tx, App, ProgramId},
        types::{
            addr_to_sol_pubkey, pin_pubkey_to_addr, pin_to_sol_pubkey, to_optional_non_zero_pubkey,
            AppUser, SolPubkey, TestError, TestResult,
        },
    },
    litesvm::types::TransactionMetadata,
    pinocchio::pubkey::Pubkey,
    solana_keypair::Keypair,
    solana_signer::Signer,
    spl_pod::bytemuck::pod_from_bytes,
    spl_token_2022_interface::extension::BaseStateWithExtensions,
};

pub trait Token2022TokenGroupExtension {
    fn token_2022_try_initialize_token_group(
        &mut self,
        sender: AppUser,
        group: &Pubkey,
        mint: &Pubkey,
        mint_authority: AppUser,
        update_authority: Option<&Pubkey>,
        max_size: u64,
    ) -> TestResult<TransactionMetadata>;

    fn token_2022_try_update_group_max_size(
        &mut self,
        sender: AppUser,
        group: &Pubkey,
        mint_authority: AppUser,
        update_authority: &Pubkey,
        max_size: u64,
    ) -> TestResult<TransactionMetadata>;

    fn token_2022_try_update_group_authority(
        &mut self,
        sender: AppUser,
        group: &Pubkey,
        current_authority: &Pubkey,
        new_authority: Option<&Pubkey>,
    ) -> TestResult<TransactionMetadata>;

    fn token_2022_try_initialize_member(
        &mut self,
        sender: AppUser,
        group: &Pubkey,
        group_update_authority: &Keypair,
        member: &Pubkey,
        member_mint: &Pubkey,
        member_mint_authority: &Keypair,
    ) -> TestResult<TransactionMetadata>;

    fn token_2022_query_token_group_state(
        &self,
        group: &Pubkey,
    ) -> TestResult<spl_token_group_interface::state::TokenGroup>;

    fn token_2022_proxy_query_token_group_state(
        &self,
        group: &Pubkey,
    ) -> TestResult<spl_token_group_interface::state::TokenGroup>;
}

impl Token2022TokenGroupExtension for App {
    fn token_2022_try_initialize_token_group(
        &mut self,
        sender: AppUser,
        group: &Pubkey,
        mint: &Pubkey,
        mint_authority: AppUser,
        update_authority: Option<&Pubkey>,
        max_size: u64,
    ) -> TestResult<TransactionMetadata> {
        let ProgramId {
            token_2022_program, ..
        } = self.program_id;

        let signers = &[&sender.keypair(), &mint_authority.keypair()];

        let lamports = self
            .litesvm
            .get_sysvar::<solana_program::sysvar::rent::Rent>()
            .minimum_balance(max_size as usize);
        self.transfer_sol(sender, &pin_to_sol_pubkey(&mint), lamports)?;

        let ix = spl_token_group_interface::instruction::initialize_group(
            &token_2022_program.to_bytes().into(),
            &pin_pubkey_to_addr(group),
            &pin_pubkey_to_addr(mint),
            &mint_authority.pubkey().to_bytes().into(),
            update_authority.map(pin_pubkey_to_addr),
            max_size,
        );

        // discriminator 121, 113, 108, 39, 54, 51, 0, 4
        // println!("update_authority: \n{:?}\n", update_authority);
        // println!("max_size: \n{:?}\n", max_size);
        // println!("initialize_token_group: \n{:?}\n", ix.data);

        let ix_legacy = solana_instruction::Instruction {
            program_id: addr_to_sol_pubkey(&ix.program_id),
            accounts: ix
                .accounts
                .into_iter()
                .map(|x| solana_instruction::AccountMeta {
                    pubkey: addr_to_sol_pubkey(&x.pubkey),
                    is_signer: x.is_signer,
                    is_writable: x.is_writable,
                })
                .collect(),
            data: ix.data,
        };

        send_tx(
            &mut self.litesvm,
            &[ix_legacy],
            signers,
            self.is_log_displayed,
        )
    }

    fn token_2022_try_update_group_max_size(
        &mut self,
        sender: AppUser,
        group: &Pubkey,
        mint_authority: AppUser,
        update_authority: &Pubkey,
        max_size: u64,
    ) -> TestResult<TransactionMetadata> {
        let ProgramId {
            token_2022_program, ..
        } = self.program_id;

        let signers = &[&sender.keypair(), &mint_authority.keypair()];

        // let lamports = self
        //     .litesvm
        //     .get_sysvar::<solana_program::sysvar::rent::Rent>()
        //     .minimum_balance(max_size as usize);
        //  self.transfer_sol(sender, &pin_to_sol_pubkey(&mint), lamports)?;

        let ix = spl_token_group_interface::instruction::update_group_max_size(
            &token_2022_program.to_bytes().into(),
            &pin_pubkey_to_addr(group),
            &pin_pubkey_to_addr(update_authority),
            max_size,
        );

        // discriminator 108, 37, 171, 143, 248, 30, 18, 110
        // println!("max_size: \n{:?}\n", max_size);
        // println!("update_group_max_size: \n{:?}\n", ix.data);

        let ix_legacy = solana_instruction::Instruction {
            program_id: addr_to_sol_pubkey(&ix.program_id),
            accounts: ix
                .accounts
                .into_iter()
                .map(|x| solana_instruction::AccountMeta {
                    pubkey: addr_to_sol_pubkey(&x.pubkey),
                    is_signer: x.is_signer,
                    is_writable: x.is_writable,
                })
                .collect(),
            data: ix.data,
        };

        send_tx(
            &mut self.litesvm,
            &[ix_legacy],
            signers,
            self.is_log_displayed,
        )
    }

    fn token_2022_try_update_group_authority(
        &mut self,
        sender: AppUser,
        group: &Pubkey,
        current_authority: &Pubkey,
        new_authority: Option<&Pubkey>,
    ) -> TestResult<TransactionMetadata> {
        let ProgramId {
            token_2022_program, ..
        } = self.program_id;

        let signers = &[&sender.keypair()];

        let ix = spl_token_group_interface::instruction::update_group_authority(
            &token_2022_program.to_bytes().into(),
            &pin_pubkey_to_addr(group),
            &pin_pubkey_to_addr(current_authority),
            new_authority.map(pin_pubkey_to_addr),
        );

        // discriminator 161, 105, 88, 1, 237, 221, 216, 203
        // println!("new_authority: \n{:?}\n", new_authority);
        // println!("update_group_authority: \n{:?}\n", ix.data);

        let ix_legacy = solana_instruction::Instruction {
            program_id: addr_to_sol_pubkey(&ix.program_id),
            accounts: ix
                .accounts
                .into_iter()
                .map(|x| solana_instruction::AccountMeta {
                    pubkey: addr_to_sol_pubkey(&x.pubkey),
                    is_signer: x.is_signer,
                    is_writable: x.is_writable,
                })
                .collect(),
            data: ix.data,
        };

        send_tx(
            &mut self.litesvm,
            &[ix_legacy],
            signers,
            self.is_log_displayed,
        )
    }

    fn token_2022_try_initialize_member(
        &mut self,
        sender: AppUser,
        group: &Pubkey,
        group_update_authority: &Keypair,
        member: &Pubkey,
        member_mint: &Pubkey,
        member_mint_authority: &Keypair,
    ) -> TestResult<TransactionMetadata> {
        let ProgramId {
            token_program,
            // token_2022_program,
            ..
        } = self.program_id;

        let signers = &[
            &sender.keypair(),
            member_mint_authority,
            group_update_authority,
        ];

        let ix = spl_token_group_interface::instruction::initialize_member(
            &token_program.to_bytes().into(),
            &pin_pubkey_to_addr(member),
            &pin_pubkey_to_addr(member_mint),
            &member_mint_authority.pubkey().to_bytes().into(),
            &pin_pubkey_to_addr(group),
            &group_update_authority.pubkey().to_bytes().into(),
        );

        // discriminator 152, 32, 222, 176, 223, 237, 116, 134
        // println!("initialize_member: \n{:?}\n", ix.data);

        let ix_legacy = solana_instruction::Instruction {
            program_id: addr_to_sol_pubkey(&ix.program_id),
            accounts: ix
                .accounts
                .into_iter()
                .map(|x| solana_instruction::AccountMeta {
                    pubkey: addr_to_sol_pubkey(&x.pubkey),
                    is_signer: x.is_signer,
                    is_writable: x.is_writable,
                })
                .collect(),
            data: ix.data,
        };

        send_tx(
            &mut self.litesvm,
            &[ix_legacy],
            signers,
            self.is_log_displayed,
        )
    }

    fn token_2022_query_token_group_state(
        &self,
        group: &Pubkey,
    ) -> TestResult<spl_token_group_interface::state::TokenGroup> {
        let account = self
            .litesvm
            .get_account(&pin_to_sol_pubkey(group))
            .ok_or(TestError::from_raw_error("The account isn't found"))?;

        // Parse the mint account with extensions
        let mint_with_extensions = spl_token_2022_interface::extension::StateWithExtensions::<
            spl_token_2022_interface::state::Mint,
        >::unpack(&account.data)
        .map_err(TestError::from_raw_error)?;

        // Get extension bytes
        let extension_bytes = mint_with_extensions
            .get_extension_bytes::<spl_token_group_interface::state::TokenGroup>()
            .map_err(TestError::from_raw_error)?;

        // Deserialize extension bytes
        pod_from_bytes::<spl_token_group_interface::state::TokenGroup>(extension_bytes)
            .map(|&x| x)
            .map_err(TestError::from_raw_error)
    }

    fn token_2022_proxy_query_token_group_state(
        &self,
        group: &Pubkey,
    ) -> TestResult<spl_token_group_interface::state::TokenGroup> {
        let data = &self
            .litesvm
            .get_account(&pin_to_sol_pubkey(group))
            .map(|x| x.data)
            .ok_or(TestError::from_raw_error("The state isn't found"))?;

        let state = pinocchio_token_2022::instructions::extension::token_group::states::TokenGroup::from_bytes(data).map_err(TestError::from_raw_error)?;

        Ok(spl_token_group_interface::state::TokenGroup {
            update_authority: to_optional_non_zero_pubkey(state.update_authority()),
            mint: pin_pubkey_to_addr(state.mint()),
            size: state.size().into(),
            max_size: state.max_size().into(),
        })
    }
}
