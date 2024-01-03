mod parse;
mod types;

fn main() {
    println!("hello world");

    let ledger = parse::parse(
        r#"2023-01-01 "Mcdonald's"
  expenses:fast_food         10.91
  liabilities:my_credit_card
"#,
    );

    dbg!(ledger);
}
