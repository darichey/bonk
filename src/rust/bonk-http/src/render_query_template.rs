use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::Context;
use rouille::{Request, Response};
use serde::Deserialize;

use crate::{try_or_400, State};

#[derive(Deserialize)]
struct RenderQueryTemplateRequest {
    template: String,
    variables: HashMap<String, String>,
}

pub(crate) fn render_query_template(request: &Request, state: Arc<Mutex<State>>) -> Response {
    let body = try_or_400!(handle(request, state));
    Response::json(&body)
}

fn handle(request: &Request, state: Arc<Mutex<State>>) -> anyhow::Result<String> {
    let state = state.lock().expect("Couldn't acquire state");
    let con = &state.db.con;

    let body: RenderQueryTemplateRequest = rouille::input::json_input(request)?;

    let evaluated_variables = body
        .variables
        .into_iter()
        .map(|(name, query)| {
            let stmt = con.prepare(&query)?;
            let values: Vec<sqlite::Value> = stmt
                .into_iter()
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
        .collect::<anyhow::Result<HashMap<String, String>>>()?;

    Ok(format_template(&body.template, evaluated_variables))
}

fn format_template(template: &str, values: HashMap<String, String>) -> String {
    let mut result = String::from(template);

    for (key, value) in values.iter() {
        let placeholder = format!("{{{{{}}}}}", key);
        result = result.replace(&placeholder, value);
    }

    result
}
