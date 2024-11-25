/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PartnerEndCustomerAssetsUnderManagement : Assets under management for the given end customer. Required for end customers with monthly service commitments.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PartnerEndCustomerAssetsUnderManagement {
    #[serde(rename = "amount")]
    pub amount: f64,
    #[serde(rename = "iso_currency_code")]
    pub iso_currency_code: String,
}

impl PartnerEndCustomerAssetsUnderManagement {
    /// Assets under management for the given end customer. Required for end customers with monthly service commitments.
    pub fn new(amount: f64, iso_currency_code: String) -> PartnerEndCustomerAssetsUnderManagement {
        PartnerEndCustomerAssetsUnderManagement {
            amount,
            iso_currency_code,
        }
    }
}


