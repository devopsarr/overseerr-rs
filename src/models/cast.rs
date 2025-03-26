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
pub struct Cast {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<f64>,
    #[serde(rename = "castId", skip_serializing_if = "Option::is_none")]
    pub cast_id: Option<f64>,
    #[serde(rename = "character", skip_serializing_if = "Option::is_none")]
    pub character: Option<String>,
    #[serde(rename = "creditId", skip_serializing_if = "Option::is_none")]
    pub credit_id: Option<String>,
    #[serde(rename = "gender", skip_serializing_if = "Option::is_none")]
    pub gender: Option<f64>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<f64>,
    #[serde(rename = "profilePath", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub profile_path: Option<Option<String>>,
}

impl Cast {
    pub fn new() -> Cast {
        Cast {
            id: None,
            cast_id: None,
            character: None,
            credit_id: None,
            gender: None,
            name: None,
            order: None,
            profile_path: None,
        }
    }
}

