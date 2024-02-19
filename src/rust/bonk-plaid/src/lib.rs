pub mod cli;

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
    env,
    error::Error,
    ops::Deref,
    sync::{Arc, Mutex},
    time::Duration,
};

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

    let url = format!("http://localhost:{}", server.server_addr().port());
    println!("Open {url} to link with Plaid");
    webbrowser::open(&url)?;

    let public_token = loop {
        server.poll_timeout(Duration::from_millis(100));
        if let Some(public_token) = public_token.lock().unwrap().deref() {
            break public_token.clone();
        }
    };

    println!("Plaid link done");

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
    amount: f64,
    date: String,
    name: String,
}

fn plaid_get_transactions(
    config: &Configuration,
    access_token: &str,
    start_date: &str,
    end_date: &str,
) -> Result<Vec<PlaidTransaction>, Box<dyn Error>> {
    let response = plaid_api::transactions_get(
        config,
        TransactionsGetRequest {
            access_token: access_token.to_string(),
            start_date: start_date.to_string(),
            end_date: end_date.to_string(),
            client_id: None,
            options: None,
            secret: None,
        },
    )?;

    let transactions = response
        .transactions
        .into_iter()
        .map(|transaction| PlaidTransaction {
            amount: transaction.amount,
            date: transaction.date,
            name: transaction.name,
        })
        .collect();

    Ok(transactions)
}
