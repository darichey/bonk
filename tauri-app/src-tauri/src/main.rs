// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, sync::Mutex};

use anyhow::Result;
use db::Db;
use plaid::apis::configuration::Configuration;
use reqwest::header::HeaderMap;
use tauri::{App, Manager};

use crate::commands::{
    create_link_token, exchange_public_token, get_all_transactions, get_dashboard,
    get_dashboard_names, get_metadata, get_metadata_names, plaid_get_transactions,
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
        .setup(|app| {
            if let Err(err) = setup_db(app) {
                // TODO: this is ugly but is helpful in debugging because otherwise the context is swallowed
                println!("{:#?}", err);
                return Err(err.into());
            }
            setup_plaid(app)?;
            Ok(())
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
            create_link_token,
            exchange_public_token,
            plaid_get_transactions,
        ])
        .run(tauri::generate_context!())?;

    Ok(())
}

fn setup_db(app: &mut App) -> Result<()> {
    let path_to_schema = app
        .path_resolver()
        .resolve_resource("schema.sql")
        .expect("Failed to load schema.sql resource");

    let matches = app.get_cli_matches()?;

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

pub struct PlaidAccessToken(Mutex<Option<String>>);

impl PlaidAccessToken {
    pub fn get(&self) -> String {
        self.0
            .lock()
            .unwrap()
            .as_ref()
            .expect("Plaid access token not yet set")
            .clone()
    }

    pub fn set(&self, access_token: String) {
        *self.0.lock().unwrap() = Some(access_token);
    }
}

fn setup_plaid(app: &mut App) -> Result<()> {
    let base_path = env::var("PLAID_ENV")?;
    let client_id = env::var("PLAID_CLIENT_ID")?;
    let secret = env::var("PLAID_SECRET")?;
    let version = env::var("PLAID_VERSION")?;

    let mut headers = HeaderMap::new();
    headers.insert("Plaid-Client-Id", client_id.parse()?);
    headers.insert("Plaid-Secret", secret.parse()?);
    headers.insert("Plaid-Version", version.parse()?);

    let plaid_config = Configuration {
        base_path,
        client: reqwest::Client::builder()
            .default_headers(headers)
            .build()?,
        ..Default::default()
    };
    app.manage(plaid_config);
    app.manage(PlaidAccessToken(Default::default()));

    Ok(())
}
