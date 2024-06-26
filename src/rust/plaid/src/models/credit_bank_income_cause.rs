/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditBankIncomeCause : An error object and associated `item_id` used to identify a specific Item and error when a batch operation operating on multiple Items has encountered an error in one of the Items.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditBankIncomeCause {
    #[serde(rename = "error_type")]
    pub error_type: crate::models::CreditBankIncomeErrorType,
    /// We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues. Error fields will be `null` if no error has occurred.
    #[serde(rename = "error_code")]
    pub error_code: String,
    /// A developer-friendly representation of the error code. This may change over time and is not safe for programmatic use.
    #[serde(rename = "error_message")]
    pub error_message: String,
    /// A user-friendly representation of the error code. null if the error is not related to user action. This may change over time and is not safe for programmatic use.
    #[serde(rename = "display_message")]
    pub display_message: String,
    /// The `item_id` of the Item associated with this warning.
    #[serde(rename = "item_id")]
    pub item_id: String,
}

impl CreditBankIncomeCause {
    /// An error object and associated `item_id` used to identify a specific Item and error when a batch operation operating on multiple Items has encountered an error in one of the Items.
    pub fn new(error_type: crate::models::CreditBankIncomeErrorType, error_code: String, error_message: String, display_message: String, item_id: String) -> CreditBankIncomeCause {
        CreditBankIncomeCause {
            error_type,
            error_code,
            error_message,
            display_message,
            item_id,
        }
    }
}


