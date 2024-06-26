/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EmployerVerification : An object containing employer data.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmployerVerification {
    /// Name of employer.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
}

impl EmployerVerification {
    /// An object containing employer data.
    pub fn new() -> EmployerVerification {
        EmployerVerification {
            name: None,
        }
    }
}


