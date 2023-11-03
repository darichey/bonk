/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WalletTransactionCounterparty : An object representing the e-wallet transaction's counterparty



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WalletTransactionCounterparty {
    /// The name of the counterparty
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "numbers")]
    pub numbers: crate::models::WalletTransactionCounterpartyNumbers,
    #[serde(rename = "address", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub address: Option<Option<crate::models::PaymentInitiationAddress>>,
    /// The counterparty's birthdate, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format.
    #[serde(rename = "date_of_birth", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<Option<String>>,
}

impl WalletTransactionCounterparty {
    /// An object representing the e-wallet transaction's counterparty
    pub fn new(name: String, numbers: crate::models::WalletTransactionCounterpartyNumbers) -> WalletTransactionCounterparty {
        WalletTransactionCounterparty {
            name,
            numbers,
            address: None,
            date_of_birth: None,
        }
    }
}


