use crate::plaid_config;
use crate::plaid_get_access_token;
use crate::plaid_get_transactions;
use crate::PlaidTransaction;
use bonk_ast_errorless::{Account, Amount, Date, Ledger, Posting, Transaction};
use clap::Parser;
use std::{fs, path::PathBuf};

/// Produces a partial Bonk ledger by converting transactions fetched via Plaid.
#[derive(Parser, Debug)]
#[command()]
pub struct Args {
    /// The earliest transaction date (e.g., "2023-01-01").
    #[arg(long)]
    pub start_date: String,

    /// The latest transaction date (e.g., "2023-12-31").
    #[arg(long)]
    pub end_date: String,

    /// The Bonk account to associate the converted transactions to (e.g., "assets/my_checking").
    #[arg(long)]
    pub account: String,

    /// The path to output the ledger to (e.g., "./foo.partial.bonk").
    #[arg(long)]
    pub output: PathBuf,

    /// An existing Plaid access token to use.
    #[arg(long)]
    pub access_token: Option<String>,
}

pub fn run(args: Args) -> anyhow::Result<()> {
    let Args {
        start_date,
        end_date,
        account,
        output,
        access_token,
    } = args;

    let account = Account::parse(&account, None);

    let config = plaid_config()?;

    let access_token = match access_token {
        Some(access_token) => access_token,
        None => plaid_get_access_token(&config)?,
    };

    println!("Got access token: {access_token}");

    let transactions = plaid_get_transactions(&config, &access_token, &start_date, &end_date)?;

    println!("Fetched {} transactions", transactions.len());

    let transactions = transactions
        .into_iter()
        .map(
            |PlaidTransaction {
                 amount,
                 date,
                 description,
             }| Transaction {
                date: Date::parse(&date, None).unwrap(),
                description,
                postings: vec![
                    Posting {
                        account: account.clone(),
                        amount: Some(Amount::from_dollars(-amount, None)), // flip sign
                        source: None,
                    },
                    Posting {
                        // TODO: this should use a built-in constant shared from somewhere
                        account: Account {
                            path: vec!["todo".to_string()],
                            source: None,
                        },
                        amount: None,
                        source: None,
                    },
                ],
                source: None,
            },
        )
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
