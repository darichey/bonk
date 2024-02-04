use std::collections::HashSet;

use bonk_ast::SourceSpan;

#[derive(Debug, PartialEq, Eq)]
pub struct AccountRefError(pub SourceSpan);

pub fn check_account_refs(
    ledger: bonk_ast_errorless::Ledger,
) -> Result<bonk_ast_errorless::Ledger, Vec<AccountRefError>> {
    let mut errors = vec![];

    let declared_accounts = ledger
        .declare_accounts
        .iter()
        .map(|a| a.account.path_string())
        .collect::<HashSet<String>>();

    for transaction in &ledger.transactions {
        for posting in &transaction.postings {
            if !declared_accounts.contains(&posting.account.path_string()) {
                errors.push(AccountRefError(posting.account.source_span.expect(
                    "ast passed to check_account_refs isn't annotated with source spans", // TODO: I really want to encode this in the types
                )))
            }
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

    use crate::account_ref::check_account_refs;

    #[test]
    fn test_no_errors() {
        // Note that we can get away with passing source_span: None because we expect that there are no errors
        let ledger = Ledger {
            declare_accounts: vec![
                DeclareAccount {
                    account: Account::parse("foo", None),
                    source_span: None,
                },
                DeclareAccount {
                    account: Account::parse("bar", None),
                    source_span: None,
                },
            ],
            transactions: vec![Transaction {
                date: Date::parse("2023-01-01", None).unwrap(),
                description: "baz".to_string(),
                postings: vec![
                    Posting {
                        account: Account::parse("foo", None),
                        amount: Amount::from_dollars(10.0, None),
                        source_span: None,
                    },
                    Posting {
                        account: Account::parse("bar", None),
                        amount: Amount::from_dollars(-10.0, None),
                        source_span: None,
                    },
                ],
                source_span: None,
            }],
            source_span: None,
        };

        let checked_ledger = check_account_refs(ledger.clone());

        assert_eq!(checked_ledger, Ok(ledger));
    }

    #[test]
    fn test_error() {
        let ledger = Ledger {
            declare_accounts: vec![
                DeclareAccount {
                    account: Account::parse("foo", None),
                    source_span: None,
                },
                DeclareAccount {
                    account: Account::parse("bar", None),
                    source_span: None,
                },
            ],
            transactions: vec![Transaction {
                date: Date::parse("2023-01-01", None).unwrap(),
                description: "baz".to_string(),
                postings: vec![
                    Posting {
                        account: Account::parse("foo", None),
                        amount: Amount::from_dollars(10.0, None),
                        source_span: None,
                    },
                    Posting {
                        account: Account::parse(
                            "baz",
                            // only supply a (fake) span here since it's the only error loc
                            Some(SourceSpan {
                                start_byte: 0,
                                end_byte: 1,
                                start_row: 2,
                                start_col: 3,
                                end_row: 4,
                                end_col: 5,
                            }),
                        ),
                        amount: Amount::from_dollars(-10.0, None),
                        source_span: None,
                    },
                ],
                source_span: None,
            }],
            source_span: None,
        };

        let checked_ledger = check_account_refs(ledger.clone());

        insta::assert_debug_snapshot!(checked_ledger, @r###"
        Err(
            [
                AccountRefError(
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
