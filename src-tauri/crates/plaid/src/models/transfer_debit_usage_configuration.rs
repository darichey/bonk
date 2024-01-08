/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferDebitUsageConfiguration : Specifies the originator's expected usage of debits. For all dollar amounts, use a decimal string with two digits of precision e.g. \"10.00\". This field is required if the originator is expected to process debit transfers.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferDebitUsageConfiguration {
    #[serde(rename = "expected_frequency")]
    pub expected_frequency: crate::models::OriginatorExpectedTransferFrequency,
    /// The originator’s expected highest amount for a single debit transfer.
    #[serde(rename = "expected_highest_amount")]
    pub expected_highest_amount: String,
    /// The originator’s expected average amount per debit.
    #[serde(rename = "expected_average_amount")]
    pub expected_average_amount: String,
    /// The originator’s monthly expected ACH debit processing amount for the next 6-12 months.
    #[serde(rename = "expected_monthly_amount")]
    pub expected_monthly_amount: String,
    /// Specifies the expected use cases for the originator’s debit transfers. This should be a list that contains one or more of the following codes:  `\"ccd\"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts  `\"ppd\"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment  `\"tel\"` - Telephone-Initiated Entry  `\"web\"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet
    #[serde(rename = "sec_codes")]
    pub sec_codes: Vec<crate::models::AchClass>,
}

impl TransferDebitUsageConfiguration {
    /// Specifies the originator's expected usage of debits. For all dollar amounts, use a decimal string with two digits of precision e.g. \"10.00\". This field is required if the originator is expected to process debit transfers.
    pub fn new(expected_frequency: crate::models::OriginatorExpectedTransferFrequency, expected_highest_amount: String, expected_average_amount: String, expected_monthly_amount: String, sec_codes: Vec<crate::models::AchClass>) -> TransferDebitUsageConfiguration {
        TransferDebitUsageConfiguration {
            expected_frequency,
            expected_highest_amount,
            expected_average_amount,
            expected_monthly_amount,
            sec_codes,
        }
    }
}


