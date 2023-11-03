/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferCreditFundsSource : Specifies the source of funds for the transfer. Only valid for `credit` transfers, and defaults to `sweep` if not specified. This field is not specified for `debit` transfers.  `sweep` - Sweep funds from your funding account `prefunded_rtp_credits` - Use your prefunded RTP credit balance with Plaid `prefunded_ach_credits` - Use your prefunded ACH credit balance with Plaid

/// Specifies the source of funds for the transfer. Only valid for `credit` transfers, and defaults to `sweep` if not specified. This field is not specified for `debit` transfers.  `sweep` - Sweep funds from your funding account `prefunded_rtp_credits` - Use your prefunded RTP credit balance with Plaid `prefunded_ach_credits` - Use your prefunded ACH credit balance with Plaid
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransferCreditFundsSource {
    #[serde(rename = "sweep")]
    Sweep,
    #[serde(rename = "prefunded_rtp_credits")]
    PrefundedRtpCredits,
    #[serde(rename = "prefunded_ach_credits")]
    PrefundedAchCredits,
    #[serde(rename = "null")]
    Null,

}

impl ToString for TransferCreditFundsSource {
    fn to_string(&self) -> String {
        match self {
            Self::Sweep => String::from("sweep"),
            Self::PrefundedRtpCredits => String::from("prefunded_rtp_credits"),
            Self::PrefundedAchCredits => String::from("prefunded_ach_credits"),
            Self::Null => String::from("null"),
        }
    }
}

impl Default for TransferCreditFundsSource {
    fn default() -> TransferCreditFundsSource {
        Self::Sweep
    }
}




