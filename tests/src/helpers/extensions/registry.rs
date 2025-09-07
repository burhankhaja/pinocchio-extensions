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
    solana_signer::Signer,
};

pub trait TokenExtension {
    fn token_try_initialize_mint(
        &mut self,
        sender: AppUser,
        mint: &Pubkey,
        decimals: u8,
        mint_authority: &Pubkey,
        freeze_authority: Option<&Pubkey>,
    ) -> TestResult<TransactionMetadata>;

    fn create_mint_account(
        &mut self,
        sender: AppUser,
        mint: Keypair,
    ) -> TestResult<TransactionMetadata>;

    // fn token_query_config(&self) -> TestResult<Config>;
}

impl TokenExtension for App {
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

    // TODO: update create_token_mint to support token-2022
    fn create_mint_account(
        &mut self,
        sender: AppUser,
        mint: Keypair,
    ) -> TestResult<TransactionMetadata> {
        let ProgramId {
            token_2022_program,
            system_program,
            ..
        } = self.program_id;

        // Token-2022 mint account size (this is larger than Token-1 mint)
        // Token-2022 mint base size is 82 bytes, but may need extensions
        const MINT_ACCOUNT_SIZE: u64 = 82;

        // Calculate rent exemption for mint account
        let rent = self
            .litesvm
            .get_sysvar::<solana_program::sysvar::rent::Rent>();
        let lamports = rent.minimum_balance(MINT_ACCOUNT_SIZE as usize);

        let create_account_ix = solana_instruction::Instruction {
            program_id: system_program,
            accounts: vec![
                solana_instruction::AccountMeta::new(sender.pubkey(), true), // payer
                solana_instruction::AccountMeta::new(mint.pubkey(), true), // mint account to create
            ],
            data: {
                let mut data = Vec::new();
                data.extend_from_slice(&0u32.to_le_bytes()); // CreateAccount instruction
                data.extend_from_slice(&lamports.to_le_bytes()); // lamports
                data.extend_from_slice(&MINT_ACCOUNT_SIZE.to_le_bytes()); // space
                data.extend_from_slice(&token_2022_program.to_bytes()); // owner (Token-2022 program)
                data
            },
        };

        let signers = &[sender.keypair(), mint];

        send_tx(
            &mut self.litesvm,
            &[create_account_ix],
            signers,
            self.is_log_displayed,
        )
    }

    // fn token_query_config(&self) -> TestResult<Config> {
    //     get_data(&self.litesvm, &self.pda.token_config())
    // }
}
