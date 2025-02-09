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
pub struct GetSearchCompany2XxResponse {
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<f64>,
    #[serde(rename = "totalPages", skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<f64>,
    #[serde(rename = "totalResults", skip_serializing_if = "Option::is_none")]
    pub total_results: Option<f64>,
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<models::Company>>,
}

impl GetSearchCompany2XxResponse {
    pub fn new() -> GetSearchCompany2XxResponse {
        GetSearchCompany2XxResponse {
            page: None,
            total_pages: None,
            total_results: None,
            results: None,
        }
    }
}

