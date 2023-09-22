#![feature(iterator_try_collect)]
#![feature(try_find)]

#[macro_use]
extern crate lazy_static;

mod db;
mod import;

use anyhow::Result;
use db::Transaction;

fn main() -> Result<()> {
    let db = db::Db::new("./db/schema.sql", "./data")?;
    for Transaction {
        date,
        description,
        amount,
        account,
    } in db.get_transactions(db.prepare("SELECT * FROM transactions ORDER BY date;")?)?
    {
        println!("{},{},{},{}", date, description, amount, account);
    }
    // println!("{}", db.get_transactions()?.len());
    Ok(())
}
