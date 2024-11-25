/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BeaconAccountRiskEvaluateRequestOptions : An optional object to filter `/beacon/account_risk/v1/evaluate` results to a subset of the accounts on the linked Item.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BeaconAccountRiskEvaluateRequestOptions {
    /// An array of `account_ids` for the specific accounts to evaluate.
    #[serde(rename = "account_ids", skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
}

impl BeaconAccountRiskEvaluateRequestOptions {
    /// An optional object to filter `/beacon/account_risk/v1/evaluate` results to a subset of the accounts on the linked Item.
    pub fn new() -> BeaconAccountRiskEvaluateRequestOptions {
        BeaconAccountRiskEvaluateRequestOptions {
            account_ids: None,
        }
    }
}


