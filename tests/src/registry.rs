use {
    crate::helpers::{
        extensions::registry::TokenExtension,
        suite::{
            core::App,
            types::{AppUser, PinPubkey, TestResult},
        },
    },
    pretty_assertions::assert_eq,
    solana_program_option::COption,
    solana_signer::Signer,
};

#[test]
fn initialize_mint_directly_default() -> TestResult<()> {
    const DECIMALS: u8 = 6;

    let mut app = App::new(false);

    let (_, mint_keypair) = app.token_2022_try_create_mint_account(AppUser::Admin, None, None)?;
    let mint = &mint_keypair.pubkey().to_bytes();

    app.token_2022_try_initialize_mint(
        AppUser::Admin,
        mint,
        DECIMALS,
        &AppUser::Alice.pubkey(),
        Some(&AppUser::Bob.pubkey()),
    )?;

    assert_eq!(
        app.token_2022_query_mint_state(mint)?,
        spl_token_2022_interface::state::Mint {
            mint_authority: COption::Some(AppUser::Alice.pubkey().into()),
            supply: 0,
            decimals: DECIMALS,
            is_initialized: true,
            freeze_authority: COption::Some(AppUser::Bob.pubkey().into())
        }
    );

    Ok(())
}

#[test]
fn initialize_mint_default() -> TestResult<()> {
    const DECIMALS: u8 = 6;

    let mut app = App::new(false);

    let (_, mint_keypair) = app.token_2022_try_create_mint_account(AppUser::Admin, None, None)?;
    let mint = &mint_keypair.pubkey().to_bytes();

    app.token_2022_proxy_try_initialize_mint(
        AppUser::Admin,
        mint,
        DECIMALS,
        &AppUser::Alice.pubkey(),
        Some(&AppUser::Bob.pubkey()),
    )?;

    assert_eq!(
        app.token_2022_query_mint_state(mint)?,
        spl_token_2022_interface::state::Mint {
            mint_authority: COption::Some(AppUser::Alice.pubkey().into()),
            supply: 0,
            decimals: DECIMALS,
            is_initialized: true,
            freeze_authority: COption::Some(AppUser::Bob.pubkey().into())
        }
    );

    // TODO: add pinocchio state cpi in the caller

    Ok(())
}
