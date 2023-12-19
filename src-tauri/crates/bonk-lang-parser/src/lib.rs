mod lexer;
mod tree;

use lalrpop_util::{lalrpop_mod, ErrorRecovery, ParseError};
use tree::Ledger;

use self::lexer::Token;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    pub grammar
);

pub fn parse(
    src: &str,
) -> Result<
    (Ledger, Vec<ErrorRecovery<usize, Token<'_>, &'static str>>),
    ParseError<usize, Token<'_>, &'static str>,
> {
    let lexer = lexer::Lexer::new(src).spanned().map(|(token, span)| {
        token
            .map(|token| (span.start, token, span.end))
            .map_err(|_| "lex error")
    });
    let parser = grammar::LedgerParser::new();

    let mut errors = Vec::new();
    let ledger = parser.parse(src, &mut errors, lexer)?;
    Ok((ledger, errors))
}

#[cfg(test)]
mod tests {
    use lalrpop_util::{ErrorRecovery, ParseError};

    use crate::{
        lexer::Token,
        parse,
        tree::{Ledger, Posting, Transaction},
    };

    #[test]
    fn parses() {
        let src = r#"2023-01-01 "Mcdonald's"
  expenses:fast_food         10.91
  liabilities:my_credit_card

2023-01-02 "Paying credit card"
  liabilities:my_credit_card    10.91
  assets:my_checking
"#;

        assert_eq!(
            parse(src),
            Ok((
                Ledger {
                    transactions: vec![
                        Transaction {
                            date: "2023-01-01",
                            description: Some("\"Mcdonald's\""),
                            postings: vec![
                                Posting {
                                    account: "expenses:fast_food",
                                    amount: Some("10.91"),
                                },
                                Posting {
                                    account: "liabilities:my_credit_card",
                                    amount: None
                                }
                            ]
                        },
                        Transaction {
                            date: "2023-01-02",
                            description: Some("\"Paying credit card\""),
                            postings: vec![
                                Posting {
                                    account: "liabilities:my_credit_card",
                                    amount: Some("10.91")
                                },
                                Posting {
                                    account: "assets:my_checking",
                                    amount: None
                                }
                            ]
                        }
                    ]
                },
                vec![]
            ))
        );
    }

    #[test]
    fn parse_recover_no_postings() {
        let src = r#"2023-01-01 "Mcdonald's"

2023-01-02 "Paying credit card"
  liabilities:my_credit_card    10.91
  assets:my_checking
"#;

        assert_eq!(
            parse(src),
            Ok((
                Ledger {
                    transactions: vec![
                        Transaction {
                            date: "2023-01-01",
                            description: Some("\"Mcdonald's\""),
                            postings: vec![]
                        },
                        Transaction {
                            date: "2023-01-02",
                            description: Some("\"Paying credit card\""),
                            postings: vec![
                                Posting {
                                    account: "liabilities:my_credit_card",
                                    amount: Some("10.91")
                                },
                                Posting {
                                    account: "assets:my_checking",
                                    amount: None
                                }
                            ]
                        }
                    ]
                },
                vec![ErrorRecovery {
                    error: ParseError::UnrecognizedToken {
                        token: (24, Token::NewLine, 25),
                        expected: vec!["\" \"".to_string()]
                    },
                    dropped_tokens: vec![]
                }]
            ))
        );
    }

    #[test]
    fn parse_recover_no_description() {
        let src = r#"2023-01-01
  expenses:fast_food         10.91
  liabilities:my_credit_card

2023-01-02 "Paying credit card"
  liabilities:my_credit_card    10.91
  assets:my_checking
"#;

        assert_eq!(
            parse(src),
            Ok((
                Ledger {
                    transactions: vec![
                        Transaction {
                            date: "2023-01-01",
                            description: None,
                            postings: vec![
                                Posting {
                                    account: "expenses:fast_food",
                                    amount: Some("10.91"),
                                },
                                Posting {
                                    account: "liabilities:my_credit_card",
                                    amount: None
                                }
                            ]
                        },
                        Transaction {
                            date: "2023-01-02",
                            description: Some("\"Paying credit card\""),
                            postings: vec![
                                Posting {
                                    account: "liabilities:my_credit_card",
                                    amount: Some("10.91")
                                },
                                Posting {
                                    account: "assets:my_checking",
                                    amount: None
                                }
                            ]
                        }
                    ]
                },
                vec![ErrorRecovery {
                    error: ParseError::UnrecognizedToken {
                        token: (10, Token::NewLine, 11),
                        expected: vec!["\" \"".to_string()]
                    },
                    dropped_tokens: vec![]
                }]
            ))
        );
    }
}
