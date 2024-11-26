use crate::{builtins, CheckError, CheckErrorCode};

pub fn check_declare_accounts(ledger: &bonk_ast_errorless::Ledger) -> Result<(), Vec<CheckError>> {
    let mut errors = vec![];

    let builtin_accounts = builtins::accounts();
    for declare_account in &ledger.declare_accounts {
        if builtin_accounts.contains(&declare_account.account.path_string()) {
            errors.push(CheckError {
                code: CheckErrorCode::BuiltinAccount,
                source: declare_account.source.clone().expect(
                    "ast passed to check_declare_accounts isn't annotated with source spans", // TODO: I really want to encode this in the types
                ),
            })
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
