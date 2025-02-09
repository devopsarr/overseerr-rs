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
pub struct Issue {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<f64>,
    #[serde(rename = "issueType", skip_serializing_if = "Option::is_none")]
    pub issue_type: Option<f64>,
    #[serde(rename = "media", skip_serializing_if = "Option::is_none")]
    pub media: Option<Box<models::MediaInfo>>,
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Box<models::User>>,
    #[serde(rename = "modifiedBy", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<Box<models::User>>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<models::IssueComment>>,
}

impl Issue {
    pub fn new() -> Issue {
        Issue {
            id: None,
            issue_type: None,
            media: None,
            created_by: None,
            modified_by: None,
            comments: None,
        }
    }
}

