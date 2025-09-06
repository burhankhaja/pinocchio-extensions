use crate::helpers::{
    extensions::registry::TokenExtension,
    suite::{
        core::App,
        types::{AppUser, PinPubkey, TestResult},
    },
};

#[test]
fn init_default() -> TestResult<()> {
    let mut app = App::new(true);

    let mint = &pinocchio_pubkey::from_str("9qsaqMY2DKDdhtmAzCtHBnCuahCPBJuWFJx5wQDWdJeS");

    app.token_try_initialize_mint(AppUser::Admin, mint, 6, &AppUser::Admin.pubkey(), None)?;

    Ok(())
}
