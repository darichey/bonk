use crate::{builtins, CheckError, CheckErrorCode};

pub fn check_todo(ledger: &bonk_ast_errorless::Ledger) -> Result<(), Vec<CheckError>> {
    let mut errors = vec![];

    for transaction in &ledger.transactions {
        for posting in &transaction.postings {
            if posting.account.path_string() == builtins::TODO_ACCOUNT {
                errors.push(CheckError {
                    code: CheckErrorCode::Todo,
                    source: posting.account.source.clone().expect(
                        "ast passed to check_todo isn't annotated with source spans", // TODO: I really want to encode this in the types
                    ),
                })
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
