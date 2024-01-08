/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkDeliveryDeliveryMethod : The delivery method to be used to deliver the Hosted Link session URL.  `SMS`: The URL will be delivered through SMS  `EMAIL`: The URL will be delivered through email

/// The delivery method to be used to deliver the Hosted Link session URL.  `SMS`: The URL will be delivered through SMS  `EMAIL`: The URL will be delivered through email
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LinkDeliveryDeliveryMethod {
    #[serde(rename = "SMS")]
    Sms,
    #[serde(rename = "EMAIL")]
    Email,

}

impl ToString for LinkDeliveryDeliveryMethod {
    fn to_string(&self) -> String {
        match self {
            Self::Sms => String::from("SMS"),
            Self::Email => String::from("EMAIL"),
        }
    }
}

impl Default for LinkDeliveryDeliveryMethod {
    fn default() -> LinkDeliveryDeliveryMethod {
        Self::Sms
    }
}




