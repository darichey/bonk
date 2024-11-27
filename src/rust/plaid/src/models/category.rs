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

/// Category : Information describing a transaction category
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Category {
    /// An identifying number for the category. `category_id` is a Plaid-specific identifier and does not necessarily correspond to merchant category codes.
    #[serde(rename = "category_id")]
    pub category_id: String,
    /// `place` for physical transactions or `special` for other transactions such as bank charges.
    #[serde(rename = "group")]
    pub group: String,
    /// A hierarchical array of the categories to which this `category_id` belongs.
    #[serde(rename = "hierarchy")]
    pub hierarchy: Vec<String>,
}

impl Category {
    /// Information describing a transaction category
    pub fn new(category_id: String, group: String, hierarchy: Vec<String>) -> Category {
        Category {
            category_id,
            group,
            hierarchy,
        }
    }
}

