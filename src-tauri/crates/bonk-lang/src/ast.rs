use tree_sitter::Node;

pub struct Ledger<'a>(Node<'a>, &'a str);

impl<'a> Ledger<'a> {
    pub fn new(node: Node<'a>, src: &'a str) -> Self {
        Self(node, src)
    }

    pub fn transactions(&self) -> Vec<Transaction<'_>> {
        let mut cursor = self.0.walk();
        self.0
            .children_by_field_name("transaction", &mut cursor)
            .map(|n| Transaction(n, self.1))
            .collect()
    }
}

pub struct Transaction<'a>(Node<'a>, &'a str);

impl Transaction<'_> {
    pub fn date(&self) -> Option<&str> {
        self.0
            .child_by_field_name("date")
            .map(|n| n.utf8_text(self.1.as_bytes()).unwrap())
    }

    pub fn description(&self) -> Option<&str> {
        self.0
            .child_by_field_name("description")
            .map(|n| n.utf8_text(self.1.as_bytes()).unwrap())
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
    pub fn account(&self) -> Option<&str> {
        self.0
            .child_by_field_name("account")
            .map(|n| n.utf8_text(self.1.as_bytes()).unwrap())
    }

    pub fn amount(&self) -> Option<&str> {
        self.0
            .child_by_field_name("amount")
            .map(|n| n.utf8_text(self.1.as_bytes()).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use tree_sitter::Parser;

    use super::Ledger;

    #[test]
    fn test() {
        let src = r#"2023-01-01 "Mcdonald's"
  expenses:fast_food         10.91
  liabilities:my_credit_card
      
2023-01-02 "Paying credit card"
  liabilities:my_credit_card    10.91
  assets:my_checking"#;

        let mut parser = Parser::new();
        parser.set_language(tree_sitter_bonk::language()).unwrap();
        let tree = parser.parse(src, None).unwrap();
        let root = tree.root_node();
        let ledger = Ledger::new(root, src);

        let transactions = ledger.transactions();
        assert_eq!(transactions.len(), 2);

        let transaction = transactions.get(0).unwrap();
        assert_eq!(transaction.date(), Some("2023-01-01"));
        assert_eq!(transaction.description(), Some("\"Mcdonald's\""));

        let postings = transaction.postings();
        let posting = postings.get(0).unwrap();
        assert_eq!(posting.account(), Some("expenses:fast_food"));
        assert_eq!(posting.amount(), Some("10.91"));

        let posting = postings.get(1).unwrap();
        assert_eq!(posting.account(), Some("liabilities:my_credit_card"));
        assert_eq!(posting.amount(), None);

        let transaction = transactions.get(1).unwrap();
        assert_eq!(transaction.date(), Some("2023-01-02"));
        assert_eq!(transaction.description(), Some("\"Paying credit card\""));

        let postings = transaction.postings();
        let posting = postings.get(0).unwrap();
        assert_eq!(posting.account(), Some("liabilities:my_credit_card"));
        assert_eq!(posting.amount(), Some("10.91"));

        let posting = postings.get(1).unwrap();
        assert_eq!(posting.account(), Some("assets:my_checking"));
        assert_eq!(posting.amount(), None);
    }
}
