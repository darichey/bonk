/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentInitiationStandingOrderMetadata : Metadata specifically related to valid Payment Initiation standing order configurations for the institution.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentInitiationStandingOrderMetadata {
    /// Indicates whether the institution supports closed-ended standing orders by providing an end date.
    #[serde(rename = "supports_standing_order_end_date")]
    pub supports_standing_order_end_date: bool,
    /// This is only applicable to `MONTHLY` standing orders. Indicates whether the institution supports negative integers (-1 to -5) for setting up a `MONTHLY` standing order relative to the end of the month.
    #[serde(rename = "supports_standing_order_negative_execution_days")]
    pub supports_standing_order_negative_execution_days: bool,
    /// A list of the valid standing order intervals supported by the institution.
    #[serde(rename = "valid_standing_order_intervals")]
    pub valid_standing_order_intervals: Vec<crate::models::PaymentScheduleInterval>,
}

impl PaymentInitiationStandingOrderMetadata {
    /// Metadata specifically related to valid Payment Initiation standing order configurations for the institution.
    pub fn new(supports_standing_order_end_date: bool, supports_standing_order_negative_execution_days: bool, valid_standing_order_intervals: Vec<crate::models::PaymentScheduleInterval>) -> PaymentInitiationStandingOrderMetadata {
        PaymentInitiationStandingOrderMetadata {
            supports_standing_order_end_date,
            supports_standing_order_negative_execution_days,
            valid_standing_order_intervals,
        }
    }
}


