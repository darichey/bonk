/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferConfigurationGetResponse : Defines the response schema for `/transfer/configuration/get`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferConfigurationGetResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
    /// The max limit of dollar amount of a single transfer (decimal string with two digits of precision e.g. \"10.00\").
    #[serde(rename = "max_single_transfer_amount")]
    pub max_single_transfer_amount: String,
    /// The max limit of dollar amount of a single credit transfer (decimal string with two digits of precision e.g. \"10.00\").
    #[serde(rename = "max_single_transfer_credit_amount")]
    pub max_single_transfer_credit_amount: String,
    /// The max limit of dollar amount of a single debit transfer (decimal string with two digits of precision e.g. \"10.00\").
    #[serde(rename = "max_single_transfer_debit_amount")]
    pub max_single_transfer_debit_amount: String,
    /// The max limit of sum of dollar amount of credit transfers in last 24 hours (decimal string with two digits of precision e.g. \"10.00\").
    #[serde(rename = "max_daily_credit_amount")]
    pub max_daily_credit_amount: String,
    /// The max limit of sum of dollar amount of debit transfers in last 24 hours (decimal string with two digits of precision e.g. \"10.00\").
    #[serde(rename = "max_daily_debit_amount")]
    pub max_daily_debit_amount: String,
    /// The max limit of sum of dollar amount of credit and debit transfers in one calendar month (decimal string with two digits of precision e.g. \"10.00\").
    #[serde(rename = "max_monthly_amount")]
    pub max_monthly_amount: String,
    /// The max limit of sum of dollar amount of credit transfers in one calendar month (decimal string with two digits of precision e.g. \"10.00\").
    #[serde(rename = "max_monthly_credit_amount")]
    pub max_monthly_credit_amount: String,
    /// The max limit of sum of dollar amount of debit transfers in one calendar month (decimal string with two digits of precision e.g. \"10.00\").
    #[serde(rename = "max_monthly_debit_amount")]
    pub max_monthly_debit_amount: String,
    /// The currency of the dollar amount, e.g. \"USD\".
    #[serde(rename = "iso_currency_code")]
    pub iso_currency_code: String,
}

impl TransferConfigurationGetResponse {
    /// Defines the response schema for `/transfer/configuration/get`
    pub fn new(request_id: String, max_single_transfer_amount: String, max_single_transfer_credit_amount: String, max_single_transfer_debit_amount: String, max_daily_credit_amount: String, max_daily_debit_amount: String, max_monthly_amount: String, max_monthly_credit_amount: String, max_monthly_debit_amount: String, iso_currency_code: String) -> TransferConfigurationGetResponse {
        TransferConfigurationGetResponse {
            request_id,
            max_single_transfer_amount,
            max_single_transfer_credit_amount,
            max_single_transfer_debit_amount,
            max_daily_credit_amount,
            max_daily_debit_amount,
            max_monthly_amount,
            max_monthly_credit_amount,
            max_monthly_debit_amount,
            iso_currency_code,
        }
    }
}


