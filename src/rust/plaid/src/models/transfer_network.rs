/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferNetwork : The network or rails used for the transfer.  For transfers submitted as `ach`, the next-day cutoff is 5:30 PM Eastern Time.  For transfers submitted as `same-day-ach`, the same-day cutoff is 3:30 PM Eastern Time. If the transfer is submitted after this cutoff but before the next-day cutoff, it will be sent over next-day rails and will not incur same-day charges; this will apply to both legs of the transfer if applicable.  For transfers submitted as `rtp`,  Plaid will automatically route between Real Time Payment rail by TCH or FedNow rails as necessary. If a transfer is submitted as `rtp` and the counterparty account is not eligible for RTP, the `/transfer/authorization/create` request will fail with an `INVALID_FIELD` error code. To pre-check to determine whether a counterparty account can support RTP, call `/transfer/capabilities/get` before calling `/transfer/authorization/create`.

/// The network or rails used for the transfer.  For transfers submitted as `ach`, the next-day cutoff is 5:30 PM Eastern Time.  For transfers submitted as `same-day-ach`, the same-day cutoff is 3:30 PM Eastern Time. If the transfer is submitted after this cutoff but before the next-day cutoff, it will be sent over next-day rails and will not incur same-day charges; this will apply to both legs of the transfer if applicable.  For transfers submitted as `rtp`,  Plaid will automatically route between Real Time Payment rail by TCH or FedNow rails as necessary. If a transfer is submitted as `rtp` and the counterparty account is not eligible for RTP, the `/transfer/authorization/create` request will fail with an `INVALID_FIELD` error code. To pre-check to determine whether a counterparty account can support RTP, call `/transfer/capabilities/get` before calling `/transfer/authorization/create`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransferNetwork {
    #[serde(rename = "ach")]
    Ach,
    #[serde(rename = "same-day-ach")]
    SameDayAch,
    #[serde(rename = "rtp")]
    Rtp,

}

impl ToString for TransferNetwork {
    fn to_string(&self) -> String {
        match self {
            Self::Ach => String::from("ach"),
            Self::SameDayAch => String::from("same-day-ach"),
            Self::Rtp => String::from("rtp"),
        }
    }
}

impl Default for TransferNetwork {
    fn default() -> TransferNetwork {
        Self::Ach
    }
}




