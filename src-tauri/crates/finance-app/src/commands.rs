use std::{collections::HashMap, sync::Mutex};

use anyhow::{Context, Result};
use plaid::{
    apis::{configuration::Configuration, plaid_api},
    models::{
        CountryCode, LinkTokenCreateRequest, LinkTokenCreateRequestUser, Products,
        TransactionsGetRequest,
    },
};
use serde::Serialize;
use tauri::{AppHandle, State};

use crate::{
    dashboard::Dashboard,
    db::{Db, Transaction},
    PlaidAccessToken,
};

/// A wrapper for seralizing sqlite Values
pub struct Value(sqlite::Value);

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

#[derive(Serialize)]
pub struct TableData {
    column_names: Vec<String>,
    data: Vec<Vec<Value>>,
}

type ChartData = HashMap<String, Vec<Value>>;

#[tauri::command]
pub fn get_all_transactions(db: State<Mutex<Db>>) -> Result<Vec<Transaction>, String> {
    let db = db.lock().unwrap();

    let stmt = db
        .prepare("SELECT * FROM transactions ORDER BY date")
        .map_err(|err| err.to_string())?;

    db.get_transactions(stmt).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn query_transactions_for_chart(
    query: String,
    db: State<Mutex<Db>>,
) -> Result<ChartData, String> {
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

#[tauri::command]
pub fn query_transactions(query: String, db: State<Mutex<Db>>) -> Result<TableData, String> {
    // TODO: dedup with above
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

    Ok(TableData { column_names, data })
}

#[tauri::command]
pub fn get_metadata_names(db: State<Mutex<Db>>) -> Result<Vec<String>, String> {
    let db = db.lock().unwrap();

    Ok(db
        .metadatas
        .iter()
        .map(|metadata| metadata.name.clone())
        .collect())
}

#[tauri::command]
pub fn get_metadata(name: String, db: State<Mutex<Db>>) -> Result<TableData, String> {
    let db = db.lock().unwrap();
    let (column_names, data): (Vec<String>, Vec<Vec<sqlite::Value>>) =
        db.get_metadata(&name).map_err(|err| err.to_string())?;

    let data: Vec<Vec<Value>> = data
        .into_iter()
        .map(|row| row.into_iter().map(Value).collect())
        .collect();

    Ok(TableData { column_names, data })
}

#[tauri::command]
pub fn get_dashboard_names(db: State<Mutex<Db>>) -> Result<Vec<String>, String> {
    let db = db.lock().unwrap();

    return Ok(db
        .dashboards
        .iter()
        .map(|dashboard| dashboard.name.clone())
        .collect());
}

#[tauri::command]
pub fn get_dashboard(name: String, db: State<Mutex<Db>>) -> Result<Dashboard, String> {
    let db = db.lock().unwrap();

    db.dashboards
        .iter()
        .find(|dashboard| dashboard.name == name)
        .cloned()
        .ok_or("Couldn't find dashboard".to_string())
}

#[tauri::command]
pub fn render_query_template(
    template: String,
    variables: HashMap<String, String>,
    db: State<Mutex<Db>>,
) -> Result<String, String> {
    let db = db.lock().unwrap();

    let evaluated_variables = variables
        .into_iter()
        .map(|(name, query)| {
            let stmt = db.prepare(&query)?;
            let values: Vec<sqlite::Value> = db
                .query(stmt)
                .next()
                .context("Query didn't produce a row")??
                .into();
            let value = values.first().context("Row empty")?;

            let res = match value {
                sqlite::Value::Float(f) => f.to_string(),
                sqlite::Value::Integer(i) => i.to_string(),
                sqlite::Value::String(s) => s.to_string(),
                sqlite::Value::Null => "null".to_string(),
                sqlite::Value::Binary(_) => anyhow::bail!("Can't convert Binary to string"),
            };

            Ok((name, res))
        })
        .collect::<Result<HashMap<String, String>>>()
        .map_err(|err| err.to_string())?;

    Ok(format_template(&template, evaluated_variables))
}

fn format_template(template: &str, values: HashMap<String, String>) -> String {
    let mut result = String::from(template);

    for (key, value) in values.iter() {
        let placeholder = format!("{{{{{}}}}}", key);
        result = result.replace(&placeholder, value);
    }

    result
}

#[tauri::command]
pub async fn create_link_token(plaid_config: State<'_, Configuration>) -> Result<String, String> {
    println!("create_link_token");

    Ok(plaid_api::link_token_create(
        &plaid_config,
        LinkTokenCreateRequest {
            products: Some(vec![Products::Auth, Products::Transactions]),
            ..LinkTokenCreateRequest::new(
                "finance-app".to_string(),
                "en".to_string(),
                vec![CountryCode::Us],
                LinkTokenCreateRequestUser::new("user-id".to_string()),
            )
        },
    )
    .await
    .map_err(|err| err.to_string())?
    .link_token)
}

#[tauri::command]
pub async fn exchange_public_token(
    plaid_config: State<'_, Configuration>,
    plaid_access_token: State<'_, PlaidAccessToken>,
    public_token: String,
) -> Result<String, String> {
    println!("exchange_public_token");

    let access_token = plaid_api::item_public_token_exchange(
        &plaid_config,
        plaid::models::ItemPublicTokenExchangeRequest {
            public_token,
            ..Default::default()
        },
    )
    .await
    .map_err(|err| err.to_string())?
    .access_token;

    plaid_access_token.set(access_token.clone());

    Ok(access_token)
}

#[derive(Serialize)]
pub struct PlaidTransaction {
    account: String,
    amount: f64,
    date: String,
    name: String,
}

#[tauri::command]
pub async fn plaid_get_transactions(
    plaid_config: State<'_, Configuration>,
    plaid_access_token: State<'_, PlaidAccessToken>,
) -> Result<Vec<PlaidTransaction>, String> {
    let response = plaid_api::transactions_get(
        &plaid_config,
        TransactionsGetRequest {
            access_token: plaid_access_token.get(),
            start_date: "2023-01-01".to_string(),
            end_date: "2023-11-20".to_string(),
            ..Default::default()
        },
    )
    .await
    .map_err(|err| err.to_string())?;

    let accounts: HashMap<String, String> = response
        .accounts
        .into_iter()
        .map(|account| (account.account_id, account.name))
        .collect();

    let transactions = response
        .transactions
        .into_iter()
        .map(|transaction| PlaidTransaction {
            account: accounts
                .get(&transaction.account_id)
                .expect("unknown account id")
                .clone(),
            amount: transaction.amount,
            date: transaction.date,
            name: transaction.name,
        })
        .collect();

    Ok(transactions)
}
