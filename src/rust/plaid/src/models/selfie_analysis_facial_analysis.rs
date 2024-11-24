/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SelfieAnalysisFacialAnalysis : Analysis of the facial features of the selfie when compared to the face in the uploaded document, if one is present.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SelfieAnalysisFacialAnalysis {
    #[serde(rename = "left_eye")]
    pub left_eye: crate::models::SelfieAnalysisFacialAnalysisOutcome,
    #[serde(rename = "right_eye")]
    pub right_eye: crate::models::SelfieAnalysisFacialAnalysisOutcome,
    #[serde(rename = "left_brow")]
    pub left_brow: crate::models::SelfieAnalysisFacialAnalysisOutcome,
    #[serde(rename = "right_brow")]
    pub right_brow: crate::models::SelfieAnalysisFacialAnalysisOutcome,
    #[serde(rename = "forehead")]
    pub forehead: crate::models::SelfieAnalysisFacialAnalysisOutcome,
    #[serde(rename = "middle_forehead")]
    pub middle_forehead: crate::models::SelfieAnalysisFacialAnalysisOutcome,
    #[serde(rename = "nose")]
    pub nose: crate::models::SelfieAnalysisFacialAnalysisOutcome,
    #[serde(rename = "philtrum")]
    pub philtrum: crate::models::SelfieAnalysisFacialAnalysisOutcome,
    #[serde(rename = "mouth")]
    pub mouth: crate::models::SelfieAnalysisFacialAnalysisOutcome,
    #[serde(rename = "jaw")]
    pub jaw: crate::models::SelfieAnalysisFacialAnalysisOutcome,
    #[serde(rename = "left_cheek")]
    pub left_cheek: crate::models::SelfieAnalysisFacialAnalysisOutcome,
    #[serde(rename = "right_cheek")]
    pub right_cheek: crate::models::SelfieAnalysisFacialAnalysisOutcome,
}

impl SelfieAnalysisFacialAnalysis {
    /// Analysis of the facial features of the selfie when compared to the face in the uploaded document, if one is present.
    pub fn new(left_eye: crate::models::SelfieAnalysisFacialAnalysisOutcome, right_eye: crate::models::SelfieAnalysisFacialAnalysisOutcome, left_brow: crate::models::SelfieAnalysisFacialAnalysisOutcome, right_brow: crate::models::SelfieAnalysisFacialAnalysisOutcome, forehead: crate::models::SelfieAnalysisFacialAnalysisOutcome, middle_forehead: crate::models::SelfieAnalysisFacialAnalysisOutcome, nose: crate::models::SelfieAnalysisFacialAnalysisOutcome, philtrum: crate::models::SelfieAnalysisFacialAnalysisOutcome, mouth: crate::models::SelfieAnalysisFacialAnalysisOutcome, jaw: crate::models::SelfieAnalysisFacialAnalysisOutcome, left_cheek: crate::models::SelfieAnalysisFacialAnalysisOutcome, right_cheek: crate::models::SelfieAnalysisFacialAnalysisOutcome) -> SelfieAnalysisFacialAnalysis {
        SelfieAnalysisFacialAnalysis {
            left_eye,
            right_eye,
            left_brow,
            right_brow,
            forehead,
            middle_forehead,
            nose,
            philtrum,
            mouth,
            jaw,
            left_cheek,
            right_cheek,
        }
    }
}


