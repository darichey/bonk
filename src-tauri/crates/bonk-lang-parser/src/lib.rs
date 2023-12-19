mod lexer;
mod tree;

use lalrpop_util::{lalrpop_mod, ParseError};
use tree::Ledger;

use self::lexer::Token;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    pub grammar
);

pub fn parse(src: &str) -> Result<Ledger, ParseError<usize, Token<'_>, ()>> {
    let lexer = lexer::Lexer::new(src)
        .spanned()
        .map(|(token, span)| token.map(|token| (span.start, token, span.end)));
    let parser = grammar::LedgerParser::new();
    parser.parse(src, lexer)
}

#[cfg(test)]
mod tests {
    use crate::{
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
            Ok(Ledger {
                transactions: vec![
                    Transaction {
                        date: "2023-01-01",
                        description: "\"Mcdonald's\"",
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
                        description: "\"Paying credit card\"",
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
            })
        );
    }
}
