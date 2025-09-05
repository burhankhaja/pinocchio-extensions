use crate::helpers::suite::{core::App, types::TestResult};

#[test]
fn init_default() -> TestResult<()> {
    let _app = App::new(false);

    Ok(())
}
