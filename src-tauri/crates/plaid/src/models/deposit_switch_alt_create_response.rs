/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DepositSwitchAltCreateResponse : DepositSwitchAltCreateResponse defines the response schema for `/deposit_switch/alt/create`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DepositSwitchAltCreateResponse {
    /// ID of the deposit switch. This ID is persisted throughout the lifetime of the deposit switch.
    #[serde(rename = "deposit_switch_id")]
    pub deposit_switch_id: String,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl DepositSwitchAltCreateResponse {
    /// DepositSwitchAltCreateResponse defines the response schema for `/deposit_switch/alt/create`
    pub fn new(deposit_switch_id: String, request_id: String) -> DepositSwitchAltCreateResponse {
        DepositSwitchAltCreateResponse {
            deposit_switch_id,
            request_id,
        }
    }
}


