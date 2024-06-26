/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferFailure : The failure reason if the event type for a transfer is `\"failed\"` or `\"returned\"`. Null value otherwise.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferFailure {
    /// The ACH return code, e.g. `R01`.  A return code will be provided if and only if the transfer status is `returned`. For a full listing of ACH return codes, see [Transfer errors](https://plaid.com/docs/errors/transfer/#ach-return-codes).
    #[serde(rename = "ach_return_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ach_return_code: Option<Option<String>>,
    /// A human-readable description of the reason for the failure or reversal.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl TransferFailure {
    /// The failure reason if the event type for a transfer is `\"failed\"` or `\"returned\"`. Null value otherwise.
    pub fn new() -> TransferFailure {
        TransferFailure {
            ach_return_code: None,
            description: None,
        }
    }
}


