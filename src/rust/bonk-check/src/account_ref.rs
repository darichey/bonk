use std::collections::HashSet;

use crate::{CheckError, CheckErrorCode};

pub fn check_account_refs(ledger: &bonk_ast_errorless::Ledger) -> Result<(), Vec<CheckError>> {
    let mut errors = vec![];

    let declared_accounts = ledger
        .declare_accounts
        .iter()
        .map(|a| a.account.path_string())
        .collect::<HashSet<String>>();

    for transaction in &ledger.transactions {
        for posting in &transaction.postings {
            if !declared_accounts.contains(&posting.account.path_string()) {
                errors.push(CheckError {
                    code: CheckErrorCode::UnknownAccount,
                    source: posting.account.source.clone().expect(
                        "ast passed to check_account_refs isn't annotated with source spans", // TODO: I really want to encode this in the types
                    ),
                })
            }
        }
    }

    if !errors.is_empty() {
        Err(errors)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use bonk_ast_errorless::*;
    use bonk_parse::ast::{Source, SourceSpan};

    use crate::account_ref::check_account_refs;

    #[test]
    fn test_no_errors() {
        // Note that we can get away with passing source: None because we expect that there are no errors
        let ledger = Ledger {
            imports: vec![Import {
                path: "./foo.bonk".to_string(),
                source: None,
            }],
            declare_accounts: vec![
                DeclareAccount {
                    account: Account::parse("foo", None),
                    source: None,
                },
                DeclareAccount {
                    account: Account::parse("bar", None),
                    source: None,
                },
            ],
            transactions: vec![Transaction {
                date: Date::parse("2023-01-01", None).unwrap(),
                description: "baz".to_string(),
                postings: vec![
                    Posting {
                        account: Account::parse("foo", None),
                        amount: Amount::from_dollars(10.0, None),
                        source: None,
                    },
                    Posting {
                        account: Account::parse("bar", None),
                        amount: Amount::from_dollars(-10.0, None),
                        source: None,
                    },
                    // Posting { // TODO: resolve imports to make this work
                    //     account: Account::parse("baz", None),
                    //     amount: Amount::from_dollars(-5.0, None),
                    //     source: None,
                    // },
                ],
                source: None,
            }],
            source: None,
        };

        assert!(check_account_refs(&ledger).is_ok())
    }

    #[test]
    fn test_error() {
        let ledger = Ledger {
            imports: vec![],
            declare_accounts: vec![
                DeclareAccount {
                    account: Account::parse("foo", None),
                    source: None,
                },
                DeclareAccount {
                    account: Account::parse("bar", None),
                    source: None,
                },
            ],
            transactions: vec![Transaction {
                date: Date::parse("2023-01-01", None).unwrap(),
                description: "baz".to_string(),
                postings: vec![
                    Posting {
                        account: Account::parse("foo", None),
                        amount: Amount::from_dollars(10.0, None),
                        source: None,
                    },
                    Posting {
                        account: Account::parse(
                            "baz",
                            // only supply a (fake) source here since it's the only error loc
                            Some(Source {
                                path: Some(PathBuf::from("ledger.bonk")),
                                span: SourceSpan {
                                    start_byte: 0,
                                    end_byte: 1,
                                    start_row: 2,
                                    start_col: 3,
                                    end_row: 4,
                                    end_col: 5,
                                },
                            }),
                        ),
                        amount: Amount::from_dollars(-10.0, None),
                        source: None,
                    },
                ],
                source: None,
            }],
            source: None,
        };

        let checked_ledger = check_account_refs(&ledger);

        insta::assert_debug_snapshot!(checked_ledger, @r###"
        Err(
            [
                CheckError {
                    code: UnknownAccount,
                    source: Source {
                        path: Some(
                            "ledger.bonk",
                        ),
                        span: SourceSpan {
                            start_byte: 0,
                            end_byte: 1,
                            start_row: 2,
                            start_col: 3,
                            end_row: 4,
                            end_col: 5,
                        },
                    },
                },
            ],
        )
        "###);
    }
}
