use {
    crate::helpers::suite::{
        core::{extension::send_tx, App, ProgramId},
        types::{
            addr_to_sol_pubkey, pin_pubkey_to_addr, pin_to_sol_pubkey, AppUser, SolPubkey,
            TestError, TestResult,
        },
    },
    litesvm::types::TransactionMetadata,
    pinocchio::pubkey::Pubkey,
    solana_keypair::Keypair,
    solana_program_pack::Pack,
    spl_token_2022_interface::extension::BaseStateWithExtensions,
};

pub trait Token2022TokenGroupExtension {
    // fn token_2022_try_create_mint_account(
    //     &mut self,
    //     sender: AppUser,
    //     mint: Option<Keypair>,
    //     extensions: Option<&[spl_token_2022_interface::extension::ExtensionType]>,
    // ) -> TestResult<(TransactionMetadata, Keypair)>;

    fn token_2022_try_initialize_token_group(
        &mut self,
        sender: AppUser,
        group: &Pubkey,
        mint: &Pubkey,
        mint_authority: AppUser,
        update_authority: Option<&Pubkey>,
        max_size: u64,
    ) -> TestResult<TransactionMetadata>;

    fn token_2022_try_initialize_group_pointer(
        &mut self,
        sender: AppUser,
        mint: &Pubkey,
        authority: Option<&Pubkey>,
        group_address: Option<&Pubkey>,
    ) -> TestResult<TransactionMetadata>;

    fn token_2022_query_token_group_state(
        &self,
        group: &Pubkey,
    ) -> TestResult<spl_token_group_interface::state::TokenGroup>;

    fn token_2022_query_group_pointer_state(
        &self,
        mint: &Pubkey,
    ) -> TestResult<spl_token_2022_interface::extension::group_pointer::GroupPointer>;
}

impl Token2022TokenGroupExtension for App {
    // fn token_2022_try_create_mint_account(
    //     &mut self,
    //     sender: AppUser,
    //     mint: Option<Keypair>,
    //     extensions: Option<&[spl_token_2022_interface::extension::ExtensionType]>,
    //     additional_size: usize,
    // ) -> TestResult<(TransactionMetadata, Keypair)> {
    //     let ProgramId {
    //         token_2022_program, ..
    //     } = self.program_id;

    //     let account_size = match extensions {
    //         Some(x) => {
    //             let base_size = spl_token_2022_interface::extension::ExtensionType::try_calculate_account_len::<
    //             spl_token_2022_interface::state::Mint,
    //             >(x)
    //             .map_err(TestError::from_raw_error)?;

    //             // Add TOKEN_GROUP_SIZE if TokenGroup extension is present
    //             if x.contains(&spl_token_2022_interface::extension::ExtensionType::TokenGroup) {
    //                 base_size + additional_size
    //             } else {
    //                 base_size
    //             }
    //         }
    //         None => spl_token_2022_interface::state::Mint::LEN,
    //     };

    //     self.create_account(sender, mint, account_size, &token_2022_program)
    // }

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

        let ix = spl_token_group_interface::instruction::initialize_group(
            &token_2022_program.to_bytes().into(),
            &pin_pubkey_to_addr(group),
            &pin_pubkey_to_addr(mint),
            &mint_authority.pubkey().to_bytes().into(),
            update_authority.map(pin_pubkey_to_addr),
            max_size,
        );

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

    fn token_2022_try_initialize_group_pointer(
        &mut self,
        sender: AppUser,
        mint: &Pubkey,
        authority: Option<&Pubkey>,
        group_address: Option<&Pubkey>,
    ) -> TestResult<TransactionMetadata> {
        let ProgramId {
            token_2022_program, ..
        } = self.program_id;

        let signers = &[&sender.keypair()];

        let ix = spl_token_2022_interface::extension::group_pointer::instruction::initialize(
            &token_2022_program.to_bytes().into(),
            &pin_pubkey_to_addr(mint),
            authority.map(pin_pubkey_to_addr),
            group_address.map(pin_pubkey_to_addr),
        )
        .map_err(TestError::from_raw_error)?;

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
            .ok_or(TestError::from_raw_error("The state isn't found"))?;

        bytemuck::try_from_bytes::<spl_token_group_interface::state::TokenGroup>(&account.data)
            .map(|&group_state| group_state)
            .map_err(TestError::from_raw_error)
    }

    fn token_2022_query_group_pointer_state(
        &self,
        mint: &Pubkey,
    ) -> TestResult<spl_token_2022_interface::extension::group_pointer::GroupPointer> {
        let account = self
            .litesvm
            .get_account(&pin_to_sol_pubkey(mint))
            .ok_or(TestError::from_raw_error("The account isn't found"))?;

        // Parse the mint account with extensions
        let mint_with_extensions = spl_token_2022_interface::extension::StateWithExtensions::<
            spl_token_2022_interface::state::Mint,
        >::unpack(&account.data)
        .map_err(TestError::from_raw_error)?;

        // Get the GroupPointer extension
        mint_with_extensions
            .get_extension::<spl_token_2022_interface::extension::group_pointer::GroupPointer>()
            .map(|&x| x)
            .map_err(TestError::from_raw_error)
    }
}
