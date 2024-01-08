/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkTokenCreateCardSwitch : A map containing data to pass in for the Card Switch flow.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkTokenCreateCardSwitch {
    /// The BIN (Bank Identification Number) of the card to switch.
    #[serde(rename = "card_bin")]
    pub card_bin: String,
}

impl LinkTokenCreateCardSwitch {
    /// A map containing data to pass in for the Card Switch flow.
    pub fn new(card_bin: String) -> LinkTokenCreateCardSwitch {
        LinkTokenCreateCardSwitch {
            card_bin,
        }
    }
}


