use core::fmt;
use std::path::PathBuf;

use tree_sitter::{Node, Range, Tree};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourceSpan {
    pub start_byte: usize,
    pub end_byte: usize,
    pub start_row: usize,
    pub start_col: usize,
    pub end_row: usize,
    pub end_col: usize,
}

impl From<Range> for SourceSpan {
    fn from(value: Range) -> Self {
        Self {
            start_byte: value.start_byte,
            end_byte: value.end_byte,
            start_row: value.start_point.row,
            start_col: value.start_point.column,
            end_row: value.end_point.row,
            end_col: value.end_point.column,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Source {
    pub path: Option<PathBuf>,
    pub span: SourceSpan,
}

pub struct Ledger(pub Tree);

impl Ledger {
    pub fn new(tree: Tree) -> Self {
        Self(tree)
    }

    pub fn transactions(&self) -> Vec<Transaction<'_>> {
        let mut cursor = self.0.walk();
        self.0
            .root_node()
            .children_by_field_name("transaction", &mut cursor)
            .map(Transaction)
            .collect()
    }

    pub fn declare_accounts(&self) -> Vec<DeclareAccount<'_>> {
        let mut cursor = self.0.walk();
        self.0
            .root_node()
            .children_by_field_name("declare_account", &mut cursor)
            .map(DeclareAccount)
            .collect()
    }

    pub fn errors(&self) -> Vec<SourceSpan> {
        let mut cursor = self.0.walk();

        let mut errors = vec![];
        let mut reached_root = false;

        while !reached_root {
            if cursor.node().is_error() {
                errors.push(cursor.node().range().into())
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

    pub fn span(&self) -> SourceSpan {
        self.0.root_node().range().into()
    }
}

impl fmt::Debug for Ledger {
    // adapted from tree-sitter-cli: https://github.com/tree-sitter/tree-sitter/blob/660481dbf71413eba5a928b0b0ab8da50c1109e0/cli/src/parse.rs#L132-L185
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut cursor = self.0.walk();
        let mut needs_newline = false;
        let mut indent_level = 0;
        let mut did_visit_children = false;
        loop {
            let node = cursor.node();
            let is_named = node.is_named();
            if did_visit_children {
                if is_named {
                    f.write_str(")")?;
                    needs_newline = true;
                }
                if cursor.goto_next_sibling() {
                    did_visit_children = false;
                } else if cursor.goto_parent() {
                    did_visit_children = true;
                    indent_level -= 1;
                } else {
                    break;
                }
            } else {
                if is_named {
                    if needs_newline {
                        f.write_str("\n")?;
                    }
                    for _ in 0..indent_level {
                        f.write_str("  ")?;
                    }
                    let start = node.start_position();
                    let end = node.end_position();
                    if let Some(field_name) = cursor.field_name() {
                        write!(f, "{}: ", field_name)?;
                    }
                    write!(
                        f,
                        "({} [{}, {}] - [{}, {}]",
                        node.kind(),
                        start.row,
                        start.column,
                        end.row,
                        end.column
                    )?;
                    needs_newline = true;
                }
                if cursor.goto_first_child() {
                    did_visit_children = false;
                    indent_level += 1;
                } else {
                    did_visit_children = true;
                }
            }
        }
        f.write_str("\n")?;
        Ok(())
    }
}

pub struct Transaction<'a>(Node<'a>);

impl Transaction<'_> {
    pub fn date(&self) -> Option<Date> {
        self.0.child_by_field_name("date").map(Date)
    }

    pub fn description<'s>(&self, src: &'s str) -> Option<&'s str> {
        self.0
            .child_by_field_name("description")
            .map(|n| n.utf8_text(src.as_bytes()).expect("src is not valid utf-8"))
    }

    pub fn postings(&self) -> Vec<Posting<'_>> {
        let mut cursor = self.0.walk();
        self.0
            .children_by_field_name("posting", &mut cursor)
            .map(Posting)
            .collect()
    }

    pub fn span(&self) -> SourceSpan {
        self.0.range().into()
    }
}

pub struct Date<'a>(Node<'a>);

impl Date<'_> {
    pub fn value<'s>(&self, src: &'s str) -> &'s str {
        self.0
            .utf8_text(src.as_bytes())
            .expect("src is not valid utf-8")
    }

    pub fn span(&self) -> SourceSpan {
        self.0.range().into()
    }
}

pub struct Posting<'a>(Node<'a>);

impl Posting<'_> {
    pub fn account(&self) -> Option<Account> {
        self.0.child_by_field_name("account").map(Account)
    }

    pub fn amount(&self) -> Option<Amount> {
        self.0.child_by_field_name("amount").map(Amount)
    }

    pub fn span(&self) -> SourceSpan {
        self.0.range().into()
    }
}

pub struct Account<'a>(Node<'a>);

impl Account<'_> {
    pub fn value<'s>(&self, src: &'s str) -> &'s str {
        self.0
            .utf8_text(src.as_bytes())
            .expect("src is not valid utf-8")
    }

    pub fn span(&self) -> SourceSpan {
        self.0.range().into()
    }
}

pub struct Amount<'a>(Node<'a>);

impl Amount<'_> {
    pub fn value<'s>(&self, src: &'s str) -> &'s str {
        self.0
            .utf8_text(src.as_bytes())
            .expect("src is not valid utf-8")
    }

    pub fn span(&self) -> SourceSpan {
        self.0.range().into()
    }
}

pub struct DeclareAccount<'a>(Node<'a>);

impl DeclareAccount<'_> {
    pub fn account(&self) -> Option<Account> {
        self.0.child_by_field_name("account").map(Account)
    }

    pub fn metadata(&self) -> Vec<Metadata> {
        let mut cursor = self.0.walk();
        self.0
            .children_by_field_name("metadata", &mut cursor)
            .map(Metadata)
            .collect()
    }

    pub fn span(&self) -> SourceSpan {
        self.0.range().into()
    }
}

pub struct Metadata<'a>(Node<'a>);

impl Metadata<'_> {
    pub fn key(&self) -> Option<MetadataKey> {
        self.0.child_by_field_name("key").map(MetadataKey)
    }

    pub fn value(&self) -> Option<MetadataValue> {
        self.0.child_by_field_name("value").map(MetadataValue)
    }

    pub fn span(&self) -> SourceSpan {
        self.0.range().into()
    }
}

pub struct MetadataKey<'a>(Node<'a>);

impl MetadataKey<'_> {
    pub fn value<'s>(&self, src: &'s str) -> &'s str {
        self.0
            .utf8_text(src.as_bytes())
            .expect("src is not valid utf-8")
    }

    pub fn span(&self) -> SourceSpan {
        self.0.range().into()
    }
}

pub struct MetadataValue<'a>(Node<'a>);

impl MetadataValue<'_> {
    pub fn value<'s>(&self, src: &'s str) -> &'s str {
        self.0
            .utf8_text(src.as_bytes())
            .expect("src is not valid utf-8")
    }

    pub fn span(&self) -> SourceSpan {
        self.0.range().into()
    }
}

#[cfg(test)]
mod tests {
    use crate::Parser;

    #[test]
    fn test_debug_fmt() {
        let src = r#"account assets/my_checking
  starting_balance: 5.31
        
2023-01-01 "Mcdonald's"
  expenses/fast_food         10.91
  liabilities/my_credit_card -10.91
      
2023-01-02 "Paying credit card"
  liabilities/my_credit_card    10.91
  assets/my_checking           -10.91"#;

        let ledger = Parser::new().parse(src, None);

        insta::assert_debug_snapshot!(
            ledger,
            @r###"
        (ledger [0, 0] - [9, 37]
          declare_account: (declare_account [0, 0] - [1, 24]
            account: (account [0, 8] - [0, 26]
              (ident [0, 8] - [0, 14])
              (ident [0, 15] - [0, 26]))
            metadata: (metadata [1, 2] - [1, 24]
              key: (ident [1, 2] - [1, 18])
              value: (number [1, 20] - [1, 24])))
          transaction: (transaction [3, 0] - [5, 35]
            date: (date [3, 0] - [3, 10])
            description: (string [3, 11] - [3, 23])
            posting: (posting [4, 2] - [4, 34]
              account: (account [4, 2] - [4, 20]
                (ident [4, 2] - [4, 10])
                (ident [4, 11] - [4, 20]))
              amount: (number [4, 29] - [4, 34]))
            posting: (posting [5, 2] - [5, 35]
              account: (account [5, 2] - [5, 28]
                (ident [5, 2] - [5, 13])
                (ident [5, 14] - [5, 28]))
              amount: (number [5, 29] - [5, 35])))
          transaction: (transaction [7, 0] - [9, 37]
            date: (date [7, 0] - [7, 10])
            description: (string [7, 11] - [7, 31])
            posting: (posting [8, 2] - [8, 37]
              account: (account [8, 2] - [8, 28]
                (ident [8, 2] - [8, 13])
                (ident [8, 14] - [8, 28]))
              amount: (number [8, 32] - [8, 37]))
            posting: (posting [9, 2] - [9, 37]
              account: (account [9, 2] - [9, 20]
                (ident [9, 2] - [9, 8])
                (ident [9, 9] - [9, 20]))
              amount: (number [9, 31] - [9, 37]))))
        "###
        );
    }

    #[test]
    fn test_parse() {
        let src = r#"account expenses/fast_food
account liabilities/my_credit_card
account assets/my_checking
  starting_balance: 5.31

2023-01-01 "Mcdonald's"
  expenses/fast_food         10.91
  liabilities/my_credit_card -10.91
      
2023-01-02 "Paying credit card"
  liabilities/my_credit_card    10.91
  assets/my_checking           -10.91"#;

        let ledger = Parser::new().parse(src, None);

        let declared_accounts = ledger.declare_accounts();
        assert_eq!(declared_accounts.len(), 3);
        assert_eq!(
            declared_accounts[0].account().map(|a| a.value(src)),
            Some("expenses/fast_food")
        );

        let metadata = declared_accounts[2].metadata();
        assert_eq!(metadata.len(), 1);
        assert_eq!(metadata[0].key().unwrap().value(src), "starting_balance");
        assert_eq!(metadata[0].value().unwrap().value(src), "5.31");

        let transactions = ledger.transactions();
        assert_eq!(transactions.len(), 2);

        let transaction = transactions.get(0).unwrap();
        assert_eq!(transaction.date().map(|d| d.value(src)), Some("2023-01-01"));
        assert_eq!(transaction.description(src), Some("\"Mcdonald's\""));

        let postings = transaction.postings();
        let posting = postings.get(0).unwrap();
        assert_eq!(posting.account().unwrap().value(src), "expenses/fast_food");
        assert_eq!(posting.amount().unwrap().value(src), "10.91");

        let posting = postings.get(1).unwrap();
        assert_eq!(
            posting.account().unwrap().value(src),
            "liabilities/my_credit_card"
        );
        assert_eq!(posting.amount().unwrap().value(src), "-10.91");

        let transaction = transactions.get(1).unwrap();
        assert_eq!(transaction.date().map(|d| d.value(src)), Some("2023-01-02"));
        assert_eq!(transaction.description(src), Some("\"Paying credit card\""));

        let postings = transaction.postings();
        let posting = postings.get(0).unwrap();
        assert_eq!(
            posting.account().unwrap().value(src),
            "liabilities/my_credit_card"
        );
        assert_eq!(posting.amount().unwrap().value(src), "10.91");

        let posting = postings.get(1).unwrap();
        assert_eq!(posting.account().unwrap().value(src), "assets/my_checking");
        assert_eq!(posting.amount().unwrap().value(src), "-10.91");
    }
}
