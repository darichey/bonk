/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TaxpayerId : Taxpayer ID of the individual receiving the paystub.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TaxpayerId {
    /// Type of ID, e.g. 'SSN'
    #[serde(rename = "id_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id_type: Option<Option<String>>,
    /// ID mask; i.e. last 4 digits of the taxpayer ID
    #[serde(rename = "id_mask", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id_mask: Option<Option<String>>,
    /// Last 4 digits of unique number of ID.
    #[serde(rename = "last_4_digits", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_4_digits: Option<Option<String>>,
}

impl TaxpayerId {
    /// Taxpayer ID of the individual receiving the paystub.
    pub fn new() -> TaxpayerId {
        TaxpayerId {
            id_type: None,
            id_mask: None,
            last_4_digits: None,
        }
    }
}


