/*
 * Overseerr API
 *
 * This is the documentation for the Overseerr API backend.  Two primary authentication methods are supported:  - **Cookie Authentication**: A valid sign-in to the `/auth/plex` or `/auth/local` will generate a valid authentication cookie. - **API Key Authentication**: Sign-in is also possible by passing an `X-Api-Key` header along with a valid API Key generated by Overseerr. 
 *
 * The version of the OpenAPI document: v1.33.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetUserSettingsPassword2XxResponse {
    #[serde(rename = "hasPassword", skip_serializing_if = "Option::is_none")]
    pub has_password: Option<bool>,
}

impl GetUserSettingsPassword2XxResponse {
    pub fn new() -> GetUserSettingsPassword2XxResponse {
        GetUserSettingsPassword2XxResponse {
            has_password: None,
        }
    }
}

