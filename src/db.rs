use std::{fs, num::ParseIntError, path::Path};

use sqlite::{Connection, State, Value};

pub struct Db {
    con: Connection,
}

impl Db {
    pub fn new<P: AsRef<Path>>(path_to_schema: P, path_to_transactions: P) -> Result<Db, String> {
        // open db connection
        let con = sqlite::open(":memory:").map_err(|err| err.to_string())?;

        // create transactions table
        let query = fs::read_to_string(path_to_schema).map_err(|err| err.to_string())?;
        con.execute(query).map_err(|err| err.to_string())?;

        // insert some transactions
        con.execute("BEGIN TRANSACTION")
            .map_err(|err| err.to_string())?;

        let mut csv_reader =
            csv::Reader::from_path(path_to_transactions).map_err(|err| err.to_string())?;
        for result in csv_reader.records() {
            let row = result.map_err(|err| err.to_string())?;

            let date = row.get(0).ok_or("Date not present")?;
            let description = row.get(1).ok_or("Description not present")?;
            let amount = parse_amount(row.get(2).ok_or("Amount not present")?)?;

            let mut stmt = con
                .prepare("INSERT INTO transactions VAlUES (:date, :description, :amount)")
                .map_err(|err| err.to_string())?;

            stmt.bind::<&[(_, Value)]>(
                &[
                    (":date", date.into()),
                    (":description", description.into()),
                    (":amount", amount.into()),
                ][..],
            )
            .map_err(|err| err.to_string())?;

            while let Ok(State::Row) = stmt.next() {}
        }
        con.execute("COMMIT").map_err(|err| err.to_string())?;

        Ok(Db { con })
    }

    pub fn get_transactions(&self) -> Result<Vec<(String, String, i64)>, String> {
        let query = "
            SELECT * FROM transactions;
        ";
        let mut stmt = self.con.prepare(query).map_err(|err| err.to_string())?;
        let mut vec = Vec::new();
        while let Ok(sqlite::State::Row) = stmt.next() {
            vec.push((
                stmt.read::<String, _>("date")
                    .map_err(|err| err.to_string())?,
                stmt.read::<String, _>("description")
                    .map_err(|err| err.to_string())?,
                stmt.read::<i64, _>("amount")
                    .map_err(|err| err.to_string())?,
            ))
        }
        Ok(vec)
    }
}

fn parse_amount(s: &str) -> Result<i64, String> {
    let negative = s.starts_with('-');
    let (dollars, cents) = s.split_once('.').ok_or("Couldn't split")?;
    let dollars: i64 =
        str::parse(&dollars.replace('-', "")).map_err(|err: ParseIntError| err.to_string())?;
    let cents: i64 = str::parse(cents).map_err(|err: ParseIntError| err.to_string())?;
    let amt = dollars * 100 + cents;
    if negative {
        Ok(-amt)
    } else {
        Ok(amt)
    }
}
