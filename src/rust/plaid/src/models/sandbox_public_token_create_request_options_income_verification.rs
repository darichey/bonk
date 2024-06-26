/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SandboxPublicTokenCreateRequestOptionsIncomeVerification : A set of parameters for income verification options. This field is required if `income_verification` is included in the `initial_products` array.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SandboxPublicTokenCreateRequestOptionsIncomeVerification {
    /// The types of source income data that users will be permitted to share. Options include `bank` and `payroll`. Currently you can only specify one of these options.
    #[serde(rename = "income_source_types", skip_serializing_if = "Option::is_none")]
    pub income_source_types: Option<Vec<crate::models::IncomeVerificationSourceType>>,
    #[serde(rename = "bank_income", skip_serializing_if = "Option::is_none")]
    pub bank_income: Option<Box<crate::models::SandboxPublicTokenCreateRequestIncomeVerificationBankIncome>>,
}

impl SandboxPublicTokenCreateRequestOptionsIncomeVerification {
    /// A set of parameters for income verification options. This field is required if `income_verification` is included in the `initial_products` array.
    pub fn new() -> SandboxPublicTokenCreateRequestOptionsIncomeVerification {
        SandboxPublicTokenCreateRequestOptionsIncomeVerification {
            income_source_types: None,
            bank_income: None,
        }
    }
}


