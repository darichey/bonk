/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ProfileNetworkStatusGetUser : An object specifying information about the end user for the network status check



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProfileNetworkStatusGetUser {
    /// The user's phone number in [E.164](https://en.wikipedia.org/wiki/E.164) format
    #[serde(rename = "phone_number")]
    pub phone_number: String,
}

impl ProfileNetworkStatusGetUser {
    /// An object specifying information about the end user for the network status check
    pub fn new(phone_number: String) -> ProfileNetworkStatusGetUser {
        ProfileNetworkStatusGetUser {
            phone_number,
        }
    }
}


