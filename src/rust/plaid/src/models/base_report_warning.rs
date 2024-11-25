/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BaseReportWarning : It is possible for a Base Report to be returned with missing account owner information. In such cases, the Base Report will contain warning data in the response, indicating why obtaining the owner information failed.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BaseReportWarning {
    /// The warning type, which will always be `BASE_REPORT_WARNING`
    #[serde(rename = "warning_type")]
    pub warning_type: String,
    #[serde(rename = "warning_code")]
    pub warning_code: crate::models::BaseReportWarningCode,
    #[serde(rename = "cause", deserialize_with = "Option::deserialize")]
    pub cause: Option<Box<crate::models::Cause>>,
}

impl BaseReportWarning {
    /// It is possible for a Base Report to be returned with missing account owner information. In such cases, the Base Report will contain warning data in the response, indicating why obtaining the owner information failed.
    pub fn new(warning_type: String, warning_code: crate::models::BaseReportWarningCode, cause: Option<crate::models::Cause>) -> BaseReportWarning {
        BaseReportWarning {
            warning_type,
            warning_code,
            cause: if let Some(x) = cause {Some(Box::new(x))} else {None},
        }
    }
}


