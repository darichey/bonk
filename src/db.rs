use std::{fmt::Display, fs, path::Path};

use chrono::NaiveDate;
use sqlite::{Connection, State, Value};

use anyhow::{Context, Result};

pub struct Db {
    con: Connection,
}

impl Db {
    pub fn new<P: AsRef<Path>>(path_to_schema: P, path_to_transactions: P) -> Result<Db> {
        // open db connection
        let con = sqlite::open(":memory:")?;

        // create transactions table
        let query = fs::read_to_string(path_to_schema)?;
        con.execute(query)?;

        // insert some transactions
        con.execute("BEGIN TRANSACTION")?;

        let mut csv_reader = csv::Reader::from_path(path_to_transactions)?;
        for result in csv_reader.records() {
            let row = result?;

            let date = row.get(0).context("Date not present")?;
            let description = row.get(1).context("Description not present")?;
            let amount = DollarAmount::parse(row.get(2).context("Amount not present")?)?.cents;

            let mut stmt =
                con.prepare("INSERT INTO transactions VAlUES (:date, :description, :amount)")?;

            stmt.bind::<&[(_, Value)]>(
                &[
                    (":date", date.into()),
                    (":description", description.into()),
                    (":amount", amount.into()),
                ][..],
            )?;

            while let Ok(State::Row) = stmt.next() {}
        }
        con.execute("COMMIT")?;

        Ok(Db { con })
    }

    pub fn get_transactions(&self) -> Result<Vec<Transaction>> {
        let query = "
            SELECT * FROM transactions;
        ";
        let mut stmt = self.con.prepare(query)?;
        let mut vec = Vec::new();
        while let Ok(sqlite::State::Row) = stmt.next() {
            vec.push(Transaction {
                date: NaiveDate::parse_from_str(&stmt.read::<String, _>("date")?, "%Y-%m-%d")?,
                description: stmt.read::<String, _>("description")?,
                amount: DollarAmount {
                    cents: stmt.read::<i64, _>("amount")?,
                },
            })
        }
        Ok(vec)
    }
}

pub struct Transaction {
    pub date: NaiveDate,
    pub description: String,
    pub amount: DollarAmount,
}

pub struct DollarAmount {
    pub cents: i64,
}

impl DollarAmount {
    // TODO: replace with TryFrom<&str>, need to enumerate error cases which is good anyways
    pub fn parse(s: &str) -> Result<DollarAmount> {
        let negative = s.starts_with('-');
        let (dollars, cents) = s.split_once('.').context("Couldn't split")?;
        let dollars: i64 = str::parse(&dollars.replace('-', ""))?;
        let cents: i64 = str::parse(cents)?;
        let cents = dollars * 100 + cents;
        let cents = if negative { -cents } else { cents };
        Ok(DollarAmount { cents })
    }
}

impl Display for DollarAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cents)
    }
}
