use bonk_ast_errorless::{Account, Amount, Ledger, Posting, Transaction};
use chrono::NaiveDate;
use clap::Parser;
use plaid::{
    apis::{configuration::Configuration, plaid_api},
    models::{
        CountryCode, LinkTokenCreateRequest, LinkTokenCreateRequestUser, Products,
        TransactionsGetRequest,
    },
};
use reqwest::header::HeaderMap;
use rouille::{router, Request, Response, Server};
use serde::Serialize;
use std::{
    collections::HashMap,
    env,
    error::Error,
    ops::Deref,
    sync::{Arc, Mutex},
    time::Duration,
};

/// Produces a partial Bonk ledger by importing transactions from Plaid.
#[derive(Parser, Debug)]
#[command()]
struct Args {}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let config = plaid_config()?;
    let access_token = plaid_get_access_token(&config)?;

    let mut transactions = plaid_get_transactions(&config, &access_token)?;
    transactions.sort_by(
        |PlaidTransaction { date: date_a, .. }, PlaidTransaction { date: date_b, .. }| {
            date_a.cmp(date_b)
        },
    );
    let transactions = transactions
        .into_iter()
        .map(
            |PlaidTransaction {
                 account,
                 amount,
                 date,
                 name,
             }| {
                Transaction {
                    date: NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap(),
                    description: name,
                    postings: vec![Posting {
                        account: Account {
                            path: vec![account.to_lowercase().replace(' ', "_")],
                        },
                        amount: Amount {
                            cents: (amount * 100.0) as i32,
                        },
                    }],
                }
            },
        )
        .collect();

    let ledger = Ledger { transactions };

    println!("{}", ledger);

    Ok(())
}

fn plaid_config() -> Result<Configuration, Box<dyn Error>> {
    let base_path = env::var("PLAID_ENV")?;
    let client_id = env::var("PLAID_CLIENT_ID")?;
    let secret = env::var("PLAID_SECRET")?;
    let version = env::var("PLAID_VERSION")?;

    let mut headers = HeaderMap::new();
    headers.insert("Plaid-Client-Id", client_id.parse()?);
    headers.insert("Plaid-Secret", secret.parse()?);
    headers.insert("Plaid-Version", version.parse()?);

    Ok(Configuration {
        base_path,
        client: reqwest::Client::builder()
            .default_headers(headers)
            .build()?,
        ..Default::default()
    })
}

const HTML_TEMPLATE: &str = include_str!("index.html");

fn handle_get(_request: &Request, link_token: &str) -> Response {
    Response::html(HTML_TEMPLATE.replace("$$LINK_TOKEN$$", link_token))
}

fn handle_post(request: &Request, public_token: Arc<Mutex<Option<String>>>) -> Response {
    *public_token.lock().unwrap() = request.get_param("t");
    Response::empty_204()
}

fn plaid_get_access_token(config: &Configuration) -> Result<String, Box<dyn Error>> {
    let link_token = plaid_create_link_token(config)?;

    let public_token: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));

    let server = {
        let public_token = public_token.clone();
        Server::new("localhost:0", move |request| {
            let public_token = public_token.clone();
            router!(request,
                (GET) (/) => { handle_get(request, &link_token) },
                (POST) (/) => { handle_post(request, public_token) },
                _ => Response::empty_404()
            )
        })
        .unwrap()
    };

    webbrowser::open(&format!("http://localhost:{}", server.server_addr().port()))?;

    let public_token = loop {
        server.poll_timeout(Duration::from_millis(100));
        if let Some(public_token) = public_token.lock().unwrap().deref() {
            break public_token.clone();
        }
    };

    let access_token = plaid_exchange_public_token(config, &public_token)?;

    Ok(access_token)
}

fn plaid_create_link_token(config: &Configuration) -> Result<String, Box<dyn Error>> {
    Ok(plaid_api::link_token_create(
        config,
        LinkTokenCreateRequest {
            products: Some(Some(vec![Products::Auth, Products::Transactions])),
            ..LinkTokenCreateRequest::new(
                "finance-app".to_string(),
                "en".to_string(),
                vec![CountryCode::Us],
                LinkTokenCreateRequestUser::new("user-id".to_string()),
            )
        },
    )?
    .link_token)
}

fn plaid_exchange_public_token(
    config: &Configuration,
    public_token: &str,
) -> Result<String, Box<dyn Error>> {
    let access_token = plaid_api::item_public_token_exchange(
        config,
        plaid::models::ItemPublicTokenExchangeRequest {
            public_token: public_token.to_string(),
            client_id: None,
            secret: None,
        },
    )?
    .access_token;

    Ok(access_token)
}

#[derive(Serialize)]
struct PlaidTransaction {
    account: String,
    amount: f64,
    date: String,
    name: String,
}

fn plaid_get_transactions(
    config: &Configuration,
    access_token: &str,
) -> Result<Vec<PlaidTransaction>, Box<dyn Error>> {
    let response = plaid_api::transactions_get(
        config,
        TransactionsGetRequest {
            access_token: access_token.to_string(),
            start_date: "2023-01-01".to_string(),
            end_date: "2023-11-20".to_string(),
            client_id: None,
            options: None,
            secret: None,
        },
    )?;

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
