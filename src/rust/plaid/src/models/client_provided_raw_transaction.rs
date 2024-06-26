/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ClientProvidedRawTransaction : A client-provided transaction for Plaid to enhance.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientProvidedRawTransaction {
    /// A unique ID for the transaction used to help you tie data back to your systems.
    #[serde(rename = "id")]
    pub id: String,
    /// The raw description of the transaction.
    #[serde(rename = "description")]
    pub description: String,
    /// The value of the transaction with direction. (NOTE: this will affect enrichment results, so directions are important):.   Negative (-) for credits (e.g., incoming transfers, refunds)   Positive (+) for debits (e.g., purchases, fees, outgoing transfers)
    #[serde(rename = "amount")]
    pub amount: f64,
    /// The ISO-4217 currency code of the transaction e.g. USD.
    #[serde(rename = "iso_currency_code")]
    pub iso_currency_code: String,
}

impl ClientProvidedRawTransaction {
    /// A client-provided transaction for Plaid to enhance.
    pub fn new(id: String, description: String, amount: f64, iso_currency_code: String) -> ClientProvidedRawTransaction {
        ClientProvidedRawTransaction {
            id,
            description,
            amount,
            iso_currency_code,
        }
    }
}


