/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkTokenCreateRequestBaseReport : Specifies options for initializing Link for use with the Base Report product. This field is required if `assets` is included in the `products` array and the client is CRA-enabled.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestBaseReport {
    /// The maximum integer number of days of history to include in the Base Report.
    #[serde(rename = "days_requested")]
    pub days_requested: i32,
}

impl LinkTokenCreateRequestBaseReport {
    /// Specifies options for initializing Link for use with the Base Report product. This field is required if `assets` is included in the `products` array and the client is CRA-enabled.
    pub fn new(days_requested: i32) -> LinkTokenCreateRequestBaseReport {
        LinkTokenCreateRequestBaseReport {
            days_requested,
        }
    }
}


