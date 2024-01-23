use core::fmt;

use tree_sitter::{InputEdit, Node, Point, Range, Tree};

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

    pub fn parse(&mut self, src: &str, old_ledger: Option<&Ledger>) -> Ledger {
        let tree = self
            .ts_parser
            .parse(src, old_ledger.map(|old_ledger| &old_ledger.0))
            .unwrap();
        Ledger::new(tree)
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourceSpan(pub Range);

pub struct Ledger(Tree);

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

    #[allow(clippy::too_many_arguments)] // I'm fine with this for now
    pub fn edit(
        &mut self,
        old_src: &str,
        new_src: &str,
        start_line: usize,
        start_col: usize,
        end_line: usize,
        end_col: usize,
        change_length: usize,
    ) {
        let start_byte = position_to_byte_offset(old_src, start_line, start_col);
        let new_end_byte = start_byte + change_length;
        let new_end_position = byte_offset_to_position(new_src, new_end_byte);

        self.0.edit(&InputEdit {
            start_byte,
            old_end_byte: position_to_byte_offset(old_src, end_line, end_col),
            new_end_byte,
            start_position: Point {
                row: start_line,
                column: start_col,
            },
            old_end_position: Point {
                row: end_line,
                column: end_col,
            },
            new_end_position: Point {
                row: new_end_position.0,
                column: new_end_position.1,
            },
        });
    }
}

pub fn position_to_byte_offset(text: &str, line: usize, col: usize) -> usize {
    let mut cur_line = 0;
    let mut cur_col = 0;

    for (offset, c) in text.char_indices() {
        if cur_line == line && cur_col == col {
            return offset;
        }

        if c == '\n' {
            cur_line += 1;
            cur_col = 0;
        } else {
            cur_col += 1;
        }
    }

    text.len()
}

pub fn byte_offset_to_position(text: &str, offset: usize) -> (usize, usize) {
    let mut cur_line = 0;
    let mut cur_col = 0;

    for (o, c) in text.char_indices() {
        if o == offset {
            break;
        }

        if c == '\n' {
            cur_line += 1;
            cur_col = 0;
        } else {
            cur_col += 1;
        }
    }

    (cur_line, cur_col)
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
    pub fn date<'s>(&self, src: &'s str) -> Option<&'s str> {
        self.0
            .child_by_field_name("date")
            .map(|n| n.utf8_text(src.as_bytes()).expect("src is not valid utf-8"))
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
}

pub struct Posting<'a>(Node<'a>);

impl Posting<'_> {
    pub fn account(&self) -> Option<Account> {
        self.0.child_by_field_name("account").map(Account)
    }

    pub fn amount(&self) -> Option<Amount> {
        self.0.child_by_field_name("amount").map(Amount)
    }
}

pub struct Account<'a>(Node<'a>);

impl Account<'_> {
    pub fn value<'s>(&self, src: &'s str) -> &'s str {
        self.0
            .utf8_text(src.as_bytes())
            .expect("src is not valid utf-8")
    }
}

pub struct Amount<'a>(Node<'a>);

impl Amount<'_> {
    pub fn value<'s>(&self, src: &'s str) -> &'s str {
        self.0
            .utf8_text(src.as_bytes())
            .expect("src is not valid utf-8")
    }
}

#[cfg(test)]
mod tests {
    use crate::{position_to_byte_offset, Parser};

    #[test]
    fn test_debug_fmt() {
        let src = r#"2023-01-01 "Mcdonald's"
  expenses:fast_food         10.91
  liabilities:my_credit_card
      
2023-01-02 "Paying credit card"
  liabilities:my_credit_card    10.91
  assets:my_checking"#;

        let ledger = Parser::new().parse(src, None);

        assert_eq!(
            format!("{:?}", ledger),
            r#"(ledger [0, 0] - [6, 20]
  transaction: (transaction [0, 0] - [2, 28]
    date: (date [0, 0] - [0, 10])
    description: (description [0, 11] - [0, 23])
    posting: (posting [1, 2] - [1, 34]
      account: (account [1, 2] - [1, 20])
      amount: (amount [1, 29] - [1, 34]))
    posting: (posting [2, 2] - [2, 28]
      account: (account [2, 2] - [2, 28])))
  transaction: (transaction [4, 0] - [6, 20]
    date: (date [4, 0] - [4, 10])
    description: (description [4, 11] - [4, 31])
    posting: (posting [5, 2] - [5, 37]
      account: (account [5, 2] - [5, 28])
      amount: (amount [5, 32] - [5, 37]))
    posting: (posting [6, 2] - [6, 20]
      account: (account [6, 2] - [6, 20]))))
"#
        );
    }

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
        assert_eq!(transaction.date(src), Some("2023-01-01"));
        assert_eq!(transaction.description(src), Some("\"Mcdonald's\""));

        let postings = transaction.postings();
        let posting = postings.get(0).unwrap();
        assert_eq!(posting.account().unwrap().value(src), "expenses:fast_food");
        assert_eq!(posting.amount().unwrap().value(src), "10.91");

        let posting = postings.get(1).unwrap();
        assert_eq!(
            posting.account().unwrap().value(src),
            "liabilities:my_credit_card"
        );
        assert!(posting.amount().is_none());

        let transaction = transactions.get(1).unwrap();
        assert_eq!(transaction.date(src), Some("2023-01-02"));
        assert_eq!(transaction.description(src), Some("\"Paying credit card\""));

        let postings = transaction.postings();
        let posting = postings.get(0).unwrap();
        assert_eq!(
            posting.account().unwrap().value(src),
            "liabilities:my_credit_card"
        );
        assert_eq!(posting.amount().unwrap().value(src), "10.91");

        let posting = postings.get(1).unwrap();
        assert_eq!(posting.account().unwrap().value(src), "assets:my_checking");
        assert!(posting.amount().is_none());
    }

    #[test]
    fn test_position_to_byte_offset() {
        let s = "foo\nbars\nbazzz";

        assert_eq!(position_to_byte_offset(s, 0, 0), 0);

        assert_eq!(position_to_byte_offset(s, 0, 2), 2);

        assert_eq!(position_to_byte_offset(s, 1, 0), 4);

        assert_eq!(position_to_byte_offset(s, 2, 3), 12);

        assert_eq!(position_to_byte_offset(s, 2, 5), 14)
    }
}
