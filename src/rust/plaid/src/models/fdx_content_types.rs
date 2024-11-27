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

/// FdxContentTypes : Types of document formats. (Suggested values)
/// Types of document formats. (Suggested values)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FdxContentTypes {
    #[serde(rename = "application/pdf")]
    ApplicationSlashPdf,
    #[serde(rename = "image/gif")]
    ImageSlashGif,
    #[serde(rename = "image/jpeg")]
    ImageSlashJpeg,
    #[serde(rename = "image/tiff")]
    ImageSlashTiff,
    #[serde(rename = "image/png")]
    ImageSlashPng,
    #[serde(rename = "application/json")]
    ApplicationSlashJson,

}

impl std::fmt::Display for FdxContentTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ApplicationSlashPdf => write!(f, "application/pdf"),
            Self::ImageSlashGif => write!(f, "image/gif"),
            Self::ImageSlashJpeg => write!(f, "image/jpeg"),
            Self::ImageSlashTiff => write!(f, "image/tiff"),
            Self::ImageSlashPng => write!(f, "image/png"),
            Self::ApplicationSlashJson => write!(f, "application/json"),
        }
    }
}

impl Default for FdxContentTypes {
    fn default() -> FdxContentTypes {
        Self::ApplicationSlashPdf
    }
}

