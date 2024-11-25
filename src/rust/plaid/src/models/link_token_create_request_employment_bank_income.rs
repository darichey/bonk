/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkTokenCreateRequestEmploymentBankIncome : Specifies options for initializing Link for use with Bank Employment. This field is required if `employment` is included in the `products` array and `bank` is specified in `employment_source_types`.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestEmploymentBankIncome {
    /// The number of days of data to request for the Bank Employment product.
    #[serde(rename = "days_requested")]
    pub days_requested: i32,
}

impl LinkTokenCreateRequestEmploymentBankIncome {
    /// Specifies options for initializing Link for use with Bank Employment. This field is required if `employment` is included in the `products` array and `bank` is specified in `employment_source_types`.
    pub fn new(days_requested: i32) -> LinkTokenCreateRequestEmploymentBankIncome {
        LinkTokenCreateRequestEmploymentBankIncome {
            days_requested,
        }
    }
}


