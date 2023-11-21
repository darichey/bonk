/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ProcessorStripeBankAccountTokenCreateResponse : ProcessorStripeBankAccountTokenCreateResponse defines the response schema for `/processor/stripe/bank_account/create`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProcessorStripeBankAccountTokenCreateResponse {
    /// A token that can be sent to Stripe for use in making API calls to Plaid
    #[serde(rename = "stripe_bank_account_token")]
    pub stripe_bank_account_token: String,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl ProcessorStripeBankAccountTokenCreateResponse {
    /// ProcessorStripeBankAccountTokenCreateResponse defines the response schema for `/processor/stripe/bank_account/create`
    pub fn new(stripe_bank_account_token: String, request_id: String) -> ProcessorStripeBankAccountTokenCreateResponse {
        ProcessorStripeBankAccountTokenCreateResponse {
            stripe_bank_account_token,
            request_id,
        }
    }
}

