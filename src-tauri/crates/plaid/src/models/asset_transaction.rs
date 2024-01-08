/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AssetTransaction : An object representing...



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetTransaction {
    #[serde(rename = "ASSET_TRANSACTION_DETAIL")]
    pub asset_transaction_detail: crate::models::AssetTransactionDetail,
    /// Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "ASSET_TRANSACTION_DESCRIPTON")]
    pub asset_transaction_descripton: Vec<crate::models::AssetTransactionDescription>,
}

impl AssetTransaction {
    /// An object representing...
    pub fn new(asset_transaction_detail: crate::models::AssetTransactionDetail, asset_transaction_descripton: Vec<crate::models::AssetTransactionDescription>) -> AssetTransaction {
        AssetTransaction {
            asset_transaction_detail,
            asset_transaction_descripton,
        }
    }
}


