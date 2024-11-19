use crate::{
    constants::{
        config::EXCLUDE_PATHS,
        response_messages::{AUTHORIZATION_HEADER_NAME, INVALID_TOKEN, UNAUTHORIZED},
    },
    response::api_response::ApiResponse,
    utils::jwt_util::decode_jwt,
};
use actix_web::{
    body::{BoxBody, MessageBody},
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    Error, HttpResponse,
};

pub async fn jwt_auth_filter(
    req: ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    // Define the list of paths to exclude from JWT authentication
    // let excluded_paths = vec![
    //     "/api/users/login",    // Login route
    //     "/api/users/register", // Registration route
    //     "/health-check",       // Health check route
    // ];

    // fetching and logging exclude paths from env file
    EXCLUDE_PATHS
        .iter()
        .for_each(|d| log::info!("Excluded Paths: {}", d));

    // Check if the request path is in the excluded paths
    if EXCLUDE_PATHS
        .iter()
        .any(|path| req.path().starts_with(path))
    {
        // Skip authentication check for excluded paths
        return next.call(req).await.map(|res| res.map_into_boxed_body());
    }

    // Check if Authorization header exists
    let auth_header = req.headers().get(AUTHORIZATION_HEADER_NAME);

    // Return Unauthorized if the header is missing
    if auth_header.is_none() {
        let unauthorized_response = ApiResponse::<()>::failure_response(UNAUTHORIZED);
        let response = HttpResponse::Unauthorized().json(unauthorized_response);
        return Ok(req.into_response(response.map_into_boxed_body()));
    }

    // Extract token and decode
    let token = auth_header
        .unwrap()
        .to_str()
        .unwrap()
        .replace("Bearer ", "")
        .to_owned();

    log::info!("Token: {}", token);

    match decode_jwt(token) {
        Ok(_) => {
            let d = next.call(req).await.map(|res| res.map_into_boxed_body());
            log::info!("D: {:?}", d);
            d
        }
        Err(err) => {
            log::info!("Error: {:?}", err);
            let unauthorized_response = ApiResponse::<()>::failure_response(INVALID_TOKEN);
            let response = HttpResponse::Unauthorized().json(unauthorized_response);
            Ok(req.into_response(response.map_into_boxed_body()))
        }
    }
}
