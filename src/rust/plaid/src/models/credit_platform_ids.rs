/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditPlatformIds : The object containing a set of ids related to an employee.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreditPlatformIds {
    /// The ID of an employee as given by their employer.
    #[serde(rename = "employee_id", deserialize_with = "Option::deserialize")]
    pub employee_id: Option<String>,
    /// The ID of an employee as given by their payroll.
    #[serde(rename = "payroll_id", deserialize_with = "Option::deserialize")]
    pub payroll_id: Option<String>,
    /// The ID of the position of the employee.
    #[serde(rename = "position_id", deserialize_with = "Option::deserialize")]
    pub position_id: Option<String>,
}

impl CreditPlatformIds {
    /// The object containing a set of ids related to an employee.
    pub fn new(employee_id: Option<String>, payroll_id: Option<String>, position_id: Option<String>) -> CreditPlatformIds {
        CreditPlatformIds {
            employee_id,
            payroll_id,
            position_id,
        }
    }
}


