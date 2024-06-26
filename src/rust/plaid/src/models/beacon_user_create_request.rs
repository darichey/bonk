/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BeaconUserCreateRequest : Request input for creating a Beacon User.  The primary use for this endpoint is to add a new end user to Beacon for fraud and duplicate scanning. This endpoint can also be used to import historical fraud cases into the Beacon Network without being charged for creating a Beacon User. To import historical fraud cases, embed the fraud report in the optional `report` section of the request payload.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BeaconUserCreateRequest {
    /// ID of the associated Beacon Program.
    #[serde(rename = "program_id")]
    pub program_id: String,
    /// A unique ID that identifies the end user in your system. This ID can also be used to associate user-specific data from other Plaid products. Financial Account Matching requires this field and the `/link/token/create` `client_user_id` to be consistent. Personally identifiable information, such as an email address or phone number, should not be used in the `client_user_id`.
    #[serde(rename = "client_user_id")]
    pub client_user_id: String,
    #[serde(rename = "user")]
    pub user: crate::models::BeaconUserRequestData,
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

impl BeaconUserCreateRequest {
    /// Request input for creating a Beacon User.  The primary use for this endpoint is to add a new end user to Beacon for fraud and duplicate scanning. This endpoint can also be used to import historical fraud cases into the Beacon Network without being charged for creating a Beacon User. To import historical fraud cases, embed the fraud report in the optional `report` section of the request payload.
    pub fn new(program_id: String, client_user_id: String, user: crate::models::BeaconUserRequestData) -> BeaconUserCreateRequest {
        BeaconUserCreateRequest {
            program_id,
            client_user_id,
            user,
            client_id: None,
            secret: None,
        }
    }
}


