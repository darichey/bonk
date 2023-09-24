// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(iterator_try_collect)]
#![feature(try_find)]

use std::sync::Mutex;

use anyhow::Result;
use db::{Db, Transaction};
use tauri::State;

#[macro_use]
extern crate lazy_static;

mod db;
mod import;

fn main() -> Result<()> {
    let db = Mutex::new(Db::new("../db/schema.sql", "../data")?);

    tauri::Builder::default()
        .manage(db)
        .invoke_handler(tauri::generate_handler![
            get_all_transactions,
            query_transactions_for_chart
        ])
        .run(tauri::generate_context!())?;

    Ok(())
}

#[tauri::command]
fn get_all_transactions(db: State<Mutex<Db>>) -> Result<Vec<Transaction>, String> {
    let db = db.lock().unwrap();

    let stmt = db
        .prepare("SELECT * FROM transactions ORDER BY date")
        .map_err(|err| err.to_string())?;

    db.get_transactions(stmt).map_err(|err| err.to_string())
}

#[tauri::command]
fn query_transactions_for_chart(
    query: String,
    db: State<Mutex<Db>>,
) -> Result<Vec<(String, i64)>, String> {
    let db = db.lock().unwrap();

    let stmt = db.prepare(&query).map_err(|err| err.to_string())?;

    db.query(stmt, |row| {
        let x = row.try_read::<&str, _>("x")?.to_string();
        let y = row.try_read::<i64, _>("y")?;
        Ok((x, y))
    })
    // TODO: this is ugly because of mixing anyhow result and result
    .collect::<Result<Vec<(String, i64)>>>()
    .map_err(|err| format!("{:#}", err))
}
