pub mod cli;
mod get_transactions;
mod query_transactions;

use bonk_db::Db;

use serde::Serialize;

struct State {
    db: Db,
}

/// A wrapper for seralizing sqlite Values
pub struct SqlValue(sqlite::Value);

impl Serialize for SqlValue {
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
