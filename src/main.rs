use std::fs;

use sqlite::{State, Value};

fn main() {
    // open db connection
    let con = sqlite::open(":memory:").unwrap();

    // create transactions table
    let query = fs::read_to_string("./db/schema.sql").unwrap();
    con.execute(query).unwrap();

    // insert some transactions
    con.execute("BEGIN TRANSACTION").unwrap();

    let mut csv_reader = csv::Reader::from_path("./data/transactions.csv").unwrap();
    for result in csv_reader.records() {
        let row = result.unwrap();

        let date = row.get(0).unwrap();
        let description = row.get(1).unwrap();
        let amount = parse_amount(row.get(2).unwrap());

        let mut stmt = con
            .prepare("INSERT INTO transactions VAlUES (:date, :description, :amount)")
            .unwrap();

        stmt.bind::<&[(_, Value)]>(
            &[
                (":date", date.into()),
                (":description", description.into()),
                (":amount", amount.into()),
            ][..],
        )
        .unwrap();

        while let Ok(State::Row) = stmt.next() {}
    }
    con.execute("COMMIT").unwrap();

    // query transactions
    let query = "
        SELECT * FROM transactions;
    ";
    let mut stmt = con.prepare(query).unwrap();
    while let Ok(sqlite::State::Row) = stmt.next() {
        println!(
            "{},{},{}",
            stmt.read::<String, _>("date").unwrap(),
            stmt.read::<String, _>("description").unwrap(),
            stmt.read::<String, _>("amount").unwrap()
        )
    }
}

fn parse_amount(s: &str) -> i64 {
    let negative = s.starts_with('-');
    let (dollars, cents) = s.split_once('.').unwrap();
    let dollars: i64 = str::parse(&dollars.replace('-', "")).unwrap();
    let cents: i64 = str::parse(cents).unwrap();
    let amt = dollars * 100 + cents;
    if negative {
        -amt
    } else {
        amt
    }
}
