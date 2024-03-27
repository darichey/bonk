use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Query {
    pub name: String,
    pub query: String,
}
