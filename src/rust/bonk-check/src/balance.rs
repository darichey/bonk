use bonk_ast::SourceSpan;

#[derive(Debug, PartialEq, Eq)]
pub struct BalanceError(pub SourceSpan);

// TODO: if/when we add inference for posting amounts (i.e., allowing one posting to have an implicit amount), there should be another ast type to represent that and this should go from bonk_ast_errorless::Ledger to that one instead
pub fn check_balance(
    ledger: bonk_ast_errorless::Ledger,
) -> Result<bonk_ast_errorless::Ledger, Vec<BalanceError>> {
    let mut errors = vec![];

    for transaction in &ledger.transactions {
        let sum: i32 = transaction.postings.iter().map(|p| p.amount.cents).sum();
        if sum != 0 {
            errors.push(BalanceError(
                transaction
                    .source_span
                    .expect("ast passed to check_balance isn't annotated with source spans"), // TODO: I really want to encode this in the types
            ))
        }
    }

    if !errors.is_empty() {
        Err(errors)
    } else {
        Ok(ledger)
    }
}

#[cfg(test)]
mod tests {
    use bonk_ast::SourceSpan;
    use bonk_ast_errorless::*;

    use super::check_balance;

    #[test]
    fn test_no_errors() {
        // Note that we can get away with passing source_span: None because we expect that there are no errors
        let ledger = Ledger {
            declare_accounts: vec![],
            transactions: vec![Transaction {
                date: Date::parse("2023-01-01", None).unwrap(),
                description: "\"Mcdonald's\"".to_string(),
                postings: vec![
                    Posting {
                        account: Account::parse("expenses:fast_food", None),
                        amount: Amount::from_dollars(10.91, None),
                        source_span: None,
                    },
                    Posting {
                        account: Account::parse("liabilities:my_credit_card", None),
                        amount: Amount::from_dollars(-10.91, None),
                        source_span: None,
                    },
                ],
                source_span: None,
            }],
            source_span: None,
        };

        let checked_ledger = check_balance(ledger.clone());

        assert_eq!(checked_ledger, Ok(ledger));
    }

    #[test]
    fn test_error() {
        let ledger = Ledger {
            declare_accounts: vec![],
            transactions: vec![Transaction {
                date: Date::parse("2023-01-01", None).unwrap(),
                description: "\"Mcdonald's\"".to_string(),
                postings: vec![Posting {
                    account: Account::parse("expenses:fast_food", None),
                    amount: Amount::from_dollars(10.91, None),
                    source_span: None,
                }],
                // only supply a (fake) span here since it's the only error loc
                source_span: Some(SourceSpan {
                    start_byte: 0,
                    end_byte: 1,
                    start_row: 2,
                    start_col: 3,
                    end_row: 4,
                    end_col: 5,
                }),
            }],
            source_span: None,
        };

        let checked_ledger = check_balance(ledger.clone());

        insta::assert_debug_snapshot!(checked_ledger, @r###"
        Err(
            [
                BalanceError(
                    SourceSpan {
                        start_byte: 0,
                        end_byte: 1,
                        start_row: 2,
                        start_col: 3,
                        end_row: 4,
                        end_col: 5,
                    },
                ),
            ],
        )
        "###);
    }
}
