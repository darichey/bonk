use crate::{AppJson, AppState, BonkHttpResult};
use axum::{
    debug_handler,
    extract::{Query, State},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Transaction {
    date: String,
    description: String,
    postings: Vec<Posting>,
}

#[derive(Serialize)]
pub struct Posting {
    account: String,
    amount: i32,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    before_date: Option<String>,
    after_date: Option<String>,
    limit: usize,
}

const BASE_QUERY: &str = r#"
    SELECT
        id,
        date,
        description,
        account,
        amount
    FROM
        "transaction"
    INNER JOIN
        posting
    ON
        "transaction".id = posting."transaction"
    WHERE
        {date_condition}
    ORDER BY
        {order}
    LIMIT
        {limit}
"#;

fn construct_query(params: &PaginationParams) -> String {
    let (date_condition, order) = if let Some(before_date) = &params.before_date {
        (format!("date < '{}'", before_date), "date DESC".to_string())
    } else if let Some(after_date) = &params.after_date {
        (format!("date > '{}'", after_date), "date ASC".to_string())
    } else {
        panic!() // FIXME
    };

    BASE_QUERY
        .replace("{date_condition}", &date_condition)
        .replace("{order}", &order)
        .replace("{limit}", &params.limit.to_string())
}

#[debug_handler(state = AppState)]
pub async fn get_transactions(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> BonkHttpResult<Vec<Transaction>> {
    let con = &state
        .mutable
        .lock()
        .expect("mutable state lock poisoned")
        .db
        .con;

    let query = construct_query(&params);
    let mut transactions: Vec<(i64, Transaction)> = vec![];

    for row in con.prepare(&query)?.into_iter() {
        let row = row?;
        let id = row.read::<i64, _>("id");
        let date = row.read::<&str, _>("date");
        let description = row.read::<&str, _>("description");
        let account = row.read::<&str, _>("account");
        let amount = row.read::<i64, _>("amount") as i32;

        let posting = Posting {
            account: account.to_string(),
            amount,
        };

        match transactions.last_mut() {
            Some((last_id, transaction)) if *last_id == id => {
                transaction.postings.push(posting);
            }
            _ => {
                transactions.push((
                    id,
                    Transaction {
                        date: date.to_string(),
                        description: description.to_string(),
                        postings: vec![posting],
                    },
                ));
            }
        }
    }

    let mut transactions: Vec<Transaction> = transactions
        .into_iter()
        .map(|(_, transaction)| transaction)
        .collect();

    if params.after_date.is_some() {
        transactions.reverse();
    }

    Ok(AppJson(transactions))
}
