use actix_web::http::StatusCode;

use crate::{constants::response_messages, response::api_response::ApiResponse};

pub struct ServiceError<T>{
    pub response: ApiResponse<T>,
    pub status_code: StatusCode
}

pub type ServiceResponse<T> = Result<ApiResponse<T>, ServiceError<T>>;

pub fn handle_db_error<T>(e: mongodb::error::Error) -> ServiceError<T>{
    log::error!("Error fetching: {:?}", e);
    ServiceError {
        response: ApiResponse::failure_response(response_messages::SERVER_ERROR),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub fn handle_conversion_error<T>(e: mongodb::error::Error) -> ServiceError<T>{
    log::error!("Error while converting cursor to vector: {:?}", e);
        ServiceError {
            response: ApiResponse::failure_response(response_messages::SERVER_ERROR),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
}