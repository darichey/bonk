/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionsRulesRemoveResponse : TransactionsRulesRemoveResponse defines the response schema for `/beta/transactions/rules/v1/remove`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransactionsRulesRemoveResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl TransactionsRulesRemoveResponse {
    /// TransactionsRulesRemoveResponse defines the response schema for `/beta/transactions/rules/v1/remove`
    pub fn new(request_id: String) -> TransactionsRulesRemoveResponse {
        TransactionsRulesRemoveResponse {
            request_id,
        }
    }
}


