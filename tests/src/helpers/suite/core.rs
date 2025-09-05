use {
    crate::helpers::suite::{
        core::sol_kite::create_token_mint,
        types::{
            AppAsset, AppCoin, AppToken, AppUser, GetDecimals, SolPubkey, TestError, TestResult,
        },
    },
    bytemuck::{checked::try_from_bytes, Pod, Zeroable},
    litesvm::{types::TransactionMetadata, LiteSVM},
    pinocchio::program_error,
    solana_compute_budget::compute_budget::ComputeBudget,
    solana_instruction::{AccountMeta, Instruction},
    solana_keypair::Keypair,
    solana_kite::{deploy_program, get_token_account_balance, mint_tokens_to_account},
    solana_program::{
        clock::Clock, native_token::LAMPORTS_PER_SOL, program_option::COption, program_pack::Pack,
        system_instruction, system_program,
    },
    solana_pubkey::Pubkey,
    solana_signer::signers::Signers,
    solana_transaction::Transaction,
    spl_associated_token_account::get_associated_token_address,
    spl_token::state::Mint,
    strum::IntoEnumIterator,
};

pub const PROGRAM_NAME_TOKEN: &str = "token";

pub mod sol_kite {
    use {
        litesvm::LiteSVM, solana_keypair::Keypair, solana_kite::SolanaKiteError,
        solana_message::Message, solana_pubkey::Pubkey, solana_signer::Signer,
        solana_transaction::Transaction,
        spl_associated_token_account::instruction::create_associated_token_account as create_ata_instruction,
    };

    pub fn create_associated_token_account(
        litesvm: &mut LiteSVM,
        owner: &Pubkey,
        mint: &Pubkey,
        payer: &Keypair,
    ) -> Result<Pubkey, SolanaKiteError> {
        let associated_token_account =
            spl_associated_token_account::get_associated_token_address(owner, mint);

        let create_ata_instruction =
            create_ata_instruction(&payer.pubkey(), owner, mint, &spl_token::id());

        let message = Message::new(&[create_ata_instruction], Some(&payer.pubkey()));
        let mut transaction = Transaction::new_unsigned(message);
        let blockhash = litesvm.latest_blockhash();
        transaction.sign(&[payer], blockhash);

        litesvm.send_transaction(transaction).map_err(|e| {
            SolanaKiteError::TokenOperationFailed(format!(
                "Failed to create associated token account: {:?}",
                e
            ))
        })?;

        Ok(associated_token_account)
    }

    pub fn create_token_mint(
        litesvm: &mut LiteSVM,
        mint_authority: &Keypair,
        decimals: u8,
        mint: Option<Pubkey>,
    ) -> Result<Pubkey, SolanaKiteError> {
        let mint = mint.unwrap_or(Pubkey::new_unique());
        let rent = litesvm.minimum_balance_for_rent_exemption(82);

        litesvm
            .set_account(
                mint,
                solana_account::Account {
                    lamports: rent,
                    data: vec![0u8; 82],
                    owner: spl_token::ID,
                    executable: false,
                    rent_epoch: 0,
                },
            )
            .map_err(|e| {
                SolanaKiteError::TokenOperationFailed(format!(
                    "Failed to create mint account: {:?}",
                    e
                ))
            })?;

        let initialize_mint_instruction = spl_token::instruction::initialize_mint(
            &spl_token::ID,
            &mint,
            &mint_authority.pubkey(),
            None,
            decimals,
        )
        .map_err(|e| {
            SolanaKiteError::TokenOperationFailed(format!(
                "Failed to create initialize mint instruction: {:?}",
                e
            ))
        })?;

        let message = Message::new(
            &[initialize_mint_instruction],
            Some(&mint_authority.pubkey()),
        );
        let mut transaction = Transaction::new_unsigned(message);
        let blockhash = litesvm.latest_blockhash();
        transaction.sign(&[mint_authority], blockhash);

        litesvm.send_transaction(transaction).map_err(|e| {
            SolanaKiteError::TokenOperationFailed(format!("Failed to initialize mint: {:?}", e))
        })?;

        Ok(mint)
    }
}

pub struct ProgramId {
    // standard
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,

    // custom
    pub token: Pubkey,
}

pub struct Pda {
    token_program_id: Pubkey,
}

#[allow(clippy::useless_vec)]
impl Pda {
    // token
    //
    // pub fn registry_config(&self) -> Pubkey {
    //     get_pda_and_bump(
    //         &seeds![registry_cpi::state::seed::CONFIG],
    //         &self.token_program_id,
    //     )
    //     .0
    // }
}

pub struct App {
    pub litesvm: LiteSVM,
    is_log_displayed: bool,

    pub program_id: ProgramId,
    pub pda: Pda,
}

impl App {
    pub fn create_app_with_programs(is_log_displayed: bool) -> Self {
        // prepare environment with balances
        let mut litesvm = Self::init_env_with_balances();

        // specify programs
        let program_id = ProgramId {
            // standard
            system_program: system_program::ID,
            token_program: spl_token::ID,
            associated_token_program: spl_associated_token_account::ID,

            // custom
            token: pinocchio_token::ID.into(),
        };

        // specify PDA
        let pda = Pda {
            token_program_id: program_id.token,
        };

        // upload custom programs
        upload_program(&mut litesvm, PROGRAM_NAME_TOKEN, &program_id.token);

        Self {
            litesvm,
            is_log_displayed,

            program_id,
            pda,
        }
    }

    pub fn new(is_log_displayed: bool) -> Self {
        let mut app = Self::create_app_with_programs(is_log_displayed);
        app.create_wsol();

        // prepare programs
        // ...

        app
    }

    fn init_env_with_balances() -> LiteSVM {
        let mut litesvm = LiteSVM::new().with_compute_budget(ComputeBudget {
            compute_unit_limit: 10_000_000,
            ..ComputeBudget::default()
        });

        // airdrop SOL
        for user in AppUser::iter() {
            litesvm
                .airdrop(
                    &user.pubkey(),
                    user.get_initial_asset_amount(AppCoin::SOL) * LAMPORTS_PER_SOL,
                )
                .unwrap();
        }

        // create tokens
        for token in AppToken::iter() {
            // skip WSOL
            if token == AppToken::WSOL {
                continue;
            }

            create_token_mint(
                &mut litesvm,
                &AppUser::Admin.keypair(),
                token.get_decimals(),
                Some(token.pubkey()),
            )
            .unwrap();
        }

        // mint tokens
        for user in AppUser::iter() {
            for token in AppToken::iter() {
                // skip WSOL
                if token == AppToken::WSOL {
                    continue;
                }

                let ata = App::create_ata(
                    &mut litesvm,
                    &AppUser::Admin.keypair(),
                    &user.pubkey(),
                    &token.pubkey(),
                )
                .unwrap();

                mint_tokens_to_account(
                    &mut litesvm,
                    &token.pubkey(),
                    &ata,
                    user.get_initial_asset_amount(token) * 10u64.pow(token.get_decimals() as u32),
                    &AppUser::Admin.keypair(),
                )
                .unwrap();
            }
        }

        litesvm
    }

    fn create_wsol(&mut self) {
        let mut mint_account = solana_account::Account {
            lamports: self.litesvm.minimum_balance_for_rent_exemption(Mint::LEN),
            data: vec![0; Mint::LEN],
            owner: spl_token::ID,
            executable: false,
            rent_epoch: 0,
        };

        let mut mint_data = spl_token::state::Mint::unpack_unchecked(&mint_account.data).unwrap();
        mint_data.mint_authority = COption::Some(Pubkey::new_unique());
        mint_data.decimals = 9;
        mint_data.supply = 0;
        mint_data.is_initialized = true;
        mint_data.freeze_authority = COption::None;
        mint_data.pack_into_slice(&mut mint_account.data);

        self.litesvm
            .set_account(AppToken::WSOL.pubkey(), mint_account)
            .unwrap();
    }

    // utils

    pub fn get_clock_time(&self) -> u64 {
        self.litesvm.get_sysvar::<Clock>().unix_timestamp as u64
    }

    pub fn wait(&mut self, delay_s: u64) {
        let mut clock = self.litesvm.get_sysvar::<Clock>();
        clock.unix_timestamp += delay_s as i64;
        clock.slot += 25 * delay_s / 10;

        self.litesvm.set_sysvar::<Clock>(&clock);
    }

    pub fn transfer_asset(
        &mut self,
        sender: AppUser,
        recipient: &Pubkey,
        amount: u64,
        asset: impl Into<AppAsset>,
    ) -> TestResult<TransactionMetadata> {
        match asset.into() {
            AppAsset::Coin(_) => self.transfer_sol(sender, recipient, amount),
            AppAsset::Token(mint) => self.transfer_token(sender, recipient, amount, mint),
        }
    }

    pub fn transfer_sol(
        &mut self,
        sender: AppUser,
        recipient: &Pubkey,
        amount: u64,
    ) -> TestResult<TransactionMetadata> {
        let payer = &sender.pubkey();
        let signers = &[sender.keypair()];
        let ix = system_instruction::transfer(payer, recipient, amount);

        extension::send_tx(&mut self.litesvm, &[ix], signers, self.is_log_displayed)
    }

    pub fn transfer_token(
        &mut self,
        sender: AppUser,
        recipient: &Pubkey,
        amount: u64,
        token: AppToken,
    ) -> TestResult<TransactionMetadata> {
        let payer = sender.pubkey();
        let signers = &[sender.keypair()];

        let mint = token.pubkey();
        let sender_ata = self.get_or_create_ata(sender, &sender.pubkey(), &mint)?;
        let recipient_ata = self.get_or_create_ata(sender, recipient, &mint)?;

        let ix = spl_token::instruction::transfer(
            &self.program_id.token_program,
            &sender_ata,
            &recipient_ata,
            &payer,
            &[&payer],
            amount,
        )
        .map_err(TestError::from_unknown)?;

        extension::send_tx(&mut self.litesvm, &[ix], signers, self.is_log_displayed)
    }

    pub fn get_balance(&self, user: AppUser, asset: impl Into<AppAsset>) -> u64 {
        let address = &user.pubkey();

        match asset.into() {
            AppAsset::Coin(_) => self.get_coin_balance(address),
            AppAsset::Token(mint) => self.get_ata_token_balance(address, &mint.pubkey()),
        }
    }

    pub fn get_coin_balance(&self, address: &Pubkey) -> u64 {
        self.litesvm.get_balance(address).unwrap_or_default()
    }

    pub fn get_ata_token_balance(&self, address: &Pubkey, mint: &Pubkey) -> u64 {
        get_token_account_balance(&self.litesvm, &Self::get_ata(address, mint)).unwrap_or_default()
    }

    pub fn get_pda_token_balance(&self, token_account: &Pubkey) -> u64 {
        get_token_account_balance(&self.litesvm, token_account).unwrap_or_default()
    }

    pub fn get_or_create_ata(
        &mut self,
        sender: AppUser,
        owner: &Pubkey,
        mint: &Pubkey,
    ) -> TestResult<Pubkey> {
        let ata_address = Self::get_ata(owner, mint);

        // check if the ATA already exists
        match self.litesvm.get_account(&ata_address) {
            // ATA exists, return its address
            Some(_) => Ok(ata_address),
            // ATA doesn't exist, create it
            None => Self::create_ata(&mut self.litesvm, &sender.keypair(), owner, mint),
        }
    }

    pub fn get_ata(owner: &Pubkey, mint: &Pubkey) -> Pubkey {
        get_associated_token_address(owner, mint)
    }

    pub fn create_ata(
        litesvm: &mut LiteSVM,
        sender: &Keypair,
        owner: &Pubkey,
        mint: &Pubkey,
    ) -> TestResult<Pubkey> {
        sol_kite::create_associated_token_account(litesvm, owner, mint, sender)
            .map_err(TestError::from_unknown)
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new(false)
    }
}

pub fn get_test_error_from_logs(logs: &[String]) -> TestError {
    match TestError::parse_program_error(logs) {
        Some(program_error) => TestError::from_unknown(program_error),
        None => {
            TestError::from_unknown("Not a ProgramError and custom_program_error_idx isn't found")
        }
    }
}

pub fn get_program_size(program_name: &str) -> TestResult<u64> {
    let program_path = &get_program_path(program_name);

    std::fs::metadata(program_path)
        .map(|x| x.len())
        .map_err(|_| TestError {
            info: format!("{} program isn't found!", program_path),
            index: None,
        })
}

fn get_program_path(program_name: &str) -> String {
    const PROGRAM_PATH: &str = "../target/deploy/";
    format!("{}{}.so", PROGRAM_PATH, program_name)
}

fn get_dumps_path(program_name: &str) -> String {
    const DUMPS_PATH: &str = "./src/helpers/dumps/";
    format!("{}{}.so", DUMPS_PATH, program_name)
}

fn upload_program(litesvm: &mut LiteSVM, program_name: &str, program_id: &Pubkey) {
    // try to deploy custom programs first, if it doesn't work then deploy dumps
    if deploy_program(litesvm, program_id, &get_program_path(program_name)).is_err() {
        deploy_program(litesvm, program_id, &get_dumps_path(program_name)).unwrap()
    }
}

pub mod extension {
    use {super::*, crate::helpers::suite::types::Result};

    fn deserialize<T>(data: &[u8]) -> Result<&T>
    where
        T: Pod + Zeroable,
    {
        try_from_bytes(data).map_err(|_| program_error::ProgramError::InvalidAccountData)
    }

    pub fn get_data<T>(litesvm: &LiteSVM, pda: &Pubkey) -> TestResult<T>
    where
        T: Pod + Zeroable,
    {
        match litesvm.get_account(pda) {
            Some(account) => {
                Ok(*deserialize::<T>(&account.data).map_err(TestError::from_raw_error)?)
            }
            _ => Err(TestError::from_raw_error(
                program_error::ProgramError::UninitializedAccount,
            )),
        }
    }

    pub fn send_tx<S>(
        litesvm: &mut LiteSVM,
        instructions: &[Instruction],
        signers: &S,
        is_log_displayed: bool,
    ) -> TestResult<TransactionMetadata>
    where
        S: Signers + ?Sized,
    {
        // to avoid AlreadyProcessed error
        litesvm.expire_blockhash();

        let transaction = Transaction::new_signed_with_payer(
            instructions,
            None, // 1st account by default
            signers,
            litesvm.latest_blockhash(),
        );

        match litesvm.send_transaction(transaction) {
            Ok(x) => {
                let logs = &x.logs;

                if is_log_displayed {
                    println!("Transaction logs: {:#?}\n", logs);
                }

                Ok(x)
            }
            Err(e) => {
                let logs = &e.meta.logs;

                if is_log_displayed {
                    println!("Transaction logs: {:#?}\n", logs);
                }

                Err(get_test_error_from_logs(logs))
            }
        }
    }

    pub fn send_tx_with_ix<S>(
        app: &mut App,
        program_id: &Pubkey,
        accounts: &[AccountMeta],
        instruction_data: &[u8],
        signers: &S,
        remaining_accounts: &[AccountMeta],
    ) -> TestResult<TransactionMetadata>
    where
        S: Signers + ?Sized,
    {
        let ix = Instruction {
            program_id: *program_id,
            accounts: [accounts, remaining_accounts].concat(),
            data: instruction_data.to_vec(),
        };

        send_tx(&mut app.litesvm, &[ix], signers, app.is_log_displayed)
    }
}

pub fn assert_error<T: Sized + std::fmt::Debug>(err: TestError, expected: T) {
    let expected_error_name = format!("{:?}", expected).replace("\"", "");

    let error = err.info;
    let contains_name = error.contains(&expected_error_name);

    pretty_assertions::assert_eq!(
        "",
        if contains_name { "" } else { " " },
        "\n\n✅ Expected error:\n{}\n\n❌ Received error:\n{}",
        expected_error_name,
        error
    );
}
