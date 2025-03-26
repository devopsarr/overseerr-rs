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
pub struct GetAbout2XxResponse {
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "totalRequests", skip_serializing_if = "Option::is_none")]
    pub total_requests: Option<f64>,
    #[serde(rename = "totalMediaItems", skip_serializing_if = "Option::is_none")]
    pub total_media_items: Option<f64>,
    #[serde(rename = "tz", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tz: Option<Option<String>>,
    #[serde(rename = "appDataPath", skip_serializing_if = "Option::is_none")]
    pub app_data_path: Option<String>,
}

impl GetAbout2XxResponse {
    pub fn new() -> GetAbout2XxResponse {
        GetAbout2XxResponse {
            version: None,
            total_requests: None,
            total_media_items: None,
            tz: None,
            app_data_path: None,
        }
    }
}

