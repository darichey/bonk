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

/// AssetReportInvestmentSecurity : Investment security associated with the account.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetReportInvestmentSecurity {
    /// A unique, Plaid-specific identifier for the security, used to associate securities with holdings. Like all Plaid identifiers, the `security_id` is case sensitive. The `security_id` may change if inherent details of the security change due to a corporate action, for example, in the event of a ticker symbol change or CUSIP change.
    #[serde(rename = "security_id")]
    pub security_id: String,
    /// A descriptive name for the security, suitable for display.
    #[serde(rename = "name", deserialize_with = "Option::deserialize")]
    pub name: Option<String>,
    /// The security’s trading symbol for publicly traded securities, and otherwise a short identifier if available.
    #[serde(rename = "ticker_symbol", deserialize_with = "Option::deserialize")]
    pub ticker_symbol: Option<String>,
    /// The security type of the holding. Valid security types are:  `cash`: Cash, currency, and money market funds  `cryptocurrency`: Digital or virtual currencies  `derivative`: Options, warrants, and other derivative instruments  `equity`: Domestic and foreign equities  `etf`: Multi-asset exchange-traded investment funds  `fixed income`: Bonds and certificates of deposit (CDs)  `loan`: Loans and loan receivables  `mutual fund`: Open- and closed-end vehicles pooling funds of multiple investors  `other`: Unknown or other investment types
    #[serde(rename = "type", deserialize_with = "Option::deserialize")]
    pub r#type: Option<String>,
}

impl AssetReportInvestmentSecurity {
    /// Investment security associated with the account.
    pub fn new(security_id: String, name: Option<String>, ticker_symbol: Option<String>, r#type: Option<String>) -> AssetReportInvestmentSecurity {
        AssetReportInvestmentSecurity {
            security_id,
            name,
            ticker_symbol,
            r#type,
        }
    }
}

