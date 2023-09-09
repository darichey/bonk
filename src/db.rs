use std::{fs, path::Path};

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
            let amount = parse_amount(row.get(2).context("Amount not present")?)?;

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

    pub fn get_transactions(&self) -> Result<Vec<(String, String, i64)>> {
        let query = "
            SELECT * FROM transactions;
        ";
        let mut stmt = self.con.prepare(query)?;
        let mut vec = Vec::new();
        while let Ok(sqlite::State::Row) = stmt.next() {
            vec.push((
                stmt.read::<String, _>("date")?,
                stmt.read::<String, _>("description")?,
                stmt.read::<i64, _>("amount")?,
            ))
        }
        Ok(vec)
    }
}

fn parse_amount(s: &str) -> Result<i64> {
    let negative = s.starts_with('-');
    let (dollars, cents) = s.split_once('.').context("Couldn't split")?;
    let dollars: i64 = str::parse(&dollars.replace('-', ""))?;
    let cents: i64 = str::parse(cents)?;
    let amt = dollars * 100 + cents;
    if negative {
        Ok(-amt)
    } else {
        Ok(amt)
    }
}
