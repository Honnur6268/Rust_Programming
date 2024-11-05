use serde::{Deserialize, Serialize};

use crate::constants::{status_enum::Status, timestamp::current_timestamp};

// Common Response for all API
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: Status,
    pub message: String,
    pub timestamp: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success_response(message: &str, data: T) -> Self {
        ApiResponse {
            status: Status::SUCCESS,
            message: message.to_string(),
            timestamp: current_timestamp(),
            data: Some(data),
        }
    }

    pub fn failure_response(message: &str) -> Self {
        ApiResponse {
            status: Status::FAILURE,
            message: message.to_string(),
            timestamp: current_timestamp(),
            data: None,
        }
    }
}

// Validation Response for request validation
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResponse<T> {
    pub message: T,
}

impl<T> ValidationResponse<T> {
    pub fn validation_error_response(message: T) -> Self {
        ValidationResponse { message }
    }
}
