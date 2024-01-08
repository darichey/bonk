/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ServiceProductFulfillmentDetail : Documentation not found in the MISMO model viewer and not provided by Freddie Mac.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceProductFulfillmentDetail {
    /// A string that uniquely identifies a type of order Verification of Asset.
    #[serde(rename = "VendorOrderIdentifier", deserialize_with = "Option::deserialize")]
    pub vendor_order_identifier: Option<String>,
    #[serde(rename = "ServiceProductFulfillmentIdentifier")]
    pub service_product_fulfillment_identifier: crate::models::ServiceProductFulfillmentIdentifier,
}

impl ServiceProductFulfillmentDetail {
    /// Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    pub fn new(vendor_order_identifier: Option<String>, service_product_fulfillment_identifier: crate::models::ServiceProductFulfillmentIdentifier) -> ServiceProductFulfillmentDetail {
        ServiceProductFulfillmentDetail {
            vendor_order_identifier,
            service_product_fulfillment_identifier,
        }
    }
}


