/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ExternalPaymentOptions : Additional payment options



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ExternalPaymentOptions {
    /// When `true`, Plaid will attempt to request refund details from the payee's financial institution.  Support varies between financial institutions and will not always be available.  If refund details could be retrieved, they will be available in the `/payment_initiation/payment/get` response.
    #[serde(rename = "request_refund_details", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub request_refund_details: Option<Option<bool>>,
    /// The International Bank Account Number (IBAN) for the payer's account. Where possible, the end user will be able to send payments only from the specified bank account if provided.
    #[serde(rename = "iban", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub iban: Option<Option<String>>,
    #[serde(rename = "bacs", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub bacs: Option<Option<Box<crate::models::PaymentInitiationOptionalRestrictionBacs>>>,
    #[serde(rename = "scheme", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub scheme: Option<Option<crate::models::PaymentScheme>>,
}

impl ExternalPaymentOptions {
    /// Additional payment options
    pub fn new() -> ExternalPaymentOptions {
        ExternalPaymentOptions {
            request_refund_details: None,
            iban: None,
            bacs: None,
            scheme: None,
        }
    }
}


