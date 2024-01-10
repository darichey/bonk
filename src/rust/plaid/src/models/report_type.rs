/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ReportType : The report type. It can be `asset`. Income report types are not yet supported.

/// The report type. It can be `asset`. Income report types are not yet supported.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ReportType {
    #[serde(rename = "asset")]
    Asset,

}

impl ToString for ReportType {
    fn to_string(&self) -> String {
        match self {
            Self::Asset => String::from("asset"),
        }
    }
}

impl Default for ReportType {
    fn default() -> ReportType {
        Self::Asset
    }
}



