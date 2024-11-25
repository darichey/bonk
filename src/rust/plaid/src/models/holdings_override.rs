/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// HoldingsOverride : Specify the holdings on the account.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HoldingsOverride {
    /// The last price given by the institution for this security
    #[serde(rename = "institution_price")]
    pub institution_price: f64,
    /// The date at which `institution_price` was current. Must be formatted as an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) date.
    #[serde(rename = "institution_price_as_of", skip_serializing_if = "Option::is_none")]
    pub institution_price_as_of: Option<String>,
    /// The total cost basis of the holding (e.g., the total amount spent to acquire all assets currently in the holding).
    #[serde(rename = "cost_basis", skip_serializing_if = "Option::is_none")]
    pub cost_basis: Option<f64>,
    /// The total quantity of the asset held, as reported by the financial institution.
    #[serde(rename = "quantity")]
    pub quantity: f64,
    /// Either a valid `iso_currency_code` or `unofficial_currency_code`
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "security")]
    pub security: Box<crate::models::SecurityOverride>,
}

impl HoldingsOverride {
    /// Specify the holdings on the account.
    pub fn new(institution_price: f64, quantity: f64, currency: String, security: crate::models::SecurityOverride) -> HoldingsOverride {
        HoldingsOverride {
            institution_price,
            institution_price_as_of: None,
            cost_basis: None,
            quantity,
            currency,
            security: Box::new(security),
        }
    }
}


