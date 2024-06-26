/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkTokenCreateInstitutionData : A map containing data used to highlight institutions in Link.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkTokenCreateInstitutionData {
    /// The routing number of the bank to highlight.
    #[serde(rename = "routing_number", skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}

impl LinkTokenCreateInstitutionData {
    /// A map containing data used to highlight institutions in Link.
    pub fn new() -> LinkTokenCreateInstitutionData {
        LinkTokenCreateInstitutionData {
            routing_number: None,
        }
    }
}


