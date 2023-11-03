/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkTokenCreateRequestIncomeVerificationPayrollIncome : Specifies options for initializing Link for use with Payroll Income (including Document Income). Further customization options for Document Income, such as customizing which document types may be uploaded, are also available via the [Link Customization pane](https://dashboard.plaid.com/link) in the Dashboard. (Requires Production enablement.)



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestIncomeVerificationPayrollIncome {
    /// The types of payroll income verification to enable for the Link session. If none are specified, then users will see both document and digital payroll income.
    #[serde(rename = "flow_types", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub flow_types: Option<Option<Vec<crate::models::IncomeVerificationPayrollFlowType>>>,
    /// An identifier to indicate whether the income verification Link token will be used for update mode. This field is only relevant for participants in the Payroll Income Refresh beta.
    #[serde(rename = "is_update_mode", skip_serializing_if = "Option::is_none")]
    pub is_update_mode: Option<bool>,
    /// Uniquely identify a payroll income Item to update with.  This field is only relevant for participants in the Payroll Income Refresh beta.
    #[serde(rename = "item_id_to_update", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub item_id_to_update: Option<Option<String>>,
    /// The types of analysis to enable for document uploads. If none are specified, then docs will undergo OCR parsing only. This field is only relevant to participants in the Document Fraud beta.
    #[serde(rename = "parsing_config", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parsing_config: Option<Option<Vec<crate::models::IncomeVerificationDocParsingConfig>>>,
}

impl LinkTokenCreateRequestIncomeVerificationPayrollIncome {
    /// Specifies options for initializing Link for use with Payroll Income (including Document Income). Further customization options for Document Income, such as customizing which document types may be uploaded, are also available via the [Link Customization pane](https://dashboard.plaid.com/link) in the Dashboard. (Requires Production enablement.)
    pub fn new() -> LinkTokenCreateRequestIncomeVerificationPayrollIncome {
        LinkTokenCreateRequestIncomeVerificationPayrollIncome {
            flow_types: None,
            is_update_mode: None,
            item_id_to_update: None,
            parsing_config: None,
        }
    }
}


