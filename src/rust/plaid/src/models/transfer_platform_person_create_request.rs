/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferPlatformPersonCreateRequest : Defines the response schema for `/transfer/platform/person/create`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferPlatformPersonCreateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The client ID of the originator
    #[serde(rename = "originator_client_id")]
    pub originator_client_id: String,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<crate::models::TransferPlatformPersonName>>,
    /// A valid email address. Must not have leading or trailing spaces.
    #[serde(rename = "email_address", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// A valid phone number in E.164 format.
    #[serde(rename = "phone_number", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<Box<crate::models::TransferPlatformPersonAddress>>,
    #[serde(rename = "id_number", skip_serializing_if = "Option::is_none")]
    pub id_number: Option<Box<crate::models::TransferPlatformPersonIdNumber>>,
    /// The date of birth of the person. Formatted as YYYY-MM-DD.
    #[serde(rename = "date_of_birth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// The relationship between this person and the originator they are related to.
    #[serde(rename = "relationship_to_originator", skip_serializing_if = "Option::is_none")]
    pub relationship_to_originator: Option<String>,
}

impl TransferPlatformPersonCreateRequest {
    /// Defines the response schema for `/transfer/platform/person/create`
    pub fn new(originator_client_id: String) -> TransferPlatformPersonCreateRequest {
        TransferPlatformPersonCreateRequest {
            client_id: None,
            secret: None,
            originator_client_id,
            name: None,
            email_address: None,
            phone_number: None,
            address: None,
            id_number: None,
            date_of_birth: None,
            relationship_to_originator: None,
        }
    }
}


