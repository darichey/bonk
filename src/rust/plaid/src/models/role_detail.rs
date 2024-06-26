/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RoleDetail : Documentation not found in the MISMO model viewer and not provided by Freddie Mac.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleDetail {
    #[serde(rename = "PartyRoleType")]
    pub party_role_type: crate::models::PartyRoleType,
}

impl RoleDetail {
    /// Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    pub fn new(party_role_type: crate::models::PartyRoleType) -> RoleDetail {
        RoleDetail {
            party_role_type,
        }
    }
}


