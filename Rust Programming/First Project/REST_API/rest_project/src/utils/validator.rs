// use std::collections::HashMap;

use actix_web::HttpResponse;
use validator::ValidationErrors;

use crate::response::api_response::ValidationResponse;

// Performs request validation and returns a response with the validation error messages
pub fn handle_validation_errors(errors: &ValidationErrors) -> HttpResponse {
    log::error!("Validation Error: {:?}", errors);
    let error_msgs = errors
        .field_errors()
        .iter()
        .map(|(field, errors)| {
            let messages: Vec<String> = errors
                .iter()
                .map(|error| format!("{}: {}", field, error.message.clone().unwrap_or_default()))
                .collect();
            messages
        })
        .flatten()
        .collect::<Vec<String>>()
        .join(", ");

    let response = ValidationResponse::validation_error_response(&error_msgs);
    return HttpResponse::BadRequest().json(response);
}
