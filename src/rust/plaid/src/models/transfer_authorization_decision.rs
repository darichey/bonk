/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// TransferAuthorizationDecision :  A decision regarding the proposed transfer.  `approved` – The proposed transfer has received the end user's consent and has been approved for processing by Plaid. The `decision_rationale` field is set if Plaid was unable to fetch the account information. You may proceed with the transfer, but further review is recommended. Refer to the `code` field in the `decision_rationale` object for details.  `declined` – Plaid reviewed the proposed transfer and declined processing. Refer to the `code` field in the `decision_rationale` object for details.  `user_action_required` – An action is required before Plaid can assess the transfer risk and make a decision. The most common scenario is to update authentication for an Item. To complete the required action, initialize Link by setting `transfer.authorization_id` in the request of `/link/token/create`. After Link flow is completed, you may re-attempt the authorization request.
///  A decision regarding the proposed transfer.  `approved` – The proposed transfer has received the end user's consent and has been approved for processing by Plaid. The `decision_rationale` field is set if Plaid was unable to fetch the account information. You may proceed with the transfer, but further review is recommended. Refer to the `code` field in the `decision_rationale` object for details.  `declined` – Plaid reviewed the proposed transfer and declined processing. Refer to the `code` field in the `decision_rationale` object for details.  `user_action_required` – An action is required before Plaid can assess the transfer risk and make a decision. The most common scenario is to update authentication for an Item. To complete the required action, initialize Link by setting `transfer.authorization_id` in the request of `/link/token/create`. After Link flow is completed, you may re-attempt the authorization request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransferAuthorizationDecision {
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "declined")]
    Declined,
    #[serde(rename = "user_action_required")]
    UserActionRequired,

}

impl std::fmt::Display for TransferAuthorizationDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Approved => write!(f, "approved"),
            Self::Declined => write!(f, "declined"),
            Self::UserActionRequired => write!(f, "user_action_required"),
        }
    }
}

impl Default for TransferAuthorizationDecision {
    fn default() -> TransferAuthorizationDecision {
        Self::Approved
    }
}

