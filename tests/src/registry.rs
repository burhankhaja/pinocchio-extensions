use {
    crate::helpers::{
        extensions::registry::TokenExtension,
        suite::{
            core::App,
            types::{AppUser, PinPubkey, TestError, TestResult},
        },
    },
    pretty_assertions::assert_eq,
    solana_program_option::COption,
    solana_program_pack::Pack,
    solana_signer::Signer,
};

#[test]
fn initialize_mint_default() -> TestResult<()> {
    const DECIMALS: u8 = 6;

    let mut app = App::new(true);

    let (_, mint_keypair) = app.token_try_create_mint_account(AppUser::Admin, None, None)?;
    app.token_try_initialize_mint(
        AppUser::Admin,
        &mint_keypair.pubkey().to_bytes(),
        DECIMALS,
        &AppUser::Alice.pubkey(),
        Some(&AppUser::Bob.pubkey()),
    )?;

    let mint_data = app
        .litesvm
        .get_account(&mint_keypair.pubkey())
        .map(|x| spl_token_2022_interface::state::Mint::unpack_from_slice(&x.data))
        .transpose()
        .map_err(TestError::from_raw_error)?
        .ok_or(TestError::from_raw_error("mint data is not found"))?;

    assert_eq!(
        mint_data,
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
