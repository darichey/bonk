openapi-generator-cli generate -g rust \
  -i https://raw.githubusercontent.com/plaid/plaid-openapi/master/2020-09-14.yml \
  -o . \
  --additional-properties=packageName=plaid,packageVersion=1.0.0,supportAsync=false
