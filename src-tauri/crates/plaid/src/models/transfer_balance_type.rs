/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferBalanceType : The type of balance.  `prefunded_rtp_credits` - Your prefunded RTP credit balance with Plaid `prefunded_ach_credits` - Your prefunded ACH credit balance with Plaid

/// The type of balance.  `prefunded_rtp_credits` - Your prefunded RTP credit balance with Plaid `prefunded_ach_credits` - Your prefunded ACH credit balance with Plaid
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransferBalanceType {
    #[serde(rename = "prefunded_rtp_credits")]
    RtpCredits,
    #[serde(rename = "prefunded_ach_credits")]
    AchCredits,

}

impl ToString for TransferBalanceType {
    fn to_string(&self) -> String {
        match self {
            Self::RtpCredits => String::from("prefunded_rtp_credits"),
            Self::AchCredits => String::from("prefunded_ach_credits"),
        }
    }
}

impl Default for TransferBalanceType {
    fn default() -> TransferBalanceType {
        Self::RtpCredits
    }
}



