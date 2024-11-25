/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaystubOverrideNetPay : An object representing information about the net pay amount on the paystub.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaystubOverrideNetPay {
    /// Description of the net pay
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// The ISO-4217 currency code of the net pay.
    #[serde(rename = "currency", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub currency: Option<Option<String>>,
    /// The year-to-date amount of the net pay
    #[serde(rename = "ytd_amount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ytd_amount: Option<Option<f64>>,
}

impl PaystubOverrideNetPay {
    /// An object representing information about the net pay amount on the paystub.
    pub fn new() -> PaystubOverrideNetPay {
        PaystubOverrideNetPay {
            description: None,
            currency: None,
            ytd_amount: None,
        }
    }
}


