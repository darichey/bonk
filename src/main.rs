#![feature(iterator_try_collect)]
#![feature(try_find)]

#[macro_use]
extern crate lazy_static;

mod db;
mod import;

use anyhow::Result;

fn main() -> Result<()> {
    let db = db::Db::new("./db/schema.sql", "./data")?;
    // for Transaction {
    //     date,
    //     description,
    //     amount,
    // } in db.get_transactions()?
    // {
    //     println!("{},{},{}", date, description, amount);
    // }
    println!("{}", db.get_transactions()?.len());
    Ok(())
}
