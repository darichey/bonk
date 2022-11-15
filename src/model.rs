#[derive(Debug, sqlx::Type)]
#[repr(transparent)]
struct AccountId(i64);

#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
struct Account {
    id: AccountId,
    name: String,
}

#[derive(Debug, sqlx::Type)]
#[repr(transparent)]
struct TransactionId(i64);

#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
struct Transaction {
    id: TransactionId,
    date: chrono::NaiveDate,
    description: String,
    source: AccountId,
    destination: AccountId,
    amount: u64,
}
