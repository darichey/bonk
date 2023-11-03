/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InstitutionsSearchRequestOptions : An optional object to filter `/institutions/search` results.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InstitutionsSearchRequestOptions {
    /// Limit results to institutions with or without OAuth login flows. Note that institutions will have `oauth` set to `true` if some Items associated with that institution are required to use OAuth flows; institutions in a state of migration to OAuth will have the `oauth` attribute set to `true`.
    #[serde(rename = "oauth", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub oauth: Option<Option<bool>>,
    /// When true, return the institution's homepage URL, logo and primary brand color.
    #[serde(rename = "include_optional_metadata", skip_serializing_if = "Option::is_none")]
    pub include_optional_metadata: Option<bool>,
    /// When `true`, returns metadata related to the Auth product indicating which auth methods are supported.
    #[serde(rename = "include_auth_metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub include_auth_metadata: Option<Option<bool>>,
    /// When `true`, returns metadata related to the Payment Initiation product indicating which payment configurations are supported.
    #[serde(rename = "include_payment_initiation_metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub include_payment_initiation_metadata: Option<Option<bool>>,
    #[serde(rename = "payment_initiation", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub payment_initiation: Option<Option<Box<crate::models::InstitutionsSearchPaymentInitiationOptions>>>,
}

impl InstitutionsSearchRequestOptions {
    /// An optional object to filter `/institutions/search` results.
    pub fn new() -> InstitutionsSearchRequestOptions {
        InstitutionsSearchRequestOptions {
            oauth: None,
            include_optional_metadata: None,
            include_auth_metadata: None,
            include_payment_initiation_metadata: None,
            payment_initiation: None,
        }
    }
}


