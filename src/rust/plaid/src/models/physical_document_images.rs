/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PhysicalDocumentImages : URLs for downloading original and cropped images for this document submission. The URLs are designed to only allow downloading, not hot linking, so the URL will only serve the document image for 60 seconds before expiring. The expiration time is 60 seconds after the `GET` request for the associated Identity Verification attempt. A new expiring URL is generated with each request, so you can always rerequest the Identity Verification attempt if one of your URLs expires.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PhysicalDocumentImages {
    /// Temporary URL that expires after 60 seconds for downloading the uncropped original image of the front of the document.
    #[serde(rename = "original_front", deserialize_with = "Option::deserialize")]
    pub original_front: Option<String>,
    /// Temporary URL that expires after 60 seconds for downloading the original image of the back of the document. Might be null if the back of the document was not collected.
    #[serde(rename = "original_back", deserialize_with = "Option::deserialize")]
    pub original_back: Option<String>,
    /// Temporary URL that expires after 60 seconds for downloading a cropped image containing just the front of the document.
    #[serde(rename = "cropped_front", deserialize_with = "Option::deserialize")]
    pub cropped_front: Option<String>,
    /// Temporary URL that expires after 60 seconds for downloading a cropped image containing just the back of the document. Might be null if the back of the document was not collected.
    #[serde(rename = "cropped_back", deserialize_with = "Option::deserialize")]
    pub cropped_back: Option<String>,
    /// Temporary URL that expires after 60 seconds for downloading a crop of just the user's face from the document image. Might be null if the document does not contain a face photo.
    #[serde(rename = "face", deserialize_with = "Option::deserialize")]
    pub face: Option<String>,
}

impl PhysicalDocumentImages {
    /// URLs for downloading original and cropped images for this document submission. The URLs are designed to only allow downloading, not hot linking, so the URL will only serve the document image for 60 seconds before expiring. The expiration time is 60 seconds after the `GET` request for the associated Identity Verification attempt. A new expiring URL is generated with each request, so you can always rerequest the Identity Verification attempt if one of your URLs expires.
    pub fn new(original_front: Option<String>, original_back: Option<String>, cropped_front: Option<String>, cropped_back: Option<String>, face: Option<String>) -> PhysicalDocumentImages {
        PhysicalDocumentImages {
            original_front,
            original_back,
            cropped_front,
            cropped_back,
            face,
        }
    }
}


