#![feature(iterator_try_collect)]

mod db;

fn main() -> Result<(), String> {
    let db = db::Db::new("./db/schema.sql", "./data/transactions.csv")?;
    for (date, description, amount) in db.get_transactions()? {
        println!("{},{},{}", date, description, amount);
    }
    Ok(())
}
