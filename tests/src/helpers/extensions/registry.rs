use {
    crate::helpers::suite::{
        core::{extension::send_tx, App, ProgramId},
        types::{
            addr_to_sol_pubkey, pin_pubkey_to_addr, AppUser, SolPubkey, TestError, TestResult,
        },
    },
    litesvm::types::TransactionMetadata,
    pinocchio::pubkey::Pubkey,
    solana_keypair::Keypair,
    solana_program::system_instruction,
    solana_program_pack::Pack,
    solana_signer::Signer,
};

pub trait TokenExtension {
    fn token_try_create_mint_account(
        &mut self,
        sender: AppUser,
        mint: Option<Keypair>,
        extensions: Option<&[spl_token_2022_interface::extension::ExtensionType]>,
    ) -> TestResult<(TransactionMetadata, Keypair)>;

    fn token_try_initialize_mint(
        &mut self,
        sender: AppUser,
        mint: &Pubkey,
        decimals: u8,
        mint_authority: &Pubkey,
        freeze_authority: Option<&Pubkey>,
    ) -> TestResult<TransactionMetadata>;

    // fn token_query_config(&self) -> TestResult<Config>;
}

impl TokenExtension for App {
    fn token_try_create_mint_account(
        &mut self,
        sender: AppUser,
        mint: Option<Keypair>,
        extensions: Option<&[spl_token_2022_interface::extension::ExtensionType]>,
    ) -> TestResult<(TransactionMetadata, Keypair)> {
        let ProgramId {
            token_2022_program, ..
        } = self.program_id;

        // Generate mint keypair if not provided
        let mint_keypair = mint.unwrap_or(Keypair::new());
        let signers = &[&sender.keypair(), &mint_keypair];

        // Calculate account size with extensions
        let account_size = match extensions {
            Some(x) => {
                spl_token_2022_interface::extension::ExtensionType::try_calculate_account_len::<
                    spl_token_2022_interface::state::Mint,
                >(x)
                .map_err(|e| {
                    TestError::from_raw_error(format!(
                        "Failed to calculate account length: {:?}",
                        e
                    ))
                })?
            }
            None => spl_token_2022_interface::state::Mint::LEN,
        };

        // Get rent exemption amount
        let lamports = self
            .litesvm
            .get_sysvar::<solana_program::sysvar::rent::Rent>()
            .minimum_balance(account_size);

        // Use system instruction helper instead of manual instruction creation
        let create_account_ix = system_instruction::create_account(
            &sender.pubkey(),
            &mint_keypair.pubkey(),
            lamports,
            account_size as u64,
            &token_2022_program,
        );

        let tx_metadata = send_tx(
            &mut self.litesvm,
            &[create_account_ix],
            signers,
            self.is_log_displayed,
        )?;

        Ok((tx_metadata, mint_keypair))
    }

    fn token_try_initialize_mint(
        &mut self,
        sender: AppUser,
        mint: &Pubkey,
        decimals: u8,
        mint_authority: &Pubkey,
        freeze_authority: Option<&Pubkey>,
    ) -> TestResult<TransactionMetadata> {
        // programs
        let ProgramId {
            token_2022_program,
            token_2022_caller,
            ..
        } = self.program_id;

        // signers
        let signers = &[sender.keypair()];

        // instruction builder checks program_id, token_2022_program must be specified here
        let ix = spl_token_2022_interface::instruction::initialize_mint(
            &pin_pubkey_to_addr(&token_2022_program.to_bytes()),
            &pin_pubkey_to_addr(mint),
            &pin_pubkey_to_addr(mint_authority),
            freeze_authority.map(pin_pubkey_to_addr).as_ref(),
            decimals,
        )
        .map_err(TestError::from_raw_error)?;

        // required by runtime to validate programs
        let additional_accounts = [solana_instruction::AccountMeta::new_readonly(
            token_2022_program,
            false,
        )];

        // convert Instruction v3.0.0 to Instruction v2.3.0
        // program_id should be replaced now: token_2022_program -> token_2022_caller
        let ix_legacy = solana_instruction::Instruction {
            program_id: token_2022_caller,
            accounts: ix
                .accounts
                .into_iter()
                .map(|x| solana_instruction::AccountMeta {
                    pubkey: addr_to_sol_pubkey(&x.pubkey),
                    is_signer: x.is_signer,
                    is_writable: x.is_writable,
                })
                .chain(additional_accounts.into_iter())
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

    // fn token_query_config(&self) -> TestResult<Config> {
    //     get_data(&self.litesvm, &self.pda.token_config())
    // }
}
