use {
    crate::helpers::{
        extensions::token_2022::{
            initialize_mint::Token2022InitializeMintExtension,
            token_metadata::Token2022TokenMetadataExtension,
        },
        suite::{
            core::App,
            types::{pin_pubkey_to_addr, to_c_option, AppUser, PinPubkey, TestResult},
        },
    },
    pretty_assertions::assert_eq,
    solana_keypair::Keypair,
    solana_program_option::COption,
    solana_signer::Signer,
    spl_pod::optional_keys::OptionalNonZeroPubkey,
    spl_token_2022_interface::extension::ExtensionType,
};

#[test]
fn token_metadata() -> TestResult<()> {
    let mut app = App::new(false);

    let mint_keypair = Keypair::new();
    let mint = &mint_keypair.pubkey().to_bytes();
    let decimals: u8 = 6;
    let mint_authority = AppUser::Alice.pubkey();
    let freeze_authority = Some(AppUser::Bob.pubkey());

    // let update_authority: OptionalNonZeroPubkey =
    //     OptionalNonZeroPubkey(pin_pubkey_to_addr(&AppUser::Alice.pubkey()));
    // let name = "Awesome token";
    // let symbol = "AWSM";
    // let uri = "https://awsm.fun";
    // let additional_metadata = vec![];

    // let token_metadata = &spl_token_metadata_interface::state::TokenMetadata {
    //     update_authority,
    //     mint: pin_pubkey_to_addr(mint),
    //     name: name.to_string(),
    //     symbol: symbol.to_string(),
    //     uri: uri.to_string(),
    //     additional_metadata,
    // };

    app.token_2022_try_create_mint_account_with_metadata(
        AppUser::Admin,
        Some(mint_keypair),
        &[ExtensionType::MetadataPointer],
        None,
    )?;

    app.token_2022_try_initialize_metadata_pointer(
        AppUser::Admin,
        &mint,
        Some(&AppUser::Alice.pubkey()),
        Some(&mint),
    )?;

    app.token_2022_try_initialize_mint(
        AppUser::Admin,
        &mint,
        decimals,
        &mint_authority,
        freeze_authority.as_ref(),
    )?;

    // println!(
    //     "mint_address {:#?}",
    //     solana_pubkey::Pubkey::new_from_array(*mint)
    // );
    // println!("mint {:#?}", app.token_2022_query_mint_state(mint)?);
    // println!(
    //     "pointer {:#?}",
    //     app.token_2022_query_metadata_pointer_state(mint)?
    // );

    // println!(
    //     "metadata {:#?}",
    //     app.token_2022_query_token_metadata_state(mint)?
    // );

    Ok(())
}
