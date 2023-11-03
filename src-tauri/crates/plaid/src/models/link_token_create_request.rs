/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkTokenCreateRequest : LinkTokenCreateRequest defines the request schema for `/link/token/create`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LinkTokenCreateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The name of your application, as it should be displayed in Link. Maximum length of 30 characters. If a value longer than 30 characters is provided, Link will display \"This Application\" instead.
    #[serde(rename = "client_name")]
    pub client_name: String,
    /// The language that Link should be displayed in. When initializing with Identity Verification, this field is not used; for more details, see [Identity Verification supported languages](https://www.plaid.com/docs/identity-verification/#supported-languages).  Supported languages are: - Danish (`'da'`) - Dutch (`'nl'`) - English (`'en'`) - Estonian (`'et'`) - French (`'fr'`) - German (`'de'`) - Italian (`'it'`) - Latvian (`'lv'`) - Lithuanian (`'lt'`) - Norwegian (`'no'`) - Polish (`'pl'`) - Portuguese (`'pt'`) - Romanian (`'ro'`) - Spanish (`'es'`) - Swedish (`'sv'`)  When using a Link customization, the language configured here must match the setting in the customization, or the customization will not be applied.
    #[serde(rename = "language")]
    pub language: String,
    /// Specify an array of Plaid-supported country codes using the ISO-3166-1 alpha-2 country code standard. Institutions from all listed countries will be shown. For a complete mapping of supported products by country, see https://plaid.com/global/.  If Link is launched with multiple country codes, only products that you are enabled for in all countries will be used by Link. Note that while all countries are enabled by default in Sandbox and Development, in Production only US and Canada are enabled by default. Access to European institutions requires additional compliance steps. To request access to European institutions in the Production environment, [file a product access Support ticket](https://dashboard.plaid.com/support/new/product-and-development/product-troubleshooting/request-product-access) via the Plaid dashboard. If you initialize with a European country code, your users will see the European consent panel during the Link flow.  If using a Link customization, make sure the country codes in the customization match those specified in `country_codes`, or the customization may not be applied.  If using the Auth features Instant Match, Same-day Micro-deposits, or Automated Micro-deposits, `country_codes` must be set to `['US']`.
    #[serde(rename = "country_codes")]
    pub country_codes: Vec<crate::models::CountryCode>,
    #[serde(rename = "user")]
    pub user: Box<crate::models::LinkTokenCreateRequestUser>,
    /// List of Plaid product(s) you wish to use. If launching Link in update mode, should be omitted (unless you are using update mode to add Income or Assets to an Item); required otherwise.  `balance` is *not* a valid value, the Balance product does not require explicit initialization and will automatically be initialized when any other product is initialized.  The products specified here will determine which institutions will be available to your users in Link. Only institutions that support *all* requested products can be selected; a if a user attempts to select an institution that does not support a listed product, a \"Connectivity not supported\" error message will appear in Link. To maximize the number of institutions available, initialize Link with the minimal product set required for your use case. Additional products can be included via the [`optional_products`](https://plaid.com/docs/api/tokens/#link-token-create-request-optional-products) or  [`required_if_supported_products`](https://plaid.com/docs/api/tokens/#link-token-create-request-required-if-supported-products) fields, or can be initialized by calling the endpoint after obtaining an access token. For details and exceptions, see [Choosing when to initialize products](https://plaid.com/docs/link/initializing-products/).  Note that, unless you have opted to disable Instant Match support, institutions that support Instant Match will also be shown in Link if `auth` is specified as a product, even though these institutions do not contain `auth` in their product array.  In Production, you will be billed for each product that you specify when initializing Link. Note that a product cannot be removed from an Item once the Item has been initialized with that product. To stop billing on an Item for subscription-based products, such as Liabilities, Investments, and Transactions, remove the Item via `/item/remove`.
    #[serde(rename = "products", skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<crate::models::Products>>,
    /// List of Plaid product(s) you wish to use only if the institution and account(s) selected by the user support the product. Institutions that do not support these products will still be shown in Link. The products will only be extracted and billed if the user selects an institution and account type that supports them.  There should be no overlap between this array and the `products`, `optional_products`, or `additional_consented_products` arrays. The `products` array must have at least one product.  For more details on using this feature, see [Required if Supported Products](https://www.plaid.com/docs/link/initializing-products/#required-if-supported-products).
    #[serde(rename = "required_if_supported_products", skip_serializing_if = "Option::is_none")]
    pub required_if_supported_products: Option<Vec<crate::models::Products>>,
    /// List of Plaid product(s) that you may wish to use but that are not required for your use case. Plaid will attempt to fetch data for these products on a best-effort basis, and failure to support these products will not affect Item creation.  There should be no overlap between this array and the `products`, `required_if_supported_products`, or `additional_consented_products` arrays. The `products` array must have at least one product.  For more details on using this feature, see [Optional Products](https://www.plaid.com/docs/link/initializing-products/#optional-products).
    #[serde(rename = "optional_products", skip_serializing_if = "Option::is_none")]
    pub optional_products: Option<Vec<crate::models::Products>>,
    /// (Beta) This field has no effect unless you are participating in the [Data Transparency](https://plaid.com/docs/link/data-transparency-messaging-migration-guide) beta program. List of additional Plaid product(s) you wish to collect consent for. These products will not be billed until you start using them by calling the relevant endpoints.  `balance` is *not* a valid value, the Balance product does not require explicit initialization and will automatically have consent collected.  Institutions that do not support these products will still be shown in Link.  There should be no overlap between this array and the `products` or `required_if_supported_products` arrays.
    #[serde(rename = "additional_consented_products", skip_serializing_if = "Option::is_none")]
    pub additional_consented_products: Option<Vec<crate::models::Products>>,
    /// The destination URL to which any webhooks should be sent. Note that webhooks for Payment Initiation (e-wallet transactions only), Transfer, Bank Transfer (including Auth micro-deposit notification webhooks) and Identity Verification are configured via the Dashboard instead.
    #[serde(rename = "webhook", skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
    /// The `access_token` associated with the Item to update or reference, used when updating, modifying, or accessing an existing `access_token`. Used when launching Link in update mode, when completing the Same-day (manual) Micro-deposit flow, or (optionally) when initializing Link for a returning user as part of the Transfer UI flow.
    #[serde(rename = "access_token", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// The name of the Link customization from the Plaid Dashboard to be applied to Link. If not specified, the `default` customization will be used. When using a Link customization, the language in the customization must match the language selected via the `language` parameter, and the countries in the customization should match the country codes selected via `country_codes`.
    #[serde(rename = "link_customization_name", skip_serializing_if = "Option::is_none")]
    pub link_customization_name: Option<String>,
    /// A URI indicating the destination where a user should be forwarded after completing the Link flow; used to support OAuth authentication flows when launching Link in the browser or via a webview. The `redirect_uri` should not contain any query parameters. When used in Production or Development, must be an https URI. To specify any subdomain, use `*` as a wildcard character, e.g. `https://_*.example.com/oauth.html`. Note that any redirect URI must also be added to the Allowed redirect URIs list in the [developer dashboard](https://dashboard.plaid.com/team/api). If initializing on Android, `android_package_name` must be specified instead and `redirect_uri` should be left blank.
    #[serde(rename = "redirect_uri", skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    /// The name of your app's Android package. Required if using the `link_token` to initialize Link on Android. Any package name specified here must also be added to the Allowed Android package names setting on the [developer dashboard](https://dashboard.plaid.com/team/api). When creating a `link_token` for initializing Link on other platforms, `android_package_name` must be left blank and `redirect_uri` should be used instead.
    #[serde(rename = "android_package_name", skip_serializing_if = "Option::is_none")]
    pub android_package_name: Option<String>,
    #[serde(rename = "institution_data", skip_serializing_if = "Option::is_none")]
    pub institution_data: Option<Box<crate::models::LinkTokenCreateInstitutionData>>,
    #[serde(rename = "account_filters", skip_serializing_if = "Option::is_none")]
    pub account_filters: Option<crate::models::LinkTokenAccountFilters>,
    #[serde(rename = "eu_config", skip_serializing_if = "Option::is_none")]
    pub eu_config: Option<Box<crate::models::LinkTokenEuConfig>>,
    /// Used for certain Europe-only configurations, as well as certain legacy use cases in other regions.
    #[serde(rename = "institution_id", skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
    #[serde(rename = "payment_initiation", skip_serializing_if = "Option::is_none")]
    pub payment_initiation: Option<Box<crate::models::LinkTokenCreateRequestPaymentInitiation>>,
    #[serde(rename = "deposit_switch", skip_serializing_if = "Option::is_none")]
    pub deposit_switch: Option<Box<crate::models::LinkTokenCreateRequestDepositSwitch>>,
    #[serde(rename = "employment", skip_serializing_if = "Option::is_none")]
    pub employment: Option<Box<crate::models::LinkTokenCreateRequestEmployment>>,
    #[serde(rename = "income_verification", skip_serializing_if = "Option::is_none")]
    pub income_verification: Option<Box<crate::models::LinkTokenCreateRequestIncomeVerification>>,
    #[serde(rename = "base_report", skip_serializing_if = "Option::is_none")]
    pub base_report: Option<Box<crate::models::LinkTokenCreateRequestBaseReport>>,
    #[serde(rename = "consumer_report_permissible_purpose", skip_serializing_if = "Option::is_none")]
    pub consumer_report_permissible_purpose: Option<crate::models::ConsumerReportPermissiblePurpose>,
    #[serde(rename = "auth", skip_serializing_if = "Option::is_none")]
    pub auth: Option<Box<crate::models::LinkTokenCreateRequestAuth>>,
    #[serde(rename = "transfer", skip_serializing_if = "Option::is_none")]
    pub transfer: Option<Box<crate::models::LinkTokenCreateRequestTransfer>>,
    #[serde(rename = "update", skip_serializing_if = "Option::is_none")]
    pub update: Option<Box<crate::models::LinkTokenCreateRequestUpdate>>,
    #[serde(rename = "identity_verification", skip_serializing_if = "Option::is_none")]
    pub identity_verification: Option<Box<crate::models::LinkTokenCreateRequestIdentityVerification>>,
    #[serde(rename = "statements", skip_serializing_if = "Option::is_none")]
    pub statements: Option<Box<crate::models::LinkTokenCreateRequestStatements>>,
    /// A user token generated using `/user/create`. Any Item created during the Link session will be associated with the user.
    #[serde(rename = "user_token", skip_serializing_if = "Option::is_none")]
    pub user_token: Option<String>,
    #[serde(rename = "investments", skip_serializing_if = "Option::is_none")]
    pub investments: Option<Box<crate::models::LinkTokenInvestments>>,
    #[serde(rename = "investments_auth", skip_serializing_if = "Option::is_none")]
    pub investments_auth: Option<Box<crate::models::LinkTokenInvestmentsAuth>>,
    #[serde(rename = "hosted_link", skip_serializing_if = "Option::is_none")]
    pub hosted_link: Option<crate::models::LinkTokenCreateHostedLink>,
}

impl LinkTokenCreateRequest {
    /// LinkTokenCreateRequest defines the request schema for `/link/token/create`
    pub fn new(client_name: String, language: String, country_codes: Vec<crate::models::CountryCode>, user: crate::models::LinkTokenCreateRequestUser) -> LinkTokenCreateRequest {
        LinkTokenCreateRequest {
            client_id: None,
            secret: None,
            client_name,
            language,
            country_codes,
            user: Box::new(user),
            products: None,
            required_if_supported_products: None,
            optional_products: None,
            additional_consented_products: None,
            webhook: None,
            access_token: None,
            link_customization_name: None,
            redirect_uri: None,
            android_package_name: None,
            institution_data: None,
            account_filters: None,
            eu_config: None,
            institution_id: None,
            payment_initiation: None,
            deposit_switch: None,
            employment: None,
            income_verification: None,
            base_report: None,
            consumer_report_permissible_purpose: None,
            auth: None,
            transfer: None,
            update: None,
            identity_verification: None,
            statements: None,
            user_token: None,
            investments: None,
            investments_auth: None,
            hosted_link: None,
        }
    }
}


