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
pub struct GetCache2XxResponse {
    #[serde(rename = "imageCache", skip_serializing_if = "Option::is_none")]
    pub image_cache: Option<Box<models::GetCache2XxResponseImageCache>>,
    #[serde(rename = "apiCaches", skip_serializing_if = "Option::is_none")]
    pub api_caches: Option<Vec<models::GetCache2XxResponseApiCachesInner>>,
}

impl GetCache2XxResponse {
    pub fn new() -> GetCache2XxResponse {
        GetCache2XxResponse {
            image_cache: None,
            api_caches: None,
        }
    }
}

