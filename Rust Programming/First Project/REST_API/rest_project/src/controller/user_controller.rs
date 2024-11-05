use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use validator::Validate;

use crate::{
    constants::response_messages,
    db::database::Database,
    request::{user_request::UserRequest, user_update_request::UserUpdateRequest},
    response::api_response::ApiResponse,
    service::user_service,
    utils::validator::handle_validation_errors,
};

#[post("/add-user")]
async fn add_user(data: web::Data<Database>, user: web::Json<UserRequest>) -> HttpResponse {
    if let Err(validation_errors) = user.validate() {
        return handle_validation_errors(&validation_errors);
    }

    match user_service::create_user(&data, user.into_inner()).await {
        Ok(response) => HttpResponse::Created().json(response),
        Err(err) => HttpResponse::build(err.status_code).json(err.response),
    }
}

#[get("/get-all-users")]
async fn get_all_users(client: web::Data<Database>) -> HttpResponse {
    match user_service::get_all_users(&client).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => HttpResponse::build(err.status_code).json(err.response),
    }
}

#[delete("/delete-user/{user_id}")]
async fn delete_user_by_id(
    client: web::Data<Database>,
    user_id: web::Path<String>,
) -> HttpResponse {
    match user_service::delete_user_by_id(&client, &user_id).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => HttpResponse::build(err.status_code).json(err.response),
    }
}

// #[get("/user-by-id/{user_id}")]
// async fn get_user_by_id(client: web::Data<Client>, user_id: web::Path<String>) -> HttpResponse {
//     match user_service::get_user_by_id(&client, &user_id).await {
//         Ok(response) => HttpResponse::Ok().json(response),
//         Err(err) => HttpResponse::build(err.status_code).json(err.response),
//     }
// }

#[get("/user-by-id")]
async fn get_user_by_id(req: HttpRequest, client: web::Data<Database>) -> HttpResponse {
    let user_id = match req.headers().get("user_id") {
        Some(value) => value.to_str().unwrap_or(""),
        None => {
            let response = ApiResponse::<()>::failure_response(response_messages::HEADER_MISSING);
            return HttpResponse::BadRequest().json(response);
        }
    };
    match user_service::get_user_by_id(&client, &user_id).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => HttpResponse::build(err.status_code).json(err.response),
    }
}

#[put("/update-user")]
async fn update_user(
    client: web::Data<Database>,
    user_update_request: web::Json<UserUpdateRequest>,
) -> HttpResponse {
    if let Err(validation_errors) = user_update_request.validate() {
        return handle_validation_errors(&validation_errors);
    }

    match user_service::update_user(&client, user_update_request.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => HttpResponse::build(err.status_code).json(err.response),
    }
}
