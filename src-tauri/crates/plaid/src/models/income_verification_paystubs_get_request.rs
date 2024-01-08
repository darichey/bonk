/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IncomeVerificationPaystubsGetRequest : IncomeVerificationPaystubsGetRequest defines the request schema for `/income/verification/paystubs/get`.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncomeVerificationPaystubsGetRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The ID of the verification for which to get paystub information.
    #[serde(rename = "income_verification_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub income_verification_id: Option<Option<String>>,
    /// The access token associated with the Item data is being requested for.
    #[serde(rename = "access_token", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<Option<String>>,
}

impl IncomeVerificationPaystubsGetRequest {
    /// IncomeVerificationPaystubsGetRequest defines the request schema for `/income/verification/paystubs/get`.
    pub fn new() -> IncomeVerificationPaystubsGetRequest {
        IncomeVerificationPaystubsGetRequest {
            client_id: None,
            secret: None,
            income_verification_id: None,
            access_token: None,
        }
    }
}


