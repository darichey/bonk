/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkTokenCreateRequestIncomeVerificationBankIncome : Specifies options for initializing Link for use with Bank Income. This field is required if `income_verification` is included in the `products` array and `bank` is specified in `income_source_types`.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestIncomeVerificationBankIncome {
    /// The number of days of data to request for the Bank Income product
    #[serde(rename = "days_requested")]
    pub days_requested: i32,
    /// Whether to enable multiple Items to be added in the Link session
    #[serde(rename = "enable_multiple_items", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub enable_multiple_items: Option<Option<bool>>,
}

impl LinkTokenCreateRequestIncomeVerificationBankIncome {
    /// Specifies options for initializing Link for use with Bank Income. This field is required if `income_verification` is included in the `products` array and `bank` is specified in `income_source_types`.
    pub fn new(days_requested: i32) -> LinkTokenCreateRequestIncomeVerificationBankIncome {
        LinkTokenCreateRequestIncomeVerificationBankIncome {
            days_requested,
            enable_multiple_items: None,
        }
    }
}


