use {
    crate::helpers::{
        extensions::token_2022::{
            initialize_mint::Token2022InitializeMintExtension,
            token_group::Token2022TokenGroupExtension,
        },
        suite::{
            core::App,
            types::{pin_pubkey_to_addr, AppUser, PinPubkey, TestResult},
        },
    },
    pretty_assertions::assert_eq,
    solana_signer::Signer,
    spl_pod::optional_keys::OptionalNonZeroPubkey,
    spl_token_2022_interface::extension::{group_pointer::GroupPointer, ExtensionType},
};

#[test]
fn initialize_group_pointer_with_token_group() -> TestResult<()> {
    let mut app = App::new(false);
    let (_, mint_keypair) = app.token_2022_try_create_mint_account(
        AppUser::Admin,
        None,
        Some(&[ExtensionType::GroupPointer]),
    )?;

    let mint_pubkey = &mint_keypair.pubkey().to_bytes();
    let decimals: u8 = 6;
    let mint_authority = AppUser::Admin;
    let freeze_authority = Some(AppUser::Admin.pubkey());

    let group_pointer = GroupPointer {
        authority: OptionalNonZeroPubkey(pin_pubkey_to_addr(&mint_authority.pubkey())),
        group_address: OptionalNonZeroPubkey(pin_pubkey_to_addr(mint_pubkey)),
    };

    let update_authority = pin_pubkey_to_addr(&AppUser::Admin.pubkey());
    let max_size = 10;
    let token_group = spl_token_group_interface::state::TokenGroup::new(
        &pin_pubkey_to_addr(mint_pubkey),
        OptionalNonZeroPubkey(update_authority),
        max_size,
    );

    app.token_2022_try_initialize_group_pointer(
        AppUser::Admin,
        mint_pubkey,
        Some(&mint_authority.pubkey()),
        Some(mint_pubkey),
    )?;

    app.token_2022_try_initialize_mint(
        AppUser::Admin,
        mint_pubkey,
        decimals,
        &mint_authority.pubkey(),
        freeze_authority.as_ref(),
    )?;

    app.token_2022_try_initialize_token_group(
        AppUser::Admin,
        mint_pubkey,
        mint_pubkey,
        mint_authority,
        Some(&update_authority.to_bytes()),
        max_size,
    )?;

    assert_eq!(
        app.token_2022_query_group_pointer_state(mint_pubkey)?,
        group_pointer
    );
    assert_eq!(
        app.token_2022_query_token_group_state(mint_pubkey)?,
        token_group
    );

    Ok(())
}
