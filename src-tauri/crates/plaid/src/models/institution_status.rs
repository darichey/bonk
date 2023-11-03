/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InstitutionStatus : The status of an institution is determined by the health of its Item logins, Transactions updates, Investments updates, Liabilities updates, Auth requests, Balance requests, Identity requests, Investments requests, and Liabilities requests. A login attempt is conducted during the initial Item add in Link. If there is not enough traffic to accurately calculate an institution's status, Plaid will return null rather than potentially inaccurate data.  Institution status is accessible in the Dashboard and via the API using the `/institutions/get_by_id` endpoint with the `include_status` option set to true. Note that institution status is not available in the Sandbox environment. 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InstitutionStatus {
    #[serde(rename = "item_logins", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub item_logins: Option<Option<crate::models::ProductStatus>>,
    #[serde(rename = "transactions_updates", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub transactions_updates: Option<Option<crate::models::ProductStatus>>,
    #[serde(rename = "auth", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub auth: Option<Option<crate::models::ProductStatus>>,
    #[serde(rename = "identity", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub identity: Option<Option<crate::models::ProductStatus>>,
    #[serde(rename = "investments_updates", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub investments_updates: Option<Option<crate::models::ProductStatus>>,
    #[serde(rename = "liabilities_updates", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub liabilities_updates: Option<Option<crate::models::ProductStatus>>,
    #[serde(rename = "liabilities", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub liabilities: Option<Option<crate::models::ProductStatus>>,
    #[serde(rename = "investments", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub investments: Option<Option<crate::models::ProductStatus>>,
    /// Details of recent health incidents associated with the institution.
    #[serde(rename = "health_incidents", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub health_incidents: Option<Option<Vec<crate::models::HealthIncident>>>,
}

impl InstitutionStatus {
    /// The status of an institution is determined by the health of its Item logins, Transactions updates, Investments updates, Liabilities updates, Auth requests, Balance requests, Identity requests, Investments requests, and Liabilities requests. A login attempt is conducted during the initial Item add in Link. If there is not enough traffic to accurately calculate an institution's status, Plaid will return null rather than potentially inaccurate data.  Institution status is accessible in the Dashboard and via the API using the `/institutions/get_by_id` endpoint with the `include_status` option set to true. Note that institution status is not available in the Sandbox environment. 
    pub fn new() -> InstitutionStatus {
        InstitutionStatus {
            item_logins: None,
            transactions_updates: None,
            auth: None,
            identity: None,
            investments_updates: None,
            liabilities_updates: None,
            liabilities: None,
            investments: None,
            health_incidents: None,
        }
    }
}


