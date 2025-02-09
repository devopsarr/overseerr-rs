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
pub struct GetMovieRatings2XxResponse {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "year", skip_serializing_if = "Option::is_none")]
    pub year: Option<f64>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "criticsScore", skip_serializing_if = "Option::is_none")]
    pub critics_score: Option<f64>,
    #[serde(rename = "criticsRating", skip_serializing_if = "Option::is_none")]
    pub critics_rating: Option<CriticsRating>,
    #[serde(rename = "audienceScore", skip_serializing_if = "Option::is_none")]
    pub audience_score: Option<f64>,
    #[serde(rename = "audienceRating", skip_serializing_if = "Option::is_none")]
    pub audience_rating: Option<AudienceRating>,
}

impl GetMovieRatings2XxResponse {
    pub fn new() -> GetMovieRatings2XxResponse {
        GetMovieRatings2XxResponse {
            title: None,
            year: None,
            url: None,
            critics_score: None,
            critics_rating: None,
            audience_score: None,
            audience_rating: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CriticsRating {
    #[serde(rename = "Rotten")]
    Rotten,
    #[serde(rename = "Fresh")]
    Fresh,
    #[serde(rename = "Certified Fresh")]
    CertifiedFresh,
}

impl Default for CriticsRating {
    fn default() -> CriticsRating {
        Self::Rotten
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AudienceRating {
    #[serde(rename = "Spilled")]
    Spilled,
    #[serde(rename = "Upright")]
    Upright,
}

impl Default for AudienceRating {
    fn default() -> AudienceRating {
        Self::Spilled
    }
}

