#[derive(PartialEq, Eq, Debug)]
pub struct Ledger<'input> {
    pub transactions: Vec<Transaction<'input>>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Transaction<'input> {
    pub date: &'input str,
    pub description: &'input str,
    pub postings: Vec<Posting<'input>>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Posting<'input> {
    pub account: &'input str,
    pub amount: Option<&'input str>,
}
