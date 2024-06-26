/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Originator : Originator and their status.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Originator {
    /// Originator’s client ID.
    #[serde(rename = "client_id")]
    pub client_id: String,
    #[serde(rename = "transfer_diligence_status")]
    pub transfer_diligence_status: crate::models::TransferDiligenceStatus,
}

impl Originator {
    /// Originator and their status.
    pub fn new(client_id: String, transfer_diligence_status: crate::models::TransferDiligenceStatus) -> Originator {
        Originator {
            client_id,
            transfer_diligence_status,
        }
    }
}


