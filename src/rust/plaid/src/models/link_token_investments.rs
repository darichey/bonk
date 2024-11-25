/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkTokenInvestments : Configuration parameters for the Investments product



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LinkTokenInvestments {
    /// If `true`, allow self-custody crypto wallets to be added without requiring signature verification. Defaults to `false`.
    #[serde(rename = "allow_unverified_crypto_wallets", skip_serializing_if = "Option::is_none")]
    pub allow_unverified_crypto_wallets: Option<bool>,
    /// If `true`, allow users to manually enter Investments account and holdings information. Defaults to `false`.
    #[serde(rename = "allow_manual_entry", skip_serializing_if = "Option::is_none")]
    pub allow_manual_entry: Option<bool>,
}

impl LinkTokenInvestments {
    /// Configuration parameters for the Investments product
    pub fn new() -> LinkTokenInvestments {
        LinkTokenInvestments {
            allow_unverified_crypto_wallets: None,
            allow_manual_entry: None,
        }
    }
}


