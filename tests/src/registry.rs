use {
    crate::helpers::{
        extensions::registry::TokenExtension,
        suite::{
            core::App,
            types::{AppUser, PinPubkey, TestResult},
        },
    },
    std::io::Read,
};

#[test]
fn init_default() -> TestResult<()> {
    let mut app = App::new(true);

    let mint = &pinocchio_pubkey::from_str("9qsaqMY2DKDdhtmAzCtHBnCuahCPBJuWFJx5wQDWdJeS");

    println!("admin: {:?}\n", AppUser::Admin.pubkey().bytes());

    app.token_try_initialize_mint(
        AppUser::Admin,
        mint,
        6,
        &AppUser::Admin.pubkey(),
        Some(&AppUser::Admin.pubkey()),
    )?;

    Ok(())
}
