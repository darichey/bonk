use crate::{CheckError, CheckErrorCode};

pub fn check_single_infer(ledger: &bonk_ast_errorless::Ledger) -> Result<(), Vec<CheckError>> {
    let mut errors = vec![];

    for transaction in &ledger.transactions {
        let num_inferred = transaction
            .postings
            .iter()
            .filter(|p| p.amount.is_none())
            .count();

        if num_inferred > 1 {
            errors.push(CheckError {
                code: CheckErrorCode::MultipleInfers,
                source: transaction.source.clone().expect(
                    "ast passed to check_account_refs isn't annotated with source spans", // TODO: I really want to encode this in the types
                ),
            })
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

    use crate::single_infer::check_single_infer;

    #[test]
    fn test_no_errors_none_inferred() {
        // Note that we can get away with passing source: None because we expect that there are no errors
        let ledger = Ledger {
            declare_accounts: vec![
                DeclareAccount {
                    account: Account::parse("foo", None),
                    metadata: Metadata::new(),
                    source: None,
                },
                DeclareAccount {
                    account: Account::parse("bar", None),
                    metadata: Metadata::new(),
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

        assert!(check_single_infer(&ledger).is_ok());
    }

    #[test]
    fn test_no_errors_one_inferred() {
        // Note that we can get away with passing source: None because we expect that there are no errors
        let ledger = Ledger {
            declare_accounts: vec![
                DeclareAccount {
                    account: Account::parse("foo", None),
                    metadata: Metadata::new(),
                    source: None,
                },
                DeclareAccount {
                    account: Account::parse("bar", None),
                    metadata: Metadata::new(),
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
                        amount: None,
                        source: None,
                    },
                ],
                source: None,
            }],
            source: None,
        };

        assert!(check_single_infer(&ledger).is_ok());
    }

    #[test]
    fn test_error() {
        let ledger = Ledger {
            declare_accounts: vec![
                DeclareAccount {
                    account: Account::parse("foo", None),
                    metadata: Metadata::new(),
                    source: None,
                },
                DeclareAccount {
                    account: Account::parse("bar", None),
                    metadata: Metadata::new(),
                    source: None,
                },
            ],
            transactions: vec![Transaction {
                date: Date::parse("2023-01-01", None).unwrap(),
                description: "baz".to_string(),
                postings: vec![
                    Posting {
                        account: Account::parse("foo", None),
                        amount: None,
                        source: None,
                    },
                    Posting {
                        account: Account::parse("bar", None),
                        amount: None,
                        source: None,
                    },
                ],
                source: Some(Source {
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
            }],
            source: None,
        };

        let checked_ledger = check_single_infer(&ledger);

        insta::assert_debug_snapshot!(checked_ledger, @r###"
        Err(
            [
                CheckError {
                    code: MultipleInfers,
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
