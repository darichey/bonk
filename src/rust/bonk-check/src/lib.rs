mod account_ref;
mod balance;
mod syntax;

use std::path::Path;

pub use account_ref::AccountRefError;
pub use balance::BalanceError;
pub use syntax::SyntaxError;

#[derive(Debug, PartialEq, Eq)]
pub enum CheckError {
    AccountRefError(AccountRefError),
    BalanceError(BalanceError),
    SyntaxError(SyntaxError),
}

pub fn check(
    ledger: &bonk_ast::Ledger,
    src: &str,
    path: Option<&Path>,
) -> Result<bonk_ast_errorless::Ledger, Vec<CheckError>> {
    let ledger = syntax::check_syntax(ledger, src, path).map_err(|errs| {
        errs.into_iter()
            .map(CheckError::SyntaxError)
            .collect::<Vec<_>>()
    })?;

    let ledger = account_ref::check_account_refs(ledger).map_err(|errs| {
        errs.into_iter()
            .map(CheckError::AccountRefError)
            .collect::<Vec<_>>()
    })?;

    let ledger = balance::check_balance(ledger).map_err(|errs| {
        errs.into_iter()
            .map(CheckError::BalanceError)
            .collect::<Vec<_>>()
    })?;

    Ok(ledger)
}
