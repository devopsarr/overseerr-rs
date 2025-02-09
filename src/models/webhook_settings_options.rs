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
pub struct WebhookSettingsOptions {
    #[serde(rename = "webhookUrl", skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    #[serde(rename = "authHeader", skip_serializing_if = "Option::is_none")]
    pub auth_header: Option<String>,
    #[serde(rename = "jsonPayload", skip_serializing_if = "Option::is_none")]
    pub json_payload: Option<String>,
}

impl WebhookSettingsOptions {
    pub fn new() -> WebhookSettingsOptions {
        WebhookSettingsOptions {
            webhook_url: None,
            auth_header: None,
            json_payload: None,
        }
    }
}

