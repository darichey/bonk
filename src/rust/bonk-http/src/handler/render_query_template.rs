use std::collections::HashMap;

use anyhow::Context;
use axum::{debug_handler, extract::State, Json};
use serde::Deserialize;

use crate::{AppJson, AppState, BonkHttpResult};

#[derive(Deserialize)]
pub struct RenderQueryTemplateRequest {
    template: String,
    variables: HashMap<String, String>,
}

#[debug_handler(state = AppState)]
pub async fn render_query_template(
    State(state): State<AppState>,
    Json(body): Json<RenderQueryTemplateRequest>,
) -> BonkHttpResult<String> {
    let con = &state.db.lock().expect("db lock poisoned").con;

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

    let formatted = format_template(&body.template, evaluated_variables);

    Ok(AppJson(formatted))
}

fn format_template(template: &str, values: HashMap<String, String>) -> String {
    let mut result = String::from(template);

    for (key, value) in values.iter() {
        let placeholder = format!("{{{{{}}}}}", key);
        result = result.replace(&placeholder, value);
    }

    result
}
