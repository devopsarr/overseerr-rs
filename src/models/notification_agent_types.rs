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
pub struct NotificationAgentTypes {
    #[serde(rename = "discord", skip_serializing_if = "Option::is_none")]
    pub discord: Option<f64>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<f64>,
    #[serde(rename = "pushbullet", skip_serializing_if = "Option::is_none")]
    pub pushbullet: Option<f64>,
    #[serde(rename = "pushover", skip_serializing_if = "Option::is_none")]
    pub pushover: Option<f64>,
    #[serde(rename = "slack", skip_serializing_if = "Option::is_none")]
    pub slack: Option<f64>,
    #[serde(rename = "telegram", skip_serializing_if = "Option::is_none")]
    pub telegram: Option<f64>,
    #[serde(rename = "webhook", skip_serializing_if = "Option::is_none")]
    pub webhook: Option<f64>,
    #[serde(rename = "webpush", skip_serializing_if = "Option::is_none")]
    pub webpush: Option<f64>,
}

impl NotificationAgentTypes {
    pub fn new() -> NotificationAgentTypes {
        NotificationAgentTypes {
            discord: None,
            email: None,
            pushbullet: None,
            pushover: None,
            slack: None,
            telegram: None,
            webhook: None,
            webpush: None,
        }
    }
}

