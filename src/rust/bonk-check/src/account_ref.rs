use std::collections::HashSet;

use crate::{CheckError, CheckErrorCode, CheckUnit};

pub fn check_account_refs(
    ledger: &bonk_ast_errorless::Ledger,
    check_unit: &CheckUnit<bonk_ast_errorless::Ledger>,
) -> Result<(), Vec<CheckError>> {
    let mut errors = vec![];

    let declared_accounts = check_unit
        .ledgers()
        .flat_map(|(_, ledger)| {
            ledger
                .declare_accounts
                .iter()
                .map(|a| a.account.path_string())
        })
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

    use crate::{account_ref::check_account_refs, CheckUnit};

    #[test]
    fn test_no_errors() {
        // Note that we can get away with passing source: None because we expect that there are no errors
        let ledger = Ledger {
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
                        amount: Some(Amount::from_dollars(10.0, None)),
                        source: None,
                    },
                    Posting {
                        account: Account::parse("bar", None),
                        amount: Some(Amount::from_dollars(-10.0, None)),
                        source: None,
                    },
                ],
                source: None,
            }],
            source: None,
        };

        let path = PathBuf::from("ledger.bonk");

        let check_unit = CheckUnit::new(vec![(path.clone(), ledger.clone())]);

        assert!(check_account_refs(&ledger, &check_unit).is_ok())
    }

    #[test]
    fn test_error() {
        let ledger = Ledger {
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
                        amount: Some(Amount::from_dollars(10.0, None)),
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
                        amount: Some(Amount::from_dollars(-10.0, None)),
                        source: None,
                    },
                ],
                source: None,
            }],
            source: None,
        };

        let path = PathBuf::from("ledger.bonk");

        let check_unit = CheckUnit::new(vec![(path.clone(), ledger.clone())]);

        let checked_ledger = check_account_refs(&ledger, &check_unit);

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
