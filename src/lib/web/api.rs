use std::str::FromStr;

use base64::{engine::general_purpose, Engine};
use rocket::{serde::json::Json, Responder};
use serde::Serialize;

use crate::ServiceError;

pub const API_KEY_HEADER: &str = "x-api-key";

// Responder enables rocket to respond with this enum type directly
#[derive(Responder, Debug, thiserror::Error, Serialize)]
pub enum ApiKeyError {
    #[error("API key not found")]
    #[response(status = 404, content_type = "json")]
    NotFound(String),
    #[error("invalid API key format")]
    #[response(status = 400, content_type = "json")]
    DecodeError(String),
}

#[derive(Debug, Clone)]
pub struct ApiKey(Vec<u8>);

impl ApiKey {
    pub fn to_base64(&self) -> String {
        general_purpose::STANDARD.encode(self.0.as_slice())
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }
}

impl Default for ApiKey {
    fn default() -> Self {
        let key = (0..16).map(|_| rand::random::<u8>()).collect();

        Self(key)
    }
}

impl FromStr for ApiKey {
    type Err = ApiKeyError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        general_purpose::STANDARD
            .decode(s)
            .map(ApiKey)
            .map_err(|e| Self::Err::DecodeError(e.to_string()))
    }
}

#[derive(Responder, Debug, thiserror::Error)]
pub enum ApiError {
    #[error("not found")]
    #[response(status = 404, content_type = "json")]
    NotFound(Json<String>),

    #[error("server error")]
    #[response(status = 500, content_type = "json")]
    ServerError(Json<String>),

    #[error("client error")]
    #[response(status = 401, content_type = "json")]
    UserError(Json<String>),

    #[error("key error")]
    #[response(status = 400, content_type = "json")]
    KeyError(Json<String>),
}

impl From<ServiceError> for ApiError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::Clip(c) => Self::UserError(Json(format!("clip parsing error: {}", c))),
            ServiceError::NotFound => Self::UserError(Json("entity not found".to_owned())),
            ServiceError::Data(_) => Self::ServerError(Json("a server error occurred".to_owned())),
            ServiceError::PermissionError(msg) => Self::UserError(Json(msg)),
        }
    }
}
