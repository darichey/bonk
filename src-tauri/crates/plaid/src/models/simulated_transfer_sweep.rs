/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SimulatedTransferSweep : A sweep returned from the `/sandbox/transfer/sweep/simulate` endpoint. Can be null if there are no transfers to include in a sweep.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimulatedTransferSweep {
    /// Identifier of the sweep.
    #[serde(rename = "id")]
    pub id: String,
    /// The id of the funding account to use, available in the Plaid Dashboard. This determines which of your business checking accounts will be credited or debited.
    #[serde(rename = "funding_account_id")]
    pub funding_account_id: String,
    /// The datetime when the sweep occurred, in RFC 3339 format.
    #[serde(rename = "created")]
    pub created: String,
    /// Signed decimal amount of the sweep as it appears on your sweep account ledger (e.g. \"-10.00\")  If amount is not present, the sweep was net-settled to zero and outstanding debits and credits between the sweep account and Plaid are balanced.
    #[serde(rename = "amount")]
    pub amount: String,
    /// The currency of the sweep, e.g. \"USD\".
    #[serde(rename = "iso_currency_code")]
    pub iso_currency_code: String,
    /// The date when the sweep settled, in the YYYY-MM-DD format.
    #[serde(rename = "settled", deserialize_with = "Option::deserialize")]
    pub settled: Option<String>,
    #[serde(rename = "status", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<crate::models::SweepStatus>>,
    #[serde(rename = "trigger", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub trigger: Option<Option<crate::models::SweepTrigger>>,
    /// The description of the deposit that will be passed to the receiving bank (up to 10 characters). Note that banks utilize this field differently, and may or may not show it on the bank statement.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The trace identifier for the transfer based on its network. This will only be set after the transfer has posted.  For `ach` or `same-day-ach` transfers, this is the ACH trace number. Currently, the field will remain null for transfers on other rails.
    #[serde(rename = "network_trace_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub network_trace_id: Option<Option<String>>,
}

impl SimulatedTransferSweep {
    /// A sweep returned from the `/sandbox/transfer/sweep/simulate` endpoint. Can be null if there are no transfers to include in a sweep.
    pub fn new(id: String, funding_account_id: String, created: String, amount: String, iso_currency_code: String, settled: Option<String>) -> SimulatedTransferSweep {
        SimulatedTransferSweep {
            id,
            funding_account_id,
            created,
            amount,
            iso_currency_code,
            settled,
            status: None,
            trigger: None,
            description: None,
            network_trace_id: None,
        }
    }
}


