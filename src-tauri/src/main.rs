// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(iterator_try_collect)]
#![feature(try_find)]

use std::sync::Mutex;

use anyhow::Result;
use db::Db;

use crate::commands::{
    get_all_transactions, get_dashboard, get_dashboard_names, get_metadata, get_metadata_names,
    query_transactions, query_transactions_for_chart,
};

#[macro_use]
extern crate lazy_static;

mod commands;
mod dashboard;
mod db;
mod import_transactions;

fn main() -> Result<()> {
    let db = Mutex::new(Db::new("../data")?);

    tauri::Builder::default()
        .manage(db)
        .invoke_handler(tauri::generate_handler![
            get_all_transactions,
            query_transactions_for_chart,
            query_transactions,
            get_metadata_names,
            get_metadata,
            get_dashboard_names,
            get_dashboard
        ])
        .run(tauri::generate_context!())?;

    Ok(())
}
