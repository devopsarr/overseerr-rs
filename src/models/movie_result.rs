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
pub struct MovieResult {
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "mediaType")]
    pub media_type: String,
    #[serde(rename = "popularity", skip_serializing_if = "Option::is_none")]
    pub popularity: Option<f64>,
    #[serde(rename = "posterPath", skip_serializing_if = "Option::is_none")]
    pub poster_path: Option<String>,
    #[serde(rename = "backdropPath", skip_serializing_if = "Option::is_none")]
    pub backdrop_path: Option<String>,
    #[serde(rename = "voteCount", skip_serializing_if = "Option::is_none")]
    pub vote_count: Option<f64>,
    #[serde(rename = "voteAverage", skip_serializing_if = "Option::is_none")]
    pub vote_average: Option<f64>,
    #[serde(rename = "genreIds", skip_serializing_if = "Option::is_none")]
    pub genre_ids: Option<Vec<f64>>,
    #[serde(rename = "overview", skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,
    #[serde(rename = "originalLanguage", skip_serializing_if = "Option::is_none")]
    pub original_language: Option<String>,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "originalTitle", skip_serializing_if = "Option::is_none")]
    pub original_title: Option<String>,
    #[serde(rename = "releaseDate", skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,
    #[serde(rename = "adult", skip_serializing_if = "Option::is_none")]
    pub adult: Option<bool>,
    #[serde(rename = "video", skip_serializing_if = "Option::is_none")]
    pub video: Option<bool>,
    #[serde(rename = "mediaInfo", skip_serializing_if = "Option::is_none")]
    pub media_info: Option<Box<models::MediaInfo>>,
}

impl MovieResult {
    pub fn new(id: f64, media_type: String, title: String) -> MovieResult {
        MovieResult {
            id,
            media_type,
            popularity: None,
            poster_path: None,
            backdrop_path: None,
            vote_count: None,
            vote_average: None,
            genre_ids: None,
            overview: None,
            original_language: None,
            title,
            original_title: None,
            release_date: None,
            adult: None,
            video: None,
            media_info: None,
        }
    }
}

