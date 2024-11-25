/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ConsumerDisputeCategory : Type of data being disputed by the consumer

/// Type of data being disputed by the consumer
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConsumerDisputeCategory {
    #[serde(rename = "TRANSACTION")]
    Transaction,
    #[serde(rename = "BALANCE")]
    Balance,
    #[serde(rename = "IDENTITY")]
    Identity,
    #[serde(rename = "OTHER")]
    Other,

}

impl ToString for ConsumerDisputeCategory {
    fn to_string(&self) -> String {
        match self {
            Self::Transaction => String::from("TRANSACTION"),
            Self::Balance => String::from("BALANCE"),
            Self::Identity => String::from("IDENTITY"),
            Self::Other => String::from("OTHER"),
        }
    }
}

impl Default for ConsumerDisputeCategory {
    fn default() -> ConsumerDisputeCategory {
        Self::Transaction
    }
}




