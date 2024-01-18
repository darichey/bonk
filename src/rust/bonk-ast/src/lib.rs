use core::fmt;

use tree_sitter::{Node, Range, Tree};

pub struct Parser {
    ts_parser: tree_sitter::Parser,
}

impl Parser {
    pub fn new() -> Self {
        let mut ts_parser = tree_sitter::Parser::new();
        ts_parser
            .set_language(tree_sitter_bonk::language())
            .unwrap();
        Self { ts_parser }
    }

    pub fn parse<'a>(&mut self, src: &'a str, old_ledger: Option<&Ledger>) -> Ledger<'a> {
        let tree = self
            .ts_parser
            .parse(src, old_ledger.map(|old_ledger| &old_ledger.0))
            .unwrap();
        Ledger::new(tree, src)
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourceSpan(pub Range);

pub struct Ledger<'a>(Tree, &'a str);

impl<'a> Ledger<'a> {
    pub fn new(tree: Tree, src: &'a str) -> Self {
        Self(tree, src)
    }

    pub fn transactions(&self) -> Vec<Transaction<'_>> {
        let mut cursor = self.0.walk();
        self.0
            .root_node()
            .children_by_field_name("transaction", &mut cursor)
            .map(|n| Transaction(n, self.1))
            .collect()
    }

    pub fn errors(&self) -> Vec<SourceSpan> {
        let mut cursor = self.0.walk();

        let mut errors = vec![];
        let mut reached_root = false;

        while !reached_root {
            if cursor.node().is_error() {
                errors.push(SourceSpan(cursor.node().range()))
            }

            if cursor.goto_first_child() {
                continue;
            }

            if cursor.goto_next_sibling() {
                continue;
            }

            let mut retracting = true;
            while retracting {
                if !cursor.goto_parent() {
                    retracting = false;
                    reached_root = true;
                }

                if cursor.goto_next_sibling() {
                    retracting = false;
                }
            }
        }

        errors
    }
}

impl fmt::Debug for Ledger<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0.root_node().to_sexp())
    }
}

pub struct Transaction<'a>(Node<'a>, &'a str);

impl Transaction<'_> {
    pub fn date(&self) -> Option<&str> {
        self.0.child_by_field_name("date").map(|n| {
            n.utf8_text(self.1.as_bytes())
                .expect("src is not valid utf-8")
        })
    }

    pub fn description(&self) -> Option<&str> {
        self.0.child_by_field_name("description").map(|n| {
            n.utf8_text(self.1.as_bytes())
                .expect("src is not valid utf-8")
        })
    }

    pub fn postings(&self) -> Vec<Posting<'_>> {
        let mut cursor = self.0.walk();
        self.0
            .children_by_field_name("posting", &mut cursor)
            .map(|n| Posting(n, self.1))
            .collect()
    }
}

pub struct Posting<'a>(Node<'a>, &'a str);

impl Posting<'_> {
    pub fn account(&self) -> Option<Account> {
        self.0
            .child_by_field_name("account")
            .map(|n| Account(n, self.1))
    }

    pub fn amount(&self) -> Option<Amount> {
        self.0
            .child_by_field_name("amount")
            .map(|n| Amount(n, self.1))
    }
}

pub struct Account<'a>(Node<'a>, &'a str);

impl Account<'_> {
    pub fn value(&self) -> &str {
        self.0
            .utf8_text(self.1.as_bytes())
            .expect("src is not valid utf-8")
    }
}

pub struct Amount<'a>(Node<'a>, &'a str);

impl Amount<'_> {
    pub fn value(&self) -> &str {
        self.0
            .utf8_text(self.1.as_bytes())
            .expect("src is not valid utf-8")
    }
}

#[cfg(test)]
mod tests {
    use crate::Parser;

    #[test]
    fn test() {
        let src = r#"2023-01-01 "Mcdonald's"
  expenses:fast_food         10.91
  liabilities:my_credit_card
      
2023-01-02 "Paying credit card"
  liabilities:my_credit_card    10.91
  assets:my_checking"#;

        let ledger = Parser::new().parse(src, None);

        let transactions = ledger.transactions();
        assert_eq!(transactions.len(), 2);

        let transaction = transactions.get(0).unwrap();
        assert_eq!(transaction.date(), Some("2023-01-01"));
        assert_eq!(transaction.description(), Some("\"Mcdonald's\""));

        let postings = transaction.postings();
        let posting = postings.get(0).unwrap();
        assert_eq!(posting.account().unwrap().value(), "expenses:fast_food");
        assert_eq!(posting.amount().unwrap().value(), "10.91");

        let posting = postings.get(1).unwrap();
        assert_eq!(
            posting.account().unwrap().value(),
            "liabilities:my_credit_card"
        );
        assert!(posting.amount().is_none());

        let transaction = transactions.get(1).unwrap();
        assert_eq!(transaction.date(), Some("2023-01-02"));
        assert_eq!(transaction.description(), Some("\"Paying credit card\""));

        let postings = transaction.postings();
        let posting = postings.get(0).unwrap();
        assert_eq!(
            posting.account().unwrap().value(),
            "liabilities:my_credit_card"
        );
        assert_eq!(posting.amount().unwrap().value(), "10.91");

        let posting = postings.get(1).unwrap();
        assert_eq!(posting.account().unwrap().value(), "assets:my_checking");
        assert!(posting.amount().is_none());
    }
}
