/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditFreddieMacIndividualName : Documentation not found in the MISMO model viewer and not provided by Freddie Mac.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreditFreddieMacIndividualName {
    /// The first name of the individual represented by the parent object.
    #[serde(rename = "FirstName")]
    pub first_name: String,
    /// The last name of the individual represented by the parent object.
    #[serde(rename = "LastName")]
    pub last_name: String,
    /// The middle name of the individual represented by the parent object.
    #[serde(rename = "MiddleName")]
    pub middle_name: String,
}

impl CreditFreddieMacIndividualName {
    /// Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    pub fn new(first_name: String, last_name: String, middle_name: String) -> CreditFreddieMacIndividualName {
        CreditFreddieMacIndividualName {
            first_name,
            last_name,
            middle_name,
        }
    }
}


