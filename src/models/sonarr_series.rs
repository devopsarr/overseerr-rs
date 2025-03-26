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
pub struct SonarrSeries {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "sortTitle", skip_serializing_if = "Option::is_none")]
    pub sort_title: Option<String>,
    #[serde(rename = "seasonCount", skip_serializing_if = "Option::is_none")]
    pub season_count: Option<f64>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "overview", skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(rename = "airTime", skip_serializing_if = "Option::is_none")]
    pub air_time: Option<String>,
    #[serde(rename = "images", skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<models::SonarrSeriesImagesInner>>,
    #[serde(rename = "remotePoster", skip_serializing_if = "Option::is_none")]
    pub remote_poster: Option<String>,
    #[serde(rename = "seasons", skip_serializing_if = "Option::is_none")]
    pub seasons: Option<Vec<models::SonarrSeriesSeasonsInner>>,
    #[serde(rename = "year", skip_serializing_if = "Option::is_none")]
    pub year: Option<f64>,
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "profileId", skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<f64>,
    #[serde(rename = "languageProfileId", skip_serializing_if = "Option::is_none")]
    pub language_profile_id: Option<f64>,
    #[serde(rename = "seasonFolder", skip_serializing_if = "Option::is_none")]
    pub season_folder: Option<bool>,
    #[serde(rename = "monitored", skip_serializing_if = "Option::is_none")]
    pub monitored: Option<bool>,
    #[serde(rename = "useSceneNumbering", skip_serializing_if = "Option::is_none")]
    pub use_scene_numbering: Option<bool>,
    #[serde(rename = "runtime", skip_serializing_if = "Option::is_none")]
    pub runtime: Option<f64>,
    #[serde(rename = "tvdbId", skip_serializing_if = "Option::is_none")]
    pub tvdb_id: Option<f64>,
    #[serde(rename = "tvRageId", skip_serializing_if = "Option::is_none")]
    pub tv_rage_id: Option<f64>,
    #[serde(rename = "tvMazeId", skip_serializing_if = "Option::is_none")]
    pub tv_maze_id: Option<f64>,
    #[serde(rename = "firstAired", skip_serializing_if = "Option::is_none")]
    pub first_aired: Option<String>,
    #[serde(rename = "lastInfoSync", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_info_sync: Option<Option<String>>,
    #[serde(rename = "seriesType", skip_serializing_if = "Option::is_none")]
    pub series_type: Option<String>,
    #[serde(rename = "cleanTitle", skip_serializing_if = "Option::is_none")]
    pub clean_title: Option<String>,
    #[serde(rename = "imdbId", skip_serializing_if = "Option::is_none")]
    pub imdb_id: Option<String>,
    #[serde(rename = "titleSlug", skip_serializing_if = "Option::is_none")]
    pub title_slug: Option<String>,
    #[serde(rename = "certification", skip_serializing_if = "Option::is_none")]
    pub certification: Option<String>,
    #[serde(rename = "genres", skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<String>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "added", skip_serializing_if = "Option::is_none")]
    pub added: Option<String>,
    #[serde(rename = "ratings", skip_serializing_if = "Option::is_none")]
    pub ratings: Option<Vec<models::SonarrSeriesRatingsInner>>,
    #[serde(rename = "qualityProfileId", skip_serializing_if = "Option::is_none")]
    pub quality_profile_id: Option<f64>,
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<f64>>,
    #[serde(rename = "rootFolderPath", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub root_folder_path: Option<Option<String>>,
    #[serde(rename = "addOptions", skip_serializing_if = "Option::is_none")]
    pub add_options: Option<Vec<models::SonarrSeriesAddOptionsInner>>,
}

impl SonarrSeries {
    pub fn new() -> SonarrSeries {
        SonarrSeries {
            title: None,
            sort_title: None,
            season_count: None,
            status: None,
            overview: None,
            network: None,
            air_time: None,
            images: None,
            remote_poster: None,
            seasons: None,
            year: None,
            path: None,
            profile_id: None,
            language_profile_id: None,
            season_folder: None,
            monitored: None,
            use_scene_numbering: None,
            runtime: None,
            tvdb_id: None,
            tv_rage_id: None,
            tv_maze_id: None,
            first_aired: None,
            last_info_sync: None,
            series_type: None,
            clean_title: None,
            imdb_id: None,
            title_slug: None,
            certification: None,
            genres: None,
            tags: None,
            added: None,
            ratings: None,
            quality_profile_id: None,
            id: None,
            root_folder_path: None,
            add_options: None,
        }
    }
}

