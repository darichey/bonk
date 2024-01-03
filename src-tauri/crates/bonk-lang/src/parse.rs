use chrono::NaiveDate;
use tree_sitter::{Parser as TSParser, TreeCursor};

use crate::types::{Account, Amount, Ledger, Posting, Transaction};

struct Parser<'a, 'b, 'c> {
    src: &'a str,
    cursor: &'b mut TreeCursor<'c>,
}

pub fn parse(src: &str) -> Ledger {
    let mut parser = TSParser::new();
    parser.set_language(tree_sitter_bonk::language()).unwrap();
    let tree = parser.parse(src, None).unwrap();
    let mut cursor = tree.walk();

    Parser {
        src,
        cursor: &mut cursor,
    }
    .parse_ledger()
}

impl Parser<'_, '_, '_> {
    fn parse_ledger(&mut self) -> Ledger {
        assert!(self.cursor.node().kind() == "ledger");
        self.cursor.goto_first_child();
        let transactions = self.parse_transactions();
        self.cursor.goto_parent();
        Ledger { transactions }
    }

    fn parse_transactions(&mut self) -> Vec<Transaction> {
        let mut transactions = Vec::new();
        loop {
            transactions.push(self.parse_transaction());
            if !self.cursor.goto_next_sibling() {
                break;
            }
        }
        transactions
    }

    fn parse_transaction(&mut self) -> Transaction {
        assert!(self.cursor.node().kind() == "transaction");
        self.cursor.goto_first_child();
        let date = self.parse_date();
        self.cursor.goto_next_sibling();
        let description = self.parse_description();
        self.cursor.goto_next_sibling();
        let postings = self.parse_postings();
        self.cursor.goto_parent();
        Transaction {
            date,
            description,
            postings,
        }
    }

    fn parse_date(&mut self) -> NaiveDate {
        assert!(self.cursor.node().kind() == "date");
        let str = self.cursor.node().utf8_text(self.src.as_bytes()).unwrap();
        NaiveDate::parse_from_str(str, "%Y-%m-%d").unwrap()
    }

    fn parse_description(&mut self) -> String {
        assert!(self.cursor.node().kind() == "description");
        let str = self.cursor.node().utf8_text(self.src.as_bytes()).unwrap();
        str.to_owned()
    }

    fn parse_postings(&mut self) -> Vec<Posting> {
        let mut postings = Vec::new();
        loop {
            postings.push(self.parse_posting());
            if !self.cursor.goto_next_sibling() {
                break;
            }
        }
        postings
    }

    fn parse_posting(&mut self) -> Posting {
        assert!(self.cursor.node().kind() == "posting");
        self.cursor.goto_first_child();
        let account = self.parse_account();
        let amount = if self.cursor.goto_next_sibling() {
            Some(self.parse_amount())
        } else {
            None
        };
        self.cursor.goto_parent();
        Posting { account, amount }
    }

    fn parse_account(&self) -> Account {
        assert!(self.cursor.node().kind() == "account");
        let str = self.cursor.node().utf8_text(self.src.as_bytes()).unwrap();
        Account {
            path: str.split(':').map(|s| s.to_owned()).collect(),
        }
    }

    fn parse_amount(&self) -> Amount {
        assert!(self.cursor.node().kind() == "amount");
        let str = self.cursor.node().utf8_text(self.src.as_bytes()).unwrap();
        Amount {
            cents: str.replace('.', "").parse().unwrap(),
        }
    }
}
