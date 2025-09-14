use {
    crate::helpers::{
        extensions::token_2022::{
            group_member_pointer::Token2022GroupMemberPointerExtension,
            initialize_mint::Token2022InitializeMintExtension,
        },
        suite::{
            core::App,
            types::{to_optional_non_zero_pubkey, AppUser, PinPubkey, Target, TestResult},
        },
    },
    pretty_assertions::assert_eq,
    solana_signer::Signer,
    spl_token_2022_interface::extension::ExtensionType,
};

#[test]
fn initialize_and_update_group_member_pointer() -> TestResult<()> {
    let mut app = App::new(false);

    let (_, mint_b_kp) = app.token_2022_try_create_mint_account(
        AppUser::Admin,
        None,
        Some(&[ExtensionType::GroupMemberPointer]),
    )?;
    let mint_b = &mint_b_kp.pubkey().to_bytes();

    app.token_2022_try_initialize_group_member_pointer(
        Target::Spl,
        AppUser::Admin,
        mint_b,
        Some(&AppUser::Admin.pubkey()),
        Some(mint_b),
    )?;

    app.token_2022_try_initialize_mint(
        Target::Spl,
        AppUser::Admin,
        mint_b,
        6,
        &AppUser::Admin.pubkey(),
        Some(&AppUser::Admin.pubkey()),
    )?;

    assert_eq!(
        &app.token_2022_query_group_member_pointer(Target::Spl, mint_b)
            .map(|x| x.member_address.0.to_bytes())?,
        mint_b
    );

    app.token_2022_try_update_group_member_pointer(
        Target::Spl,
        AppUser::Admin,
        mint_b,
        &AppUser::Admin.pubkey(),
        None,
    )?;

    assert_eq!(
        app.token_2022_query_group_member_pointer(Target::Spl, mint_b)
            .map(|x| x.member_address)?,
        to_optional_non_zero_pubkey(None)
    );

    Ok(())
}

#[test]
fn proxy_initialize_and_update_group_member_pointer() -> TestResult<()> {
    let mut app = App::new(false);

    let (_, mint_b_kp) = app.token_2022_try_create_mint_account(
        AppUser::Admin,
        None,
        Some(&[ExtensionType::GroupMemberPointer]),
    )?;
    let mint_b = &mint_b_kp.pubkey().to_bytes();

    // 2nd to run internal checks
    for _ in [0..=1] {
        app.token_2022_try_initialize_group_member_pointer(
            Target::Proxy,
            AppUser::Admin,
            mint_b,
            Some(&AppUser::Admin.pubkey()),
            Some(mint_b),
        )?;
    }

    app.token_2022_try_initialize_mint(
        Target::Proxy,
        AppUser::Admin,
        mint_b,
        6,
        &AppUser::Admin.pubkey(),
        Some(&AppUser::Admin.pubkey()),
    )?;

    assert_eq!(
        &app.token_2022_query_group_member_pointer(Target::Proxy, mint_b)
            .map(|x| x.member_address.0.to_bytes())?,
        mint_b
    );

    app.token_2022_try_update_group_member_pointer(
        Target::Proxy,
        AppUser::Admin,
        mint_b,
        &AppUser::Admin.pubkey(),
        None,
    )?;

    assert_eq!(
        app.token_2022_query_group_member_pointer(Target::Proxy, mint_b)
            .map(|x| x.member_address)?,
        to_optional_non_zero_pubkey(None)
    );

    Ok(())
}
