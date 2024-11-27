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

/// DocumentRiskSummary : A summary across all risk signals associated with a document
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DocumentRiskSummary {
    /// A number between 0 and 100, inclusive, where a score closer to 0 indicates a document is likely to be trustworthy and a score closer to 100 indicates a document is likely to be fraudulent. You can automatically reject documents with a high risk score, automatically accept documents with a low risk score, and manually review documents in between. We suggest starting with a threshold of 80 for auto-rejection and 20 for auto-acceptance. As you gather more data points on typical risk scores for your use case, you can tune these parameters to reduce the number of documents undergoing manual review.
    #[serde(rename = "risk_score", deserialize_with = "Option::deserialize")]
    pub risk_score: Option<f64>,
}

impl DocumentRiskSummary {
    /// A summary across all risk signals associated with a document
    pub fn new(risk_score: Option<f64>) -> DocumentRiskSummary {
        DocumentRiskSummary {
            risk_score,
        }
    }
}

