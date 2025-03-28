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
pub struct NotificationEmailSettingsOptions {
    #[serde(rename = "emailFrom", skip_serializing_if = "Option::is_none")]
    pub email_from: Option<String>,
    #[serde(rename = "senderName", skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
    #[serde(rename = "smtpHost", skip_serializing_if = "Option::is_none")]
    pub smtp_host: Option<String>,
    #[serde(rename = "smtpPort", skip_serializing_if = "Option::is_none")]
    pub smtp_port: Option<f64>,
    #[serde(rename = "secure", skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
    #[serde(rename = "ignoreTls", skip_serializing_if = "Option::is_none")]
    pub ignore_tls: Option<bool>,
    #[serde(rename = "requireTls", skip_serializing_if = "Option::is_none")]
    pub require_tls: Option<bool>,
    #[serde(rename = "authUser", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub auth_user: Option<Option<String>>,
    #[serde(rename = "authPass", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub auth_pass: Option<Option<String>>,
    #[serde(rename = "allowSelfSigned", skip_serializing_if = "Option::is_none")]
    pub allow_self_signed: Option<bool>,
}

impl NotificationEmailSettingsOptions {
    pub fn new() -> NotificationEmailSettingsOptions {
        NotificationEmailSettingsOptions {
            email_from: None,
            sender_name: None,
            smtp_host: None,
            smtp_port: None,
            secure: None,
            ignore_tls: None,
            require_tls: None,
            auth_user: None,
            auth_pass: None,
            allow_self_signed: None,
        }
    }
}

