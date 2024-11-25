/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditFreddieMacAssetTransactions : Documentation not found in the MISMO model viewer and not provided by Freddie Mac.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreditFreddieMacAssetTransactions {
    #[serde(rename = "ASSET_TRANSACTION")]
    pub asset_transaction: Vec<crate::models::CreditFreddieMacAssetTransaction>,
}

impl CreditFreddieMacAssetTransactions {
    /// Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    pub fn new(asset_transaction: Vec<crate::models::CreditFreddieMacAssetTransaction>) -> CreditFreddieMacAssetTransactions {
        CreditFreddieMacAssetTransactions {
            asset_transaction,
        }
    }
}


