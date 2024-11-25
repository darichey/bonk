/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BankTransferSweep : BankTransferSweep describes a sweep transfer.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BankTransferSweep {
    /// Identifier of the sweep.
    #[serde(rename = "id")]
    pub id: String,
    /// The datetime when the sweep occurred, in RFC 3339 format.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The amount of the sweep.
    #[serde(rename = "amount")]
    pub amount: String,
    /// The currency of the sweep, e.g. \"USD\".
    #[serde(rename = "iso_currency_code")]
    pub iso_currency_code: String,
}

impl BankTransferSweep {
    /// BankTransferSweep describes a sweep transfer.
    pub fn new(id: String, created_at: String, amount: String, iso_currency_code: String) -> BankTransferSweep {
        BankTransferSweep {
            id,
            created_at,
            amount,
            iso_currency_code,
        }
    }
}


