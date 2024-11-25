/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkTokenCreateRequestIncomeVerification : Specifies options for initializing Link for use with the Income product. This field is required if `income_verification` is included in the `products` array.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestIncomeVerification {
    /// The `income_verification_id` of the verification instance, as provided by `/income/verification/create`. Replaced by the user token.
    #[serde(rename = "income_verification_id", skip_serializing_if = "Option::is_none")]
    pub income_verification_id: Option<String>,
    /// The `asset_report_id` of an asset report associated with the user, as provided by `/asset_report/create`. Providing an `asset_report_id` is optional and can be used to verify the user through a streamlined flow. If provided, the bank linking flow will be skipped.
    #[serde(rename = "asset_report_id", skip_serializing_if = "Option::is_none")]
    pub asset_report_id: Option<String>,
    /// An array of access tokens corresponding to Items that a user has previously connected with. Data from these institutions will be cross-referenced with document data received during the Document Income flow to help verify that the uploaded documents are accurate. If the `transactions` product was not initialized for these Items during link, it will be initialized after this Link session.  This field should only be used with the `payroll` income source type.
    #[serde(rename = "access_tokens", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub access_tokens: Option<Option<Vec<String>>>,
    /// The types of source income data that users will be permitted to share. Options include `bank` and `payroll`. Currently you can only specify one of these options.
    #[serde(rename = "income_source_types", skip_serializing_if = "Option::is_none")]
    pub income_source_types: Option<Vec<crate::models::IncomeVerificationSourceType>>,
    #[serde(rename = "bank_income", skip_serializing_if = "Option::is_none")]
    pub bank_income: Option<Box<crate::models::LinkTokenCreateRequestIncomeVerificationBankIncome>>,
    #[serde(rename = "payroll_income", skip_serializing_if = "Option::is_none")]
    pub payroll_income: Option<Box<crate::models::LinkTokenCreateRequestIncomeVerificationPayrollIncome>>,
    /// A list of user stated income sources
    #[serde(rename = "stated_income_sources", skip_serializing_if = "Option::is_none")]
    pub stated_income_sources: Option<Vec<crate::models::LinkTokenCreateRequestUserStatedIncomeSource>>,
}

impl LinkTokenCreateRequestIncomeVerification {
    /// Specifies options for initializing Link for use with the Income product. This field is required if `income_verification` is included in the `products` array.
    pub fn new() -> LinkTokenCreateRequestIncomeVerification {
        LinkTokenCreateRequestIncomeVerification {
            income_verification_id: None,
            asset_report_id: None,
            access_tokens: None,
            income_source_types: None,
            bank_income: None,
            payroll_income: None,
            stated_income_sources: None,
        }
    }
}


