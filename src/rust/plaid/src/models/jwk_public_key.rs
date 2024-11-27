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

/// JwkPublicKey : A JSON Web Key (JWK) that can be used in conjunction with [JWT libraries](https://jwt.io/#libraries-io) to verify Plaid webhooks
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JwkPublicKey {
    /// The alg member identifies the cryptographic algorithm family used with the key.
    #[serde(rename = "alg")]
    pub alg: String,
    /// The crv member identifies the cryptographic curve used with the key.
    #[serde(rename = "crv")]
    pub crv: String,
    /// The kid (Key ID) member can be used to match a specific key. This can be used, for instance, to choose among a set of keys within the JWK during key rollover.
    #[serde(rename = "kid")]
    pub kid: String,
    /// The kty (key type) parameter identifies the cryptographic algorithm family used with the key, such as RSA or EC.
    #[serde(rename = "kty")]
    pub kty: String,
    /// The use (public key use) parameter identifies the intended use of the public key.
    #[serde(rename = "use")]
    pub r#use: String,
    /// The x member contains the x coordinate for the elliptic curve point, provided as a base64url-encoded string of the coordinate's big endian representation.
    #[serde(rename = "x")]
    pub x: String,
    /// The y member contains the y coordinate for the elliptic curve point, provided as a base64url-encoded string of the coordinate's big endian representation.
    #[serde(rename = "y")]
    pub y: String,
    /// The timestamp when the key was created, in Unix time.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    /// The timestamp when the key expired, in Unix time.
    #[serde(rename = "expired_at", deserialize_with = "Option::deserialize")]
    pub expired_at: Option<i32>,
}

impl JwkPublicKey {
    /// A JSON Web Key (JWK) that can be used in conjunction with [JWT libraries](https://jwt.io/#libraries-io) to verify Plaid webhooks
    pub fn new(alg: String, crv: String, kid: String, kty: String, r#use: String, x: String, y: String, created_at: i32, expired_at: Option<i32>) -> JwkPublicKey {
        JwkPublicKey {
            alg,
            crv,
            kid,
            kty,
            r#use,
            x,
            y,
            created_at,
            expired_at,
        }
    }
}

