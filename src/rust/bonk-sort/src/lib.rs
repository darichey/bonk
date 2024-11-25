use bonk_parse::Parser;

pub mod cli;

pub fn sort_ledger(src: &str) -> String {
    let mut parser = Parser::new();
    let ledger = parser.parse(src, None);

    let declare_accounts: Vec<_> = ledger
        .declare_accounts()
        .into_iter()
        .map(|a| a.value(src))
        .collect();
    let declare_accounts = declare_accounts.join("\n");

    let mut transactions = ledger.transactions();
    transactions.sort_by_key(|t| (t.date().map(|d| d.value(src)), t.description(src)));
    let transactions: Vec<_> = transactions.into_iter().map(|t| t.value(src)).collect();
    let transactions = transactions.join("\n\n");

    format!("{}\n\n{}", declare_accounts, transactions)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_sort_ledger() {
        let src = r#"account expenses/food
account income/salary
account assets/checking

2021-01-31 "Food"
    expenses/food  -5
    assets/checking

2021-01-10 "Food"
    expenses/food  -5
    assets/checking

2021-01-04 "Paycheck"
    assets/checking  100
    income/salary

2021-01-25 "Food"
    expenses/food  -5
    assets/checking"#;

        let sorted = super::sort_ledger(src);

        let expected = r#"account expenses/food
account income/salary
account assets/checking

2021-01-04 "Paycheck"
    assets/checking  100
    income/salary

2021-01-10 "Food"
    expenses/food  -5
    assets/checking

2021-01-25 "Food"
    expenses/food  -5
    assets/checking

2021-01-31 "Food"
    expenses/food  -5
    assets/checking"#;

        assert_eq!(sorted, expected);
    }

    #[test]
    fn test_sort_ledger_partial() {
        let src = r#"account expenses/food
account income/salary
account assets/checking

2021-01-31 "Food"
    expenses/food  -5

2021-01-10 "Food"
    expenses/food  -5

2021-01-04 "Paycheck"
    assets/checking  100

2021-01-25 "Food"
    expenses/food  -5"#;

        let sorted = super::sort_ledger(src);

        let expected = r#"account expenses/food
account income/salary
account assets/checking

2021-01-04 "Paycheck"
    assets/checking  100

2021-01-10 "Food"
    expenses/food  -5

2021-01-25 "Food"
    expenses/food  -5

2021-01-31 "Food"
    expenses/food  -5"#;

        assert_eq!(sorted, expected);
    }
}
