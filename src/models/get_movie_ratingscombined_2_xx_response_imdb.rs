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
pub struct GetMovieRatingscombined2XxResponseImdb {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "criticsScore", skip_serializing_if = "Option::is_none")]
    pub critics_score: Option<f64>,
}

impl GetMovieRatingscombined2XxResponseImdb {
    pub fn new() -> GetMovieRatingscombined2XxResponseImdb {
        GetMovieRatingscombined2XxResponseImdb {
            title: None,
            url: None,
            critics_score: None,
        }
    }
}

