/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// W2 : W2 is an object that represents income data taken from a W2 tax document.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct W2 {
    #[serde(rename = "employer", skip_serializing_if = "Option::is_none")]
    pub employer: Option<crate::models::PaystubEmployer>,
    #[serde(rename = "employee", skip_serializing_if = "Option::is_none")]
    pub employee: Option<crate::models::Employee>,
    /// The tax year of the W2 document.
    #[serde(rename = "tax_year", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tax_year: Option<Option<String>>,
    /// An employee identification number or EIN.
    #[serde(rename = "employer_id_number", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub employer_id_number: Option<Option<String>>,
    /// Wages from tips and other compensation.
    #[serde(rename = "wages_tips_other_comp", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub wages_tips_other_comp: Option<Option<String>>,
    /// Federal income tax withheld for the tax year.
    #[serde(rename = "federal_income_tax_withheld", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub federal_income_tax_withheld: Option<Option<String>>,
    /// Wages from social security.
    #[serde(rename = "social_security_wages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub social_security_wages: Option<Option<String>>,
    /// Social security tax withheld for the tax year.
    #[serde(rename = "social_security_tax_withheld", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub social_security_tax_withheld: Option<Option<String>>,
    /// Wages and tips from medicare.
    #[serde(rename = "medicare_wages_and_tips", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub medicare_wages_and_tips: Option<Option<String>>,
    /// Medicare tax withheld for the tax year.
    #[serde(rename = "medicare_tax_withheld", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub medicare_tax_withheld: Option<Option<String>>,
    /// Tips from social security.
    #[serde(rename = "social_security_tips", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub social_security_tips: Option<Option<String>>,
    /// Allocated tips.
    #[serde(rename = "allocated_tips", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub allocated_tips: Option<Option<String>>,
    /// Contents from box 9 on the W2.
    #[serde(rename = "box_9", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub box_9: Option<Option<String>>,
    /// Dependent care benefits.
    #[serde(rename = "dependent_care_benefits", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub dependent_care_benefits: Option<Option<String>>,
    /// Nonqualified plans.
    #[serde(rename = "nonqualified_plans", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub nonqualified_plans: Option<Option<String>>,
    #[serde(rename = "box_12", skip_serializing_if = "Option::is_none")]
    pub box_12: Option<Vec<crate::models::W2Box12>>,
    /// Statutory employee.
    #[serde(rename = "statutory_employee", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub statutory_employee: Option<Option<String>>,
    /// Retirement plan.
    #[serde(rename = "retirement_plan", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub retirement_plan: Option<Option<String>>,
    /// Third party sick pay.
    #[serde(rename = "third_party_sick_pay", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub third_party_sick_pay: Option<Option<String>>,
    /// Other.
    #[serde(rename = "other", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub other: Option<Option<String>>,
    #[serde(rename = "state_and_local_wages", skip_serializing_if = "Option::is_none")]
    pub state_and_local_wages: Option<Vec<crate::models::W2StateAndLocalWages>>,
}

impl W2 {
    /// W2 is an object that represents income data taken from a W2 tax document.
    pub fn new() -> W2 {
        W2 {
            employer: None,
            employee: None,
            tax_year: None,
            employer_id_number: None,
            wages_tips_other_comp: None,
            federal_income_tax_withheld: None,
            social_security_wages: None,
            social_security_tax_withheld: None,
            medicare_wages_and_tips: None,
            medicare_tax_withheld: None,
            social_security_tips: None,
            allocated_tips: None,
            box_9: None,
            dependent_care_benefits: None,
            nonqualified_plans: None,
            box_12: None,
            statutory_employee: None,
            retirement_plan: None,
            third_party_sick_pay: None,
            other: None,
            state_and_local_wages: None,
        }
    }
}


