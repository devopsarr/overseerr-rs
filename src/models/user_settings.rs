/*
 * Overseerr API
 *
 * This is the documentation for the Overseerr API backend.  Two primary authentication methods are supported:  - **Cookie Authentication**: A valid sign-in to the `/auth/plex` or `/auth/local` will generate a valid authentication cookie. - **API Key Authentication**: Sign-in is also possible by passing an `X-Api-Key` header along with a valid API Key generated by Overseerr. 
 *
 * The version of the OpenAPI document: v1.34.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSettings {
    #[serde(rename = "locale", skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "originalLanguage", skip_serializing_if = "Option::is_none")]
    pub original_language: Option<String>,
}

impl UserSettings {
    pub fn new() -> UserSettings {
        UserSettings {
            locale: None,
            region: None,
            original_language: None,
        }
    }
}

