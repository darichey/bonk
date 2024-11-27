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

/// PartnerCustomerCreateRequest : Request schema for `/partner/customer/create`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartnerCustomerCreateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The company name of the end customer being created. This will be used to display the end customer in the Plaid Dashboard. It will not be shown to end users.
    #[serde(rename = "company_name")]
    pub company_name: String,
    /// Denotes whether or not the partner has completed attestation of diligence for the end customer to be created.
    #[serde(rename = "is_diligence_attested")]
    pub is_diligence_attested: bool,
    /// The products to be enabled for the end customer. If empty or `null`, this field will default to the products enabled for the reseller at the time this endpoint is called.
    #[serde(rename = "products", skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<models::Products>>,
    /// If `true`, the end customer's default Link customization will be set to match the partner's. You can always change the end customer's Link customization in the Plaid Dashboard. See the [Link Customization docs](https://plaid.com/docs/link/customization/) for more information.
    #[serde(rename = "create_link_customization", skip_serializing_if = "Option::is_none")]
    pub create_link_customization: Option<bool>,
    /// Base64-encoded representation of the end customer's logo. Must be a PNG of size 1024x1024 under 4MB. The logo will be shared with financial institutions and shown to the end user during Link flows. A logo is required if `create_link_customization` is `true`. If `create_link_customization` is `false` and the logo is omitted, the partner's logo will be used if one exists, otherwise a stock logo will be used.
    #[serde(rename = "logo", skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    /// The end customer's legal name. This will be shared with financial institutions as part of the OAuth registration process. It will not be shown to end users.
    #[serde(rename = "legal_entity_name")]
    pub legal_entity_name: String,
    /// The end customer's website.
    #[serde(rename = "website")]
    pub website: String,
    /// The name of the end customer's application. This will be shown to end users when they go through the Plaid Link flow.
    #[serde(rename = "application_name")]
    pub application_name: String,
    #[serde(rename = "technical_contact", skip_serializing_if = "Option::is_none")]
    pub technical_contact: Option<models::PartnerEndCustomerTechnicalContact>,
    #[serde(rename = "billing_contact", skip_serializing_if = "Option::is_none")]
    pub billing_contact: Option<models::PartnerEndCustomerBillingContact>,
    #[serde(rename = "customer_support_info", skip_serializing_if = "Option::is_none")]
    pub customer_support_info: Option<models::PartnerEndCustomerCustomerSupportInfo>,
    #[serde(rename = "address")]
    pub address: models::PartnerEndCustomerAddress,
    /// Denotes whether the partner has forwarded the Plaid bank addendum to the end customer.
    #[serde(rename = "is_bank_addendum_completed")]
    pub is_bank_addendum_completed: bool,
    #[serde(rename = "assets_under_management", skip_serializing_if = "Option::is_none")]
    pub assets_under_management: Option<models::PartnerEndCustomerAssetsUnderManagement>,
    /// A list of URIs indicating the destination(s) where a user can be forwarded after completing the Link flow; used to support OAuth authentication flows when launching Link in the browser or another app. URIs should not contain any query parameters. When used in Production, URIs must use https. To specify any subdomain, use `*` as a wildcard character, e.g. `https://_*.example.com/oauth.html`. To modify redirect URIs for an end customer after creating them, go to the end customer's [API page](https://dashboard.plaid.com/team/api) in the Dashboard.
    #[serde(rename = "redirect_uris", skip_serializing_if = "Option::is_none")]
    pub redirect_uris: Option<Vec<String>>,
    /// The unique identifier assigned to a financial institution by regulatory authorities, if applicable. For banks, this is the FDIC Certificate Number. For credit unions, this is the Credit Union Charter Number.
    #[serde(rename = "registration_number", skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
}

impl PartnerCustomerCreateRequest {
    /// Request schema for `/partner/customer/create`.
    pub fn new(company_name: String, is_diligence_attested: bool, legal_entity_name: String, website: String, application_name: String, address: models::PartnerEndCustomerAddress, is_bank_addendum_completed: bool) -> PartnerCustomerCreateRequest {
        PartnerCustomerCreateRequest {
            client_id: None,
            secret: None,
            company_name,
            is_diligence_attested,
            products: None,
            create_link_customization: None,
            logo: None,
            legal_entity_name,
            website,
            application_name,
            technical_contact: None,
            billing_contact: None,
            customer_support_info: None,
            address,
            is_bank_addendum_completed,
            assets_under_management: None,
            redirect_uris: None,
            registration_number: None,
        }
    }
}

