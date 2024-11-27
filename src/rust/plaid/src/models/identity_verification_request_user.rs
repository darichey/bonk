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

/// IdentityVerificationRequestUser : User information collected outside of Link, most likely via your own onboarding process.  Each of the following identity fields are optional:  `email_address`  `phone_number`  `date_of_birth`  `name`  `address`  `id_number`  Specifically, these fields are optional in that they can either be fully provided (satisfying every required field in their subschema) or omitted from the request entirely by not providing the key or value. Providing these fields via the API will result in Link skipping the data collection process for the associated user. All verification steps enabled in the associated Identity Verification Template will still be run. Verification steps will either be run immediately, or once the user completes the `accept_tos` step, depending on the value provided to the `gave_consent` field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityVerificationRequestUser {
    /// A valid email address. Must not have leading or trailing spaces and address must be RFC compliant. For more information, see [RFC 3696](https://datatracker.ietf.org/doc/html/rfc3696).
    #[serde(rename = "email_address", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// A valid phone number in E.164 format.
    #[serde(rename = "phone_number", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<Option<String>>,
    /// A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    #[serde(rename = "date_of_birth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<Box<models::IdentityVerificationRequestUserName>>>,
    #[serde(rename = "address", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub address: Option<Option<models::UserAddress>>,
    #[serde(rename = "id_number", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id_number: Option<Option<models::UserIdNumber>>,
}

impl IdentityVerificationRequestUser {
    /// User information collected outside of Link, most likely via your own onboarding process.  Each of the following identity fields are optional:  `email_address`  `phone_number`  `date_of_birth`  `name`  `address`  `id_number`  Specifically, these fields are optional in that they can either be fully provided (satisfying every required field in their subschema) or omitted from the request entirely by not providing the key or value. Providing these fields via the API will result in Link skipping the data collection process for the associated user. All verification steps enabled in the associated Identity Verification Template will still be run. Verification steps will either be run immediately, or once the user completes the `accept_tos` step, depending on the value provided to the `gave_consent` field.
    pub fn new() -> IdentityVerificationRequestUser {
        IdentityVerificationRequestUser {
            email_address: None,
            phone_number: None,
            date_of_birth: None,
            name: None,
            address: None,
            id_number: None,
        }
    }
}

