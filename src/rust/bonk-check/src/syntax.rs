use bonk_ast::SourceSpan;
use bonk_ast_errorless::Date;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
pub struct SyntaxErrors(pub Vec<SourceSpan>);

pub fn check_syntax(
    ledger: &bonk_ast::Ledger,
    src: &str,
) -> Result<bonk_ast_errorless::Ledger, SyntaxErrors> {
    let errors = ledger.errors();
    if errors.is_empty() {
        convert_ledger(ledger, src)
    } else {
        Err(SyntaxErrors(errors))
    }
}

fn convert_ledger(
    ledger: &bonk_ast::Ledger,
    src: &str,
) -> Result<bonk_ast_errorless::Ledger, SyntaxErrors> {
    let (transactions, errors): (Vec<_>, Vec<_>) = ledger
        .transactions()
        .into_iter()
        .map(|t| convert_transaction(t, src))
        .partition_result();

    if errors.is_empty() {
        Ok(bonk_ast_errorless::Ledger {
            transactions,
            source_span: Some(ledger.span()),
        })
    } else {
        Err(SyntaxErrors(
            errors.into_iter().flat_map(|e| e.0).collect_vec(),
        ))
    }
}

fn convert_transaction(
    transaction: bonk_ast::Transaction,
    src: &str,
) -> Result<bonk_ast_errorless::Transaction, SyntaxErrors> {
    let date = transaction
        .date()
        .and_then(|date| Date::parse(date.value(src), None))
        .ok_or(SyntaxErrors(vec![transaction.span()]));

    let description = transaction
        .description(src)
        .ok_or(SyntaxErrors(vec![transaction.span()]));

    let (postings, errors): (Vec<_>, Vec<_>) = transaction
        .postings()
        .into_iter()
        .map(|p| convert_posting(p, src))
        .partition_result();

    let mut errors = errors.into_iter().flat_map(|e| e.0).collect_vec();

    match (date, description) {
        (Ok(date), Ok(description)) => {
            return if errors.is_empty() {
                Ok(bonk_ast_errorless::Transaction {
                    date,
                    description: description.to_string(),
                    postings,
                    source_span: Some(transaction.span()),
                })
            } else {
                Err(SyntaxErrors(errors))
            }
        }
        (Ok(_), Err(err)) => errors.extend(err.0),
        (Err(err), Ok(_)) => errors.extend(err.0),
        (Err(err_a), Err(err_b)) => {
            errors.extend(err_a.0);
            errors.extend(err_b.0);
        }
    };

    Err(SyntaxErrors(errors))
}

fn convert_posting(
    posting: bonk_ast::Posting,
    src: &str,
) -> Result<bonk_ast_errorless::Posting, SyntaxErrors> {
    let account = posting
        .account()
        .ok_or(SyntaxErrors(vec![posting.span()]))
        .map(|acc| convert_account(acc, src));

    let amount = posting
        .amount()
        .ok_or(SyntaxErrors(vec![posting.span()]))
        .and_then(|amt| convert_amount(amt, src));

    let mut errors = Vec::new();

    match (account, amount) {
        (Ok(account), Ok(amount)) => {
            return Ok(bonk_ast_errorless::Posting {
                account,
                amount,
                source_span: Some(posting.span()),
            })
        }
        (Ok(_), Err(err)) => errors.extend(err.0),
        (Err(err), Ok(_)) => errors.extend(err.0),
        (Err(err_a), Err(err_b)) => {
            errors.extend(err_a.0);
            errors.extend(err_b.0);
        }
    };

    Err(SyntaxErrors(errors))
}

fn convert_account(account: bonk_ast::Account, src: &str) -> bonk_ast_errorless::Account {
    bonk_ast_errorless::Account {
        path: account
            .value(src)
            .split(':')
            .map(|s| s.to_string())
            .collect_vec(),
        source_span: Some(account.span()),
    }
}

fn convert_amount(
    amount: bonk_ast::Amount,
    src: &str,
) -> Result<bonk_ast_errorless::Amount, SyntaxErrors> {
    Ok(bonk_ast_errorless::Amount {
        cents: amount
            .value(src)
            .replace('.', "")
            .parse()
            .map_err(|_| SyntaxErrors(vec![amount.span()]))?,
        source_span: Some(amount.span()),
    })
}

#[cfg(test)]
mod tests {
    use bonk_ast::SourceSpan;
    use bonk_ast_errorless::{Account, Amount, Date, Ledger, Posting, Transaction};

    use crate::check_syntax;

    #[test]
    fn test_no_errors() {
        let src = r#"2023-01-01 "Mcdonald's"
    expenses:fast_food         10.91
    liabilities:my_credit_card -10.91"#;

        let ledger = bonk_ast::Parser::new().parse(src, None);
        let ledger = check_syntax(&ledger, src);

        assert_eq!(
            ledger,
            Ok(Ledger {
                transactions: vec![Transaction {
                    date: Date::parse("2023-01-01", None,).unwrap(),
                    description: "\"Mcdonald's\"".to_string(),
                    postings: vec![
                        Posting {
                            account: Account::parse(
                                "expenses:fast_food",
                                Some(SourceSpan {
                                    start_byte: 28,
                                    end_byte: 46,
                                    start_row: 1,
                                    start_col: 4,
                                    end_row: 1,
                                    end_col: 22,
                                })
                            ),
                            amount: Amount::from_dollars(
                                10.91,
                                Some(SourceSpan {
                                    start_byte: 55,
                                    end_byte: 60,
                                    start_row: 1,
                                    start_col: 31,
                                    end_row: 1,
                                    end_col: 36,
                                })
                            ),
                            source_span: Some(SourceSpan {
                                start_byte: 28,
                                end_byte: 60,
                                start_row: 1,
                                start_col: 4,
                                end_row: 1,
                                end_col: 36,
                            })
                        },
                        Posting {
                            account: Account::parse(
                                "liabilities:my_credit_card",
                                Some(SourceSpan {
                                    start_byte: 65,
                                    end_byte: 91,
                                    start_row: 2,
                                    start_col: 4,
                                    end_row: 2,
                                    end_col: 30,
                                })
                            ),
                            amount: Amount::from_dollars(
                                -10.91,
                                Some(SourceSpan {
                                    start_byte: 92,
                                    end_byte: 98,
                                    start_row: 2,
                                    start_col: 31,
                                    end_row: 2,
                                    end_col: 37,
                                })
                            ),
                            source_span: Some(SourceSpan {
                                start_byte: 65,
                                end_byte: 98,
                                start_row: 2,
                                start_col: 4,
                                end_row: 2,
                                end_col: 37,
                            })
                        }
                    ],
                    source_span: Some(SourceSpan {
                        start_byte: 0,
                        end_byte: 98,
                        start_row: 0,
                        start_col: 0,
                        end_row: 2,
                        end_col: 37,
                    })
                }],
                source_span: Some(SourceSpan {
                    start_byte: 0,
                    end_byte: 98,
                    start_row: 0,
                    start_col: 0,
                    end_row: 2,
                    end_col: 37,
                })
            })
        )
    }

    #[test]
    fn test_error() {
        let src = r#"2023-01-01abc "Mcdonald's"
expenses:fast_food         10.91
liabilities:my_credit_card -10.91"#;

        let ledger = bonk_ast::Parser::new().parse(src, None);
        let ledger = check_syntax(&ledger, src);

        let errors = ledger.err().unwrap().0;
        assert_eq!(
            errors,
            vec![SourceSpan {
                start_byte: 10,
                end_byte: 13,
                start_row: 0,
                start_col: 10,
                end_row: 0,
                end_col: 13,
            }],
        )
    }
}
