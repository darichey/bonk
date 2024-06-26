/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentInitiationRecipientGetResponse : PaymentInitiationRecipientGetResponse defines the response schema for `/payment_initiation/recipient/get`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentInitiationRecipientGetResponse {
    /// The ID of the recipient.
    #[serde(rename = "recipient_id")]
    pub recipient_id: String,
    /// The name of the recipient.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "address", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub address: Option<Option<crate::models::PaymentInitiationAddress>>,
    /// The International Bank Account Number (IBAN) for the recipient.
    #[serde(rename = "iban", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub iban: Option<Option<String>>,
    #[serde(rename = "bacs", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub bacs: Option<Option<Box<crate::models::RecipientBacsNullable>>>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl PaymentInitiationRecipientGetResponse {
    /// PaymentInitiationRecipientGetResponse defines the response schema for `/payment_initiation/recipient/get`
    pub fn new(recipient_id: String, name: String, request_id: String) -> PaymentInitiationRecipientGetResponse {
        PaymentInitiationRecipientGetResponse {
            recipient_id,
            name,
            address: None,
            iban: None,
            bacs: None,
            request_id,
        }
    }
}


