use axum::{debug_handler, extract::State};
use serde::Serialize;

use crate::{AppJson, AppState, BonkHttpResult};

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

const QUERY: &str = r#"
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
    ORDER BY
        date DESC
"#;

// TODO: paginate by date
#[debug_handler(state = AppState)]
pub async fn get_transactions(State(state): State<AppState>) -> BonkHttpResult<Vec<Transaction>> {
    let con = &state
        .mutable
        .lock()
        .expect("mutable state lock poisoned")
        .db
        .con;

    let mut transactions: Vec<(i64, Transaction)> = vec![];

    for row in con.prepare(QUERY)?.into_iter() {
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

    let transactions = transactions
        .into_iter()
        .map(|(_, transaction)| transaction)
        .collect();

    Ok(AppJson(transactions))
}
