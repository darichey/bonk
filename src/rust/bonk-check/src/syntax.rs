use std::path::Path;

use bonk_ast_errorless::{Date, Metadata};
use bonk_parse::ast::Source;
use itertools::Itertools;

use crate::{CheckError, CheckErrorCode};

pub fn check_syntax(
    ledger: &bonk_parse::ast::Ledger,
    src: &str,
    path: Option<&Path>,
) -> Result<bonk_ast_errorless::Ledger, Vec<CheckError>> {
    let errors = ledger.errors();
    if errors.is_empty() {
        convert_ledger(ledger, src, path).map_err(|sources| {
            sources
                .into_iter()
                .map(|source| CheckError {
                    code: CheckErrorCode::SyntaxError,
                    source,
                })
                .collect()
        })
    } else {
        Err(errors
            .into_iter()
            .map(|span| CheckError {
                code: CheckErrorCode::SyntaxError,
                source: Source {
                    path: path.map(|p| p.to_path_buf()),
                    span,
                },
            })
            .collect())
    }
}

fn convert_ledger(
    ledger: &bonk_parse::ast::Ledger,
    src: &str,
    path: Option<&Path>,
) -> Result<bonk_ast_errorless::Ledger, Vec<Source>> {
    let (declare_accounts, errors_declare_accounts): (Vec<_>, Vec<_>) = ledger
        .declare_accounts()
        .into_iter()
        .map(|d| convert_declared_account(d, src, path))
        .partition_result();

    let (transactions, errors_transactions): (Vec<_>, Vec<_>) = ledger
        .transactions()
        .into_iter()
        .map(|t| convert_transaction(t, src, path))
        .partition_result();

    let errors: Vec<_> = errors_declare_accounts
        .into_iter()
        .chain(errors_transactions)
        .collect();

    if errors.is_empty() {
        Ok(bonk_ast_errorless::Ledger {
            declare_accounts,
            transactions,
            source: Some(Source {
                path: path.map(Path::to_path_buf),
                span: ledger.span(),
            }),
        })
    } else {
        Err(errors.into_iter().flatten().collect())
    }
}

fn convert_transaction(
    transaction: bonk_parse::ast::Transaction,
    src: &str,
    path: Option<&Path>,
) -> Result<bonk_ast_errorless::Transaction, Vec<Source>> {
    let date = transaction
        .date()
        .and_then(|date| {
            Date::parse(
                date.value(src),
                Some(Source {
                    path: path.map(Path::to_path_buf),
                    span: date.span(),
                }),
            )
        })
        .ok_or(vec![
            (Source {
                path: path.map(|p| p.to_path_buf()),
                span: transaction.span(),
            }),
        ]);

    let description = transaction.description(src).ok_or(vec![
        (Source {
            path: path.map(|p| p.to_path_buf()),
            span: transaction.span(),
        }),
    ]);

    let (postings, errors): (Vec<_>, Vec<_>) = transaction
        .postings()
        .into_iter()
        .map(|p| convert_posting(p, src, path))
        .partition_result();

    let mut errors = errors.into_iter().flatten().collect_vec();

    match (date, description) {
        (Ok(date), Ok(description)) => {
            return if errors.is_empty() {
                Ok(bonk_ast_errorless::Transaction {
                    date,
                    description: description.to_string(),
                    postings,
                    source: Some(Source {
                        path: path.map(Path::to_path_buf),
                        span: transaction.span(),
                    }),
                })
            } else {
                Err(errors)
            }
        }
        (Ok(_), Err(err)) => errors.extend(err),
        (Err(err), Ok(_)) => errors.extend(err),
        (Err(err_a), Err(err_b)) => {
            errors.extend(err_a);
            errors.extend(err_b);
        }
    };

    Err(errors)
}

fn convert_posting(
    posting: bonk_parse::ast::Posting,
    src: &str,
    path: Option<&Path>,
) -> Result<bonk_ast_errorless::Posting, Vec<Source>> {
    let account = posting
        .account()
        .ok_or(vec![
            (Source {
                path: path.map(|p| p.to_path_buf()),
                span: posting.span(),
            }),
        ])
        .map(|acc| convert_account(acc, src, path));

    let amount = posting
        .amount()
        .map(|amt| convert_amount(amt, src, path))
        .transpose();

    let mut errors = Vec::new();

    match (account, amount) {
        (Ok(account), Ok(amount)) => {
            return Ok(bonk_ast_errorless::Posting {
                account,
                amount,
                source: Some(Source {
                    path: path.map(Path::to_path_buf),
                    span: posting.span(),
                }),
            })
        }
        (Ok(_), Err(err)) => errors.extend(err),
        (Err(err), Ok(_)) => errors.extend(err),
        (Err(err_a), Err(err_b)) => {
            errors.extend(err_a);
            errors.extend(err_b);
        }
    };

    Err(errors)
}

fn convert_account(
    account: bonk_parse::ast::Account,
    src: &str,
    path: Option<&Path>,
) -> bonk_ast_errorless::Account {
    bonk_ast_errorless::Account::parse(
        account.value(src),
        Some(Source {
            path: path.map(Path::to_path_buf),
            span: account.span(),
        }),
    )
}

fn convert_amount(
    amount: bonk_parse::ast::Amount,
    src: &str,
    path: Option<&Path>,
) -> Result<bonk_ast_errorless::Amount, Vec<Source>> {
    let source = Source {
        path: path.map(Path::to_path_buf),
        span: amount.span(),
    };

    bonk_ast_errorless::Amount::parse(amount.value(src), Some(source.clone()))
        .map_err(|_| vec![source])
}

fn convert_declared_account(
    account: bonk_parse::ast::DeclareAccount,
    src: &str,
    path: Option<&Path>,
) -> Result<bonk_ast_errorless::DeclareAccount, Vec<Source>> {
    Ok(bonk_ast_errorless::DeclareAccount {
        account: account
            .account()
            .ok_or(vec![
                (Source {
                    path: path.map(|p| p.to_path_buf()),
                    span: account.span(),
                }),
            ])
            .map(|a| convert_account(a, src, path))?,
        metadata: {
            Metadata {
                entries: account
                    .metadata()
                    .into_iter()
                    .filter_map(|m| {
                        m.key().and_then(|k| {
                            m.value()
                                .map(|v| (k.value(src).to_string(), v.value(src).to_string()))
                        })
                    })
                    .collect(),
            }
        },
        source: Some(Source {
            path: path.map(Path::to_path_buf),
            span: account.span(),
        }),
    })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::syntax::check_syntax;

    #[test]
    fn test_no_errors() {
        let path = PathBuf::from("ledger.bonk");
        let src = r#"account foo

2023-01-01 "Mcdonald's"
    expenses/fast_food         10.91
    liabilities/my_credit_card -10.91"#;

        let ledger = bonk_parse::Parser::new().parse(src, None);
        let ledger = check_syntax(&ledger, src, Some(&path));

        insta::assert_debug_snapshot!(ledger, @r###"
        Ok(
            Ledger {
                declare_accounts: [
                    DeclareAccount {
                        account: Account {
                            path: [
                                "foo",
                            ],
                            source: Some(
                                Source {
                                    path: Some(
                                        "ledger.bonk",
                                    ),
                                    span: SourceSpan {
                                        start_byte: 8,
                                        end_byte: 11,
                                        start_row: 0,
                                        start_col: 8,
                                        end_row: 0,
                                        end_col: 11,
                                    },
                                },
                            ),
                        },
                        metadata: Metadata {
                            entries: {},
                        },
                        source: Some(
                            Source {
                                path: Some(
                                    "ledger.bonk",
                                ),
                                span: SourceSpan {
                                    start_byte: 0,
                                    end_byte: 11,
                                    start_row: 0,
                                    start_col: 0,
                                    end_row: 0,
                                    end_col: 11,
                                },
                            },
                        ),
                    },
                ],
                transactions: [
                    Transaction {
                        date: Date {
                            year: 2023,
                            month: 1,
                            day: 1,
                            source: Some(
                                Source {
                                    path: Some(
                                        "ledger.bonk",
                                    ),
                                    span: SourceSpan {
                                        start_byte: 13,
                                        end_byte: 23,
                                        start_row: 2,
                                        start_col: 0,
                                        end_row: 2,
                                        end_col: 10,
                                    },
                                },
                            ),
                        },
                        description: "\"Mcdonald's\"",
                        postings: [
                            Posting {
                                account: Account {
                                    path: [
                                        "expenses",
                                        "fast_food",
                                    ],
                                    source: Some(
                                        Source {
                                            path: Some(
                                                "ledger.bonk",
                                            ),
                                            span: SourceSpan {
                                                start_byte: 41,
                                                end_byte: 59,
                                                start_row: 3,
                                                start_col: 4,
                                                end_row: 3,
                                                end_col: 22,
                                            },
                                        },
                                    ),
                                },
                                amount: Some(
                                    Amount {
                                        cents: 1091,
                                        source: Some(
                                            Source {
                                                path: Some(
                                                    "ledger.bonk",
                                                ),
                                                span: SourceSpan {
                                                    start_byte: 68,
                                                    end_byte: 73,
                                                    start_row: 3,
                                                    start_col: 31,
                                                    end_row: 3,
                                                    end_col: 36,
                                                },
                                            },
                                        ),
                                    },
                                ),
                                source: Some(
                                    Source {
                                        path: Some(
                                            "ledger.bonk",
                                        ),
                                        span: SourceSpan {
                                            start_byte: 41,
                                            end_byte: 73,
                                            start_row: 3,
                                            start_col: 4,
                                            end_row: 3,
                                            end_col: 36,
                                        },
                                    },
                                ),
                            },
                            Posting {
                                account: Account {
                                    path: [
                                        "liabilities",
                                        "my_credit_card",
                                    ],
                                    source: Some(
                                        Source {
                                            path: Some(
                                                "ledger.bonk",
                                            ),
                                            span: SourceSpan {
                                                start_byte: 78,
                                                end_byte: 104,
                                                start_row: 4,
                                                start_col: 4,
                                                end_row: 4,
                                                end_col: 30,
                                            },
                                        },
                                    ),
                                },
                                amount: Some(
                                    Amount {
                                        cents: -1091,
                                        source: Some(
                                            Source {
                                                path: Some(
                                                    "ledger.bonk",
                                                ),
                                                span: SourceSpan {
                                                    start_byte: 105,
                                                    end_byte: 111,
                                                    start_row: 4,
                                                    start_col: 31,
                                                    end_row: 4,
                                                    end_col: 37,
                                                },
                                            },
                                        ),
                                    },
                                ),
                                source: Some(
                                    Source {
                                        path: Some(
                                            "ledger.bonk",
                                        ),
                                        span: SourceSpan {
                                            start_byte: 78,
                                            end_byte: 111,
                                            start_row: 4,
                                            start_col: 4,
                                            end_row: 4,
                                            end_col: 37,
                                        },
                                    },
                                ),
                            },
                        ],
                        source: Some(
                            Source {
                                path: Some(
                                    "ledger.bonk",
                                ),
                                span: SourceSpan {
                                    start_byte: 13,
                                    end_byte: 111,
                                    start_row: 2,
                                    start_col: 0,
                                    end_row: 4,
                                    end_col: 37,
                                },
                            },
                        ),
                    },
                ],
                source: Some(
                    Source {
                        path: Some(
                            "ledger.bonk",
                        ),
                        span: SourceSpan {
                            start_byte: 0,
                            end_byte: 111,
                            start_row: 0,
                            start_col: 0,
                            end_row: 4,
                            end_col: 37,
                        },
                    },
                ),
            },
        )
        "###);
    }

    #[test]
    fn test_error() {
        let path = PathBuf::from("ledger.bonk");
        let src = r#"2023-01-01abc "Mcdonald's"
expenses/fast_food         10.91
liabilities/my_credit_card -10.91"#;

        let ledger = bonk_parse::Parser::new().parse(src, None);
        let ledger = check_syntax(&ledger, src, Some(&path));

        insta::assert_debug_snapshot!(ledger, @r###"
        Err(
            [
                CheckError {
                    code: SyntaxError,
                    source: Source {
                        path: Some(
                            "ledger.bonk",
                        ),
                        span: SourceSpan {
                            start_byte: 10,
                            end_byte: 13,
                            start_row: 0,
                            start_col: 10,
                            end_row: 0,
                            end_col: 13,
                        },
                    },
                },
            ],
        )
        "###);
    }
}
