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
pub struct MediaRequestModifiedBy {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "plexToken", skip_serializing_if = "Option::is_none")]
    pub plex_token: Option<String>,
    #[serde(rename = "plexUsername", skip_serializing_if = "Option::is_none")]
    pub plex_username: Option<String>,
    #[serde(rename = "userType", skip_serializing_if = "Option::is_none")]
    pub user_type: Option<i32>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<f64>,
    #[serde(rename = "avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "requestCount", skip_serializing_if = "Option::is_none")]
    pub request_count: Option<f64>,
}

impl MediaRequestModifiedBy {
    pub fn new(id: i32, email: String, created_at: String, updated_at: String) -> MediaRequestModifiedBy {
        MediaRequestModifiedBy {
            id,
            email,
            username: None,
            plex_token: None,
            plex_username: None,
            user_type: None,
            permissions: None,
            avatar: None,
            created_at,
            updated_at,
            request_count: None,
        }
    }
}

