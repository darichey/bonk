/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InstitutionSupportedNetworks : Contains the RTP network and types supported by the linked Item's institution.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstitutionSupportedNetworks {
    #[serde(rename = "rtp")]
    pub rtp: crate::models::TransferCapabilitiesGetRtp,
}

impl InstitutionSupportedNetworks {
    /// Contains the RTP network and types supported by the linked Item's institution.
    pub fn new(rtp: crate::models::TransferCapabilitiesGetRtp) -> InstitutionSupportedNetworks {
        InstitutionSupportedNetworks {
            rtp,
        }
    }
}


