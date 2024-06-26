/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferOriginatorFundingAccountUpdateRequest : Defines the request schema for `/transfer/originator/funding_account/update`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferOriginatorFundingAccountUpdateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The Plaid client ID of the transfer originator.
    #[serde(rename = "originator_client_id")]
    pub originator_client_id: String,
    #[serde(rename = "funding_account")]
    pub funding_account: Box<crate::models::TransferFundingAccount>,
}

impl TransferOriginatorFundingAccountUpdateRequest {
    /// Defines the request schema for `/transfer/originator/funding_account/update`
    pub fn new(originator_client_id: String, funding_account: crate::models::TransferFundingAccount) -> TransferOriginatorFundingAccountUpdateRequest {
        TransferOriginatorFundingAccountUpdateRequest {
            client_id: None,
            secret: None,
            originator_client_id,
            funding_account: Box::new(funding_account),
        }
    }
}


