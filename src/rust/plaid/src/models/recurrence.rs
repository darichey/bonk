/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Recurrence : Insights relating to expenses and deposits that are predicted to occur on a scheduled basis, such as biweekly, monthly, or annually.  Common examples include loan payments, bill payments, subscriptions, and payroll income.  This is a beta field, available to all users.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Recurrence {
    /// Whether or not the transaction is periodically recurring.
    #[serde(rename = "is_recurring", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_recurring: Option<Option<bool>>,
    #[serde(rename = "frequency", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Option<crate::models::RecurringFrequency>>,
}

impl Recurrence {
    /// Insights relating to expenses and deposits that are predicted to occur on a scheduled basis, such as biweekly, monthly, or annually.  Common examples include loan payments, bill payments, subscriptions, and payroll income.  This is a beta field, available to all users.
    pub fn new() -> Recurrence {
        Recurrence {
            is_recurring: None,
            frequency: None,
        }
    }
}


