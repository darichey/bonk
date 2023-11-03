/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// KycCheckDetails : Additional information for the `kyc_check` step. This field will be `null` unless `steps.kyc_check` has reached a terminal state of either `success` or `failed`.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct KycCheckDetails {
    /// The outcome status for the associated Identity Verification attempt's `kyc_check` step. This field will always have the same value as `steps.kyc_check`.
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "address")]
    pub address: crate::models::KycCheckAddressSummary,
    #[serde(rename = "name")]
    pub name: crate::models::KycCheckNameSummary,
    #[serde(rename = "date_of_birth")]
    pub date_of_birth: crate::models::KycCheckDateOfBirthSummary,
    #[serde(rename = "id_number")]
    pub id_number: crate::models::KycCheckIdNumberSummary,
    #[serde(rename = "phone_number")]
    pub phone_number: crate::models::KycCheckPhoneSummary,
}

impl KycCheckDetails {
    /// Additional information for the `kyc_check` step. This field will be `null` unless `steps.kyc_check` has reached a terminal state of either `success` or `failed`.
    pub fn new(status: String, address: crate::models::KycCheckAddressSummary, name: crate::models::KycCheckNameSummary, date_of_birth: crate::models::KycCheckDateOfBirthSummary, id_number: crate::models::KycCheckIdNumberSummary, phone_number: crate::models::KycCheckPhoneSummary) -> KycCheckDetails {
        KycCheckDetails {
            status,
            address,
            name,
            date_of_birth,
            id_number,
            phone_number,
        }
    }
}


