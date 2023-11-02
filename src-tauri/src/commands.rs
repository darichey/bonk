use std::{collections::HashMap, sync::Mutex};

use anyhow::Result;
use serde::Serialize;
use tauri::State;

use crate::{
    dashboard::Dashboard,
    db::{Db, Transaction},
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
            let res = db
                .query(stmt)
                .next()
                .expect("Query didn't produce a row")?
                .try_read::<&str, _>(0)?
                .to_string();
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
