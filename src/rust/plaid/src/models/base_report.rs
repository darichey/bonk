/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BaseReport : An object representing a Base Report



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseReport {
    /// A unique ID identifying an Base Report. Like all Plaid identifiers, this ID is case sensitive.
    #[serde(rename = "report_id")]
    pub report_id: String,
    /// The date and time when the Base Report was created, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (e.g. \"2018-04-12T03:32:11Z\").
    #[serde(rename = "date_generated")]
    pub date_generated: String,
    /// The number of days of transaction history requested.
    #[serde(rename = "days_requested")]
    pub days_requested: f32,
    /// Data returned by Plaid about each of the Items included in the Base Report.
    #[serde(rename = "items")]
    pub items: Vec<crate::models::BaseReportItem>,
}

impl BaseReport {
    /// An object representing a Base Report
    pub fn new(report_id: String, date_generated: String, days_requested: f32, items: Vec<crate::models::BaseReportItem>) -> BaseReport {
        BaseReport {
            report_id,
            date_generated,
            days_requested,
            items,
        }
    }
}


