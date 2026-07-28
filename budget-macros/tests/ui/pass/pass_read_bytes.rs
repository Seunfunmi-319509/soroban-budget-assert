//! `#[budget_read_bytes_lt]` supports the same body shapes: unit, trailing
//! expression, and early return. It reports `resources().read_bytes` as its
//! read-bytes figure.

#[path = "../support/mock_env.rs"]
mod mock_env;

use budget_macros::budget_read_bytes_lt;
use mock_env::{budget_panic, Env};

#[derive(Debug, PartialEq)]
struct TestError;

#[budget_read_bytes_lt(4_096)]
fn unit_body() {
    let env = Env::new(0, 0).with_read_write(2_048, 0);
}

#[budget_read_bytes_lt(4_096)]
fn result_body() -> Result<u64, TestError> {
    let env = Env::new(0, 0).with_read_write(2_048, 0);
    Ok(env.cost_estimate().resources().read_bytes as u64)
}

#[budget_read_bytes_lt(4_096)]
fn early_return_body(exit_early: bool) -> Result<(), TestError> {
    let env = Env::new(0, 0).with_read_write(8_192, 0);
    if exit_early {
        return Ok(());
    }
    Ok(())
}

fn main() {
    unit_body();
    assert_eq!(result_body(), Ok(2_048));

    let message =
        budget_panic(|| early_return_body(true)).expect("the budget assertion should have failed");
    assert!(
        message.contains("Read bytes cost 8192 exceeded limit 4096"),
        "unexpected panic message: {message}"
    );
}
