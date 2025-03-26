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
pub struct DiscoverSlider {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<f64>,
    #[serde(rename = "type")]
    pub r#type: f64,
    #[serde(rename = "title", deserialize_with = "Option::deserialize")]
    pub title: Option<String>,
    #[serde(rename = "isBuiltIn", skip_serializing_if = "Option::is_none")]
    pub is_built_in: Option<bool>,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "data", deserialize_with = "Option::deserialize")]
    pub data: Option<String>,
}

impl DiscoverSlider {
    pub fn new(r#type: f64, title: Option<String>, enabled: bool, data: Option<String>) -> DiscoverSlider {
        DiscoverSlider {
            id: None,
            r#type,
            title,
            is_built_in: None,
            enabled,
            data,
        }
    }
}

