// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(iterator_try_collect)]
#![feature(try_find)]

use std::{collections::HashMap, sync::Mutex};

use anyhow::Result;
use db::{Db, Transaction};
use serde::{Deserialize, Serialize};
use sqlite::Row;
use tauri::State;

#[macro_use]
extern crate lazy_static;

mod db;
mod import_transactions;

fn main() -> Result<()> {
    let db = Mutex::new(Db::new("../data")?);

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

struct Value(sqlite::Value);

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self.0 {
            sqlite::Value::Binary(b) => serializer.serialize_bytes(b),
            sqlite::Value::Float(f) => serializer.serialize_f64(*f),
            sqlite::Value::Integer(i) => serializer.serialize_i64(*i),
            sqlite::Value::String(s) => serializer.serialize_str(s),
            sqlite::Value::Null => serializer.serialize_unit(),
        }
    }
}

#[tauri::command]
fn query_transactions_for_chart(
    query: String,
    db: State<Mutex<Db>>,
) -> Result<HashMap<String, Vec<Value>>, String> {
    let db = db.lock().unwrap();

    let stmt = db.prepare(&query).map_err(|err| err.to_string())?;

    let column_names = stmt.column_names();

    let data = db
        .query(stmt)
        .map(|row| {
            let values: Vec<sqlite::Value> = row?.into();
            let values: Vec<Value> = values.into_iter().map(Value).collect();
            Ok(values)
        })
        .collect::<Result<Vec<Vec<Value>>>>()
        .map_err(|err| err.to_string())?;

    let mut data_iters: Vec<_> = data.into_iter().map(Vec::into_iter).collect();

    let columns: Vec<Vec<Value>> = (0..column_names.len())
        .map(|_| data_iters.iter_mut().map(|it| it.next().unwrap()).collect())
        .collect();

    let map: HashMap<String, Vec<Value>> = column_names.into_iter().zip(columns).collect();

    Ok(map)
}
