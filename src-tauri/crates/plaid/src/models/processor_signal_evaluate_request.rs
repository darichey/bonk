/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ProcessorSignalEvaluateRequest : ProcessorSignalEvaluateRequest defines the request schema for `/processor/signal/evaluate`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProcessorSignalEvaluateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The processor token obtained from the Plaid integration partner. Processor tokens are in the format: `processor-<environment>-<identifier>`
    #[serde(rename = "processor_token")]
    pub processor_token: String,
    /// The unique ID that you would like to use to refer to this transaction. For your convenience mapping your internal data, you could use your internal ID/identifier for this transaction. The max length for this field is 36 characters.
    #[serde(rename = "client_transaction_id")]
    pub client_transaction_id: String,
    /// The transaction amount, in USD (e.g. `102.05`)
    #[serde(rename = "amount")]
    pub amount: f64,
    /// `true` if the end user is present while initiating the ACH transfer and the endpoint is being called; `false` otherwise (for example, when the ACH transfer is scheduled and the end user is not present, or you call this endpoint after the ACH transfer but before submitting the Nacha file for ACH processing).
    #[serde(rename = "user_present", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user_present: Option<Option<bool>>,
    /// A unique ID that identifies the end user in your system. This ID is used to correlate requests by a user with multiple Items. Personally identifiable information, such as an email address or phone number, should not be used in the `client_user_id`.
    #[serde(rename = "client_user_id", skip_serializing_if = "Option::is_none")]
    pub client_user_id: Option<String>,
    /// **true** if the ACH transaction is a recurring transaction; **false** otherwise 
    #[serde(rename = "is_recurring", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_recurring: Option<Option<bool>>,
    /// The default ACH or non-ACH payment method to complete the transaction. `SAME_DAY_ACH`: Same Day ACH by NACHA. The debit transaction is processed and settled on the same day `NEXT_DAY_ACH`: Next Day ACH settlement for debit transactions, offered by some payment processors `STANDARD_ACH`: standard ACH by NACHA `REAL_TIME_PAYMENTS`: real-time payments such as RTP and FedNow `DEBIT_CARD`: if the default payment is over debit card networks `MULTIPLE_PAYMENT_METHODS`: if there is no default debit rail or there are multiple payment methods Possible values:  `SAME_DAY_ACH`, `NEXT_DAY_ACH`, `STANDARD_ACH`, `REAL_TIME_PAYMENTS`, `DEBIT_CARD`, `MULTIPLE_PAYMENT_METHODS`
    #[serde(rename = "default_payment_method", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<Option<String>>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::SignalUser>>,
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<Box<crate::models::SignalDevice>>,
}

impl ProcessorSignalEvaluateRequest {
    /// ProcessorSignalEvaluateRequest defines the request schema for `/processor/signal/evaluate`
    pub fn new(processor_token: String, client_transaction_id: String, amount: f64) -> ProcessorSignalEvaluateRequest {
        ProcessorSignalEvaluateRequest {
            client_id: None,
            secret: None,
            processor_token,
            client_transaction_id,
            amount,
            user_present: None,
            client_user_id: None,
            is_recurring: None,
            default_payment_method: None,
            user: None,
            device: None,
        }
    }
}


