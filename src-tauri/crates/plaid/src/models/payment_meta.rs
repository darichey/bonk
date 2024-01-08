/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentMeta : Transaction information specific to inter-bank transfers. If the transaction was not an inter-bank transfer, all fields will be `null`.  If the `transactions` object was returned by a Transactions endpoint such as `/transactions/sync` or `/transactions/get`, the `payment_meta` key will always appear, but no data elements are guaranteed. If the `transactions` object was returned by an Assets endpoint such as `/asset_report/get/` or `/asset_report/pdf/get`, this field will only appear in an Asset Report with Insights.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentMeta {
    /// The transaction reference number supplied by the financial institution.
    #[serde(rename = "reference_number", deserialize_with = "Option::deserialize")]
    pub reference_number: Option<String>,
    /// The ACH PPD ID for the payer.
    #[serde(rename = "ppd_id", deserialize_with = "Option::deserialize")]
    pub ppd_id: Option<String>,
    /// For transfers, the party that is receiving the transaction.
    #[serde(rename = "payee", deserialize_with = "Option::deserialize")]
    pub payee: Option<String>,
    /// The party initiating a wire transfer. Will be `null` if the transaction is not a wire transfer.
    #[serde(rename = "by_order_of", deserialize_with = "Option::deserialize")]
    pub by_order_of: Option<String>,
    /// For transfers, the party that is paying the transaction.
    #[serde(rename = "payer", deserialize_with = "Option::deserialize")]
    pub payer: Option<String>,
    /// The type of transfer, e.g. 'ACH'
    #[serde(rename = "payment_method", deserialize_with = "Option::deserialize")]
    pub payment_method: Option<String>,
    /// The name of the payment processor
    #[serde(rename = "payment_processor", deserialize_with = "Option::deserialize")]
    pub payment_processor: Option<String>,
    /// The payer-supplied description of the transfer.
    #[serde(rename = "reason", deserialize_with = "Option::deserialize")]
    pub reason: Option<String>,
}

impl PaymentMeta {
    /// Transaction information specific to inter-bank transfers. If the transaction was not an inter-bank transfer, all fields will be `null`.  If the `transactions` object was returned by a Transactions endpoint such as `/transactions/sync` or `/transactions/get`, the `payment_meta` key will always appear, but no data elements are guaranteed. If the `transactions` object was returned by an Assets endpoint such as `/asset_report/get/` or `/asset_report/pdf/get`, this field will only appear in an Asset Report with Insights.
    pub fn new(reference_number: Option<String>, ppd_id: Option<String>, payee: Option<String>, by_order_of: Option<String>, payer: Option<String>, payment_method: Option<String>, payment_processor: Option<String>, reason: Option<String>) -> PaymentMeta {
        PaymentMeta {
            reference_number,
            ppd_id,
            payee,
            by_order_of,
            payer,
            payment_method,
            payment_processor,
            reason,
        }
    }
}


