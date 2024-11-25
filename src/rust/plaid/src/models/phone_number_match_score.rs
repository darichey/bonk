/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PhoneNumberMatchScore : Score found by matching phone number provided by the API with the phone number on the account at the financial institution. 100 is a perfect match and 0 is a no match. If the account contains multiple owners, the maximum match score is filled.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PhoneNumberMatchScore {
    /// Match score for normalized phone number. 100 is a perfect match, 99-70 is a partial match (matching the same phone number with extension against one without extension, etc.), anything below 70 is considered a mismatch. Typically, the match threshold should be set to a score of 70 or higher. If the phone number is missing from either the API or financial institution, this is null.
    #[serde(rename = "score", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub score: Option<Option<i32>>,
}

impl PhoneNumberMatchScore {
    /// Score found by matching phone number provided by the API with the phone number on the account at the financial institution. 100 is a perfect match and 0 is a no match. If the account contains multiple owners, the maximum match score is filled.
    pub fn new() -> PhoneNumberMatchScore {
        PhoneNumberMatchScore {
            score: None,
        }
    }
}


