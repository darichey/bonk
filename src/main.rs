#![feature(iterator_try_collect)]

mod db;

use anyhow::Result;
use db::Transaction;

fn main() -> Result<()> {
    let db = db::Db::new("./db/schema.sql", "./data/transactions.csv")?;
    for Transaction {
        date,
        description,
        amount,
    } in db.get_transactions()?
    {
        println!("{},{},{}", date, description, amount);
    }
    Ok(())
}
