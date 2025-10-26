use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Country not found")]
    NotFound,
    
    #[error("Validation failed")]
    ValidationError(HashMap<String, String>),
    
    #[error("External data source unavailable")]
    ExternalApiError(String),
    
    #[error("Database error")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("Internal server error")]
    InternalError,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<serde_json::Value>,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ApiError::ExternalApiError(_) => StatusCode::SERVICE_UNAVAILABLE,
            ApiError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        log::error!("API Error: {:?}", self);
        
        match self {
            ApiError::NotFound => {
                HttpResponse::NotFound().json(ErrorResponse {
                    error: "Country not found".to_string(),
                    details: None,
                })
            }
            ApiError::ValidationError(details) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    error: "Validation failed".to_string(),
                    details: Some(serde_json::to_value(details).unwrap()),
                })
            }
            ApiError::ExternalApiError(api_name) => {
                HttpResponse::ServiceUnavailable().json(ErrorResponse {
                    error: "External data source unavailable".to_string(),
                    details: Some(serde_json::Value::String(format!("Could not fetch data from {}", api_name))),
                })
            }
            ApiError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Internal server error".to_string(),
                    details: None,
                })
            }
            ApiError::InternalError => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Internal server error".to_string(),
                    details: None,
                })
            }
        }
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(e: reqwest::Error) -> Self {
        ApiError::ExternalApiError(format!("External API: {}", e))
    }
}

impl From<std::io::Error> for ApiError {
    fn from(_: std::io::Error) -> Self {
        ApiError::InternalError
    }
}

impl From<image::ImageError> for ApiError {
    fn from(_: image::ImageError) -> Self {
        ApiError::InternalError
    }
}