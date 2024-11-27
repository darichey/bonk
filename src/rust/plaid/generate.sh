# we only use these APIs
apis=(
  "linkTokenCreate"
  "itemPublicTokenExchange"
  "transactionsGet"
)
apis_str=$(IFS=\|; echo "${apis[*]}")

# and these are the models necessary for those APIs
# this list was manually created by generating, compiling, and adding models until the compiler stopped complaining
# it kind of sucks, but the plaid api is HUGE and we only need a small subset of it, and this improves compile times a lot
models=(
  "CashflowAttributesVersion"
  "ConsumerReportPermissiblePurpose"
  "CountryCode"
  "CreditAccountSubtype"
  "CreditFilter"
  "DepositoryAccountSubtype"
  "DepositoryFilter"
  "EmploymentSourceType"
  "HostedLinkDeliveryMethod"
  "IDNumberType" # "ID" gets normalized to "Id", but it needs to be the original name here
  "IncomeVerificationDocParsingConfig"
  "IncomeVerificationPayrollFlowType"
  "IncomeVerificationSourceType"
  "InvestmentAccountSubtype"
  "InvestmentFilter"
  "LinkTokenAccountFilters"
  "LinkTokenCreateCardSwitch"
  "LinkTokenCreateHostedLink"
  "LinkTokenCreateIdentity"
  "LinkTokenCreateInstitutionData"
  "LinkTokenCreateRequest"
  "LinkTokenCreateRequestAuth"
  "LinkTokenCreateRequestBaseReport"
  "LinkTokenCreateRequestCraOptions"
  "LinkTokenCreateRequestCraOptionsBaseReport"
  "LinkTokenCreateRequestCraOptionsCashflowInsights"
  "LinkTokenCreateRequestCraOptionsPartnerInsights"
  "LinkTokenCreateRequestCreditPartnerInsights"
  "LinkTokenCreateRequestDepositSwitch"
  "LinkTokenCreateRequestEmployment"
  "LinkTokenCreateRequestEmploymentBankIncome"
  "LinkTokenCreateRequestIdentityVerification"
  "LinkTokenCreateRequestIncomeVerification"
  "LinkTokenCreateRequestIncomeVerificationBankIncome"
  "LinkTokenCreateRequestIncomeVerificationPayrollIncome"
  "LinkTokenCreateRequestPaymentConfiguration"
  "LinkTokenCreateRequestPaymentInitiation"
  "LinkTokenCreateRequestStatements"
  "LinkTokenCreateRequestTransfer"
  "LinkTokenCreateRequestUpdate"
  "LinkTokenCreateRequestUser"
  "LinkTokenCreateRequestUserName"
  "LinkTokenCreateRequestUser_address" # This becomes "LinkTokenCreateRequestUserAddress"
  "LinkTokenCreateRequestUser_id_number" # This becomes "LinkTokenCreateRequestUserIdNumber"
  "LinkTokenCreateRequestUser_name" # This becomes "LinkTokenCreateRequestUserName"
  "LinkTokenCreateRequestUserStatedIncomeSource"
  "LinkTokenCreateResponse"
  "LinkTokenEUConfig" # "EU" gets normalized to "Eu", but it needs to be the original name here
  "LinkTokenInvestments"
  "LinkTokenInvestmentsAuth"
  "LinkTokenTransactions"
  "LoanAccountSubtype"
  "LoanFilter"
  "OtherAccountSubtype"
  "OtherFilter"
  "PrismCashScoreVersion"
  "PrismFirstDetectVersion"
  "PrismInsightsVersion"
  "PrismProduct"
  "PrismVersions"
  "Products"
  "TransactionsGetRequest"
  "TransactionsGetRequestOptions"
  "UserStatedIncomeSourceCategory"
  "UserStatedIncomeSourceFrequency"
  "UserStatedIncomeSourcePayType"

  "PlaidError"
  "ItemPublicTokenExchangeRequest"
  "ItemPublicTokenExchangeResponse"
  "TransactionsGetResponse"
  "PlaidErrorType"
  "AccountBase"
  "Transaction"
  "Item"
  "AccountBalance"
  "AccountType"
  "AccountSubtype"
  "AccountVerificationInsights"
  "AccountHolderCategory"
  "Location"
  "PaymentMeta"
  "PersonalFinanceCategory"
  "TransactionCode"
  "TransactionCounterparty"
  "AccountVerificationInsightsNetworkStatus"
  "AccountVerificationInsightsPreviousReturns"
  "AccountVerificationInsightsAccountNumberFormat"
  "AccountVerificationInsights"
  "CounterpartyType"

)
models_str=$(IFS=:; echo "${models[*]}")

openapi-generator-cli generate -g rust \
  -i https://raw.githubusercontent.com/plaid/plaid-openapi/master/2020-09-14.yml \
  -o . \
  --additional-properties=packageName=plaid,packageVersion=1.0.0,supportAsync=false \
  --openapi-normalizer FILTER="operationId:$apis_str" \
  --global-property apis \
  --global-property apiTests=false \
  --global-property models="$models_str" \
  --global-property modelTests=false \
  --global-property supportingFiles
