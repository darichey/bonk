/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IncomeVerificationPrecheckMilitaryInfo : Data about military info in the income verification precheck.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckMilitaryInfo {
    /// Is the user currently active duty in the US military
    #[serde(rename = "is_active_duty", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_active_duty: Option<Option<bool>>,
    /// If the user is currently serving in the US military, the branch of the military in which they are serving Valid values: 'AIR FORCE', 'ARMY', 'COAST GUARD', 'MARINES', 'NAVY', 'UNKNOWN'
    #[serde(rename = "branch", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub branch: Option<Option<String>>,
}

impl IncomeVerificationPrecheckMilitaryInfo {
    /// Data about military info in the income verification precheck.
    pub fn new() -> IncomeVerificationPrecheckMilitaryInfo {
        IncomeVerificationPrecheckMilitaryInfo {
            is_active_duty: None,
            branch: None,
        }
    }
}


