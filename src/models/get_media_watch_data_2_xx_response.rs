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
pub struct GetMediaWatchData2XxResponse {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<models::GetMediaWatchData2XxResponseData>>,
    #[serde(rename = "data4k", skip_serializing_if = "Option::is_none")]
    pub data4k: Option<Box<models::GetMediaWatchData2XxResponseData>>,
}

impl GetMediaWatchData2XxResponse {
    pub fn new() -> GetMediaWatchData2XxResponse {
        GetMediaWatchData2XxResponse {
            data: None,
            data4k: None,
        }
    }
}

