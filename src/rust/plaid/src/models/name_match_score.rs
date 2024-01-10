/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NameMatchScore : Score found by matching name provided by the API with the name on the account at the financial institution. If the account contains multiple owners, the maximum match score is filled.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameMatchScore {
    /// Match score for name. 100 is a perfect score, 99-85 means a strong match, 84-70 is a partial match, any score less than 70 is a mismatch. Typically, the match threshold should be set to a score of 70 or higher. If the name is missing from either the API or financial institution, this is null.
    #[serde(rename = "score", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub score: Option<Option<i32>>,
    /// first or last name completely matched, likely a family member
    #[serde(rename = "is_first_name_or_last_name_match", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_first_name_or_last_name_match: Option<Option<bool>>,
    /// nickname matched, example Jennifer and Jenn.
    #[serde(rename = "is_nickname_match", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_nickname_match: Option<Option<bool>>,
    /// Is `true` if the name on either of the names that was matched for the score contained strings indicative of a business name, such as \"CORP\", \"LLC\", \"INC\", or \"LTD\". A `true` result generally indicates the entity is a business. However, a `false` result does not mean the entity is not a business, as some businesses do not use these strings in the names used for their financial institution accounts.
    #[serde(rename = "is_business_name_detected", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_business_name_detected: Option<Option<bool>>,
}

impl NameMatchScore {
    /// Score found by matching name provided by the API with the name on the account at the financial institution. If the account contains multiple owners, the maximum match score is filled.
    pub fn new() -> NameMatchScore {
        NameMatchScore {
            score: None,
            is_first_name_or_last_name_match: None,
            is_nickname_match: None,
            is_business_name_detected: None,
        }
    }
}

