/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RiskReason : This object includes a code and description to describe medium risk transactions and above on /accounts/balance/get.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RiskReason {
    /// A code that represents the type of risk associated with the proposed transaction.  The codes are from PL01 to PL08 and from BK01 to BK07. For a full listing of risk reason codes, see [Risk codes](https://plaid.com/docs/balance/balance-plus/#risk-codes).
    #[serde(rename = "code")]
    pub code: String,
    /// A human-readable description explaining the risk code associated with the proposed transaction and some recommended actions. This field is subject to change; any programmatic logic should be based on the `code` field instead.
    #[serde(rename = "description")]
    pub description: String,
}

impl RiskReason {
    /// This object includes a code and description to describe medium risk transactions and above on /accounts/balance/get.
    pub fn new(code: String, description: String) -> RiskReason {
        RiskReason {
            code,
            description,
        }
    }
}

