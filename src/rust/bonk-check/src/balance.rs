use crate::{CheckError, CheckErrorCode};

// TODO: if/when we add inference for posting amounts (i.e., allowing one posting to have an implicit amount), there should be another ast type to represent that and this should go from bonk_ast_errorless::Ledger to that one instead
pub fn check_balance(ledger: &bonk_ast_errorless::Ledger) -> Result<(), Vec<CheckError>> {
    let mut errors = vec![];

    'outer: for transaction in &ledger.transactions {
        let mut sum = 0;
        for posting in &transaction.postings {
            let Some(amount) = &posting.amount else {
                // if a transaction contains a posting with an inferred amount, it always balances, because the inferred amount is whatever's left over
                // TODO: we separately check that every transaction has at most one posting with an inferred amount
                continue 'outer;
            };

            sum += amount.cents;
        }

        // let sum: i32 = transaction.postings.iter().map(|p| p.amount.cents).sum();
        if sum != 0 {
            errors.push(CheckError {
                code: CheckErrorCode::NoBalance,
                source: transaction
                    .source
                    .clone()
                    .expect("ast passed to check_balance isn't annotated with source spans"), // TODO: I really want to encode this in the types
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

    use super::check_balance;

    #[test]
    fn test_no_errors() {
        // Note that we can get away with passing source: None because we expect that there are no errors
        let ledger = Ledger {
            declare_accounts: vec![],
            transactions: vec![Transaction {
                date: Date::parse("2023-01-01", None).unwrap(),
                description: "\"Mcdonald's\"".to_string(),
                postings: vec![
                    Posting {
                        account: Account::parse("expenses/fast_food", None),
                        amount: Some(Amount::from_dollars(10.91, None)),
                        source: None,
                    },
                    Posting {
                        account: Account::parse("liabilities/my_credit_card", None),
                        amount: Some(Amount::from_dollars(-10.91, None)),
                        source: None,
                    },
                ],
                source: None,
            }],
            source: None,
        };

        assert!(check_balance(&ledger).is_ok())
    }

    #[test]
    fn test_no_errors_inferred_amount() {
        // Note that we can get away with passing source: None because we expect that there are no errors
        let ledger = Ledger {
            declare_accounts: vec![],
            transactions: vec![Transaction {
                date: Date::parse("2023-01-01", None).unwrap(),
                description: "\"Mcdonald's\"".to_string(),
                postings: vec![
                    Posting {
                        account: Account::parse("expenses/fast_food", None),
                        amount: Some(Amount::from_dollars(10.91, None)),
                        source: None,
                    },
                    Posting {
                        account: Account::parse("liabilities/my_credit_card", None),
                        amount: None,
                        source: None,
                    },
                ],
                source: None,
            }],
            source: None,
        };

        assert!(check_balance(&ledger).is_ok())
    }

    #[test]
    fn test_error() {
        let ledger = Ledger {
            declare_accounts: vec![],
            transactions: vec![Transaction {
                date: Date::parse("2023-01-01", None).unwrap(),
                description: "\"Mcdonald's\"".to_string(),
                postings: vec![Posting {
                    account: Account::parse("expenses/fast_food", None),
                    amount: Some(Amount::from_dollars(10.91, None)),
                    source: None,
                }],
                // only supply a (fake) source here since it's the only error loc
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

        let checked_ledger = check_balance(&ledger);

        insta::assert_debug_snapshot!(checked_ledger, @r###"
        Err(
            [
                CheckError {
                    code: NoBalance,
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
