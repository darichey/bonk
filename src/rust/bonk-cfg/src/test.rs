use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Test {
    pub name: String,
    pub query: String,
}
