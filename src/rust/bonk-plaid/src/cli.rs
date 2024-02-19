use crate::plaid_config;
use crate::plaid_get_access_token;
use crate::plaid_get_transactions;
use crate::PlaidTransaction;
use bonk_ast_errorless::{Account, Amount, Date, Ledger, Posting, Transaction};
use clap::Parser;
use std::{error::Error, fs, path::PathBuf};

/// Produces a partial Bonk ledger by converting transactions fetched via Plaid.
#[derive(Parser, Debug)]
#[command()]
pub struct Args {
    /// The earliest transaction date (e.g., "2023-01-01").
    #[arg(short, long)]
    pub start_date: String,

    /// The latest transaction date (e.g., "2023-12-31").
    #[arg(short, long)]
    pub end_date: String,

    /// The Bonk account to associate the converted transactions to (e.g., "assets:my_checking").
    #[arg(short, long)]
    pub account: String,

    /// The path to output the ledger to (e.g., "./foo.partial.bonk").
    #[arg(short, long)]
    pub output: PathBuf,
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let Args {
        start_date,
        end_date,
        account,
        output,
    } = args;

    let account = Account::parse(&account, None);

    let config = plaid_config()?;
    let access_token = plaid_get_access_token(&config)?;

    println!("Got access token");

    let mut transactions = plaid_get_transactions(&config, &access_token, &start_date, &end_date)?;
    transactions.sort_by(
        |PlaidTransaction { date: date_a, .. }, PlaidTransaction { date: date_b, .. }| {
            date_a.cmp(date_b)
        },
    );
    let transactions = transactions
        .into_iter()
        .map(|PlaidTransaction { amount, date, name }| Transaction {
            date: Date::parse(&date, None).unwrap(),
            description: name,
            postings: vec![Posting {
                account: account.clone(),
                amount: Amount::from_dollars(amount, None),
                source: None,
            }],
            source: None,
        })
        .collect();

    let ledger = Ledger {
        declare_accounts: vec![],
        transactions,
        source: None,
    };

    fs::write(&output, ledger.to_string())?;

    println!("Ledger written to {}", output.display());

    Ok(())
}
