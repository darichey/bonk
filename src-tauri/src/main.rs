// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use anyhow::Result;
use db::Db;
use tauri::Manager;

use crate::commands::{
    get_all_transactions, get_dashboard, get_dashboard_names, get_metadata, get_metadata_names,
    query_transactions, query_transactions_for_chart, render_query_template,
};

#[macro_use]
extern crate lazy_static;

mod commands;
mod dashboard;
mod db;
mod import_transactions;
mod metadata;

fn main() -> Result<()> {
    tauri::Builder::default()
        .setup(|app| match app.get_cli_matches() {
            Ok(matches) => {
                let path_to_schema = app
                    .path_resolver()
                    .resolve_resource("schema.sql")
                    .expect("Failed to load schema.sql resource");

                let path_to_data = matches
                    .args
                    .get("data_dir")
                    .expect("clap enforces data_dir is present")
                    .value
                    .as_str()
                    .expect("clap enforces data_dir is string");

                let db = Mutex::new(Db::new(path_to_schema, path_to_data)?);
                app.manage(db);

                Ok(())
            }
            Err(err) => Err(Box::new(err)),
        })
        .invoke_handler(tauri::generate_handler![
            get_all_transactions,
            query_transactions_for_chart,
            query_transactions,
            get_metadata_names,
            get_metadata,
            get_dashboard_names,
            get_dashboard,
            render_query_template,
        ])
        .run(tauri::generate_context!())?;

    Ok(())
}
