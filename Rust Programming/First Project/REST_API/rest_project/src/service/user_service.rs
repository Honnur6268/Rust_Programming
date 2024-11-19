use actix_web::http::StatusCode;
use futures_util::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use sha256::digest;
use uuid::Uuid;

use crate::{
    constants::{response_messages::{self, LOGIN_SUCCESS, USER_NOT_FOUND}, timestamp::current_timestamp},
    db::database::Database,
    model::{common_fields::CommonFields, user_model::User},
    request::{login_request::LoginRequest, user_request::UserRequest, user_update_request::UserUpdateRequest},
    response::api_response::{ApiResponse, AuthResponse},
    utils::{error::{handle_conversion_error, handle_db_error, ServiceError, ServiceResponse}, jwt_util::encode_jwt},
};

pub async fn create_user(client: &Database, user_requset: UserRequest) -> ServiceResponse<User> {
    // Reference to the collection in the database
    // let collection = data.database(&MONGO_DB_NAME).collection("users");
    let collection = Database::get_users_collection(client);

    // construct a query to check if the user already exists
    let is_user_exist = doc! {
        "email": &user_requset.email
    };

    // check if the user already exists
    match collection.find_one(is_user_exist.clone()).await {
        Ok(Some(_)) => {
            log::info!("User with email {:?} already exists", &user_requset.email);
            let response: ApiResponse<User> =
                ApiResponse::<User>::failure_response(response_messages::USER_ALREADY_EXISTS);
            return Err(ServiceError {
                response,
                status_code: StatusCode::CONFLICT,
            });
        }
        Ok(None) => {
            log::info!("User does not exist, creating new user");

            let new_user = User {
                _id: ObjectId::new(),
                user_id: Uuid::new_v4().to_string(),
                name: user_requset.name.clone(),
                email: user_requset.email.clone(),
                mobile_number: user_requset.mobile_number.clone(),
                password: digest(&user_requset.password),
                common_fields: CommonFields {
                    created_at: current_timestamp(),
                    updated_at: current_timestamp(),
                },
            };

            // convert the user struct to BSON document (optiona)
            // let user_doc = match bson::to_document(&new_user) {
            //     Ok(doc) => doc,
            //     Err(e) => {
            //         log::error!("Failed to convert user to BSON: {:?}", e);
            //         let response: ApiResponse<User> =
            //             ApiResponse::failure_response(response_messages::SERVER_ERROR);
            //         return Err(ServiceError {
            //             response,
            //             status_code: StatusCode::INTERNAL_SERVER_ERROR,
            //         });
            //     }
            // };

            match collection.insert_one(new_user.clone()).await {
                Ok(_) => {
                    let response = ApiResponse::success_response(
                        response_messages::USER_ADDED_SUCCESSFULLY,
                        new_user,
                    );
                    Ok(response)
                }
                Err(e) => {
                    log::error!("Error While adding new user: {:?}", e);
                    let response: ApiResponse<User> =
                        ApiResponse::failure_response(response_messages::ERROR_ADDING_NEW_USER);
                    Err(ServiceError {
                        response,
                        status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    })
                }
            }
        }
        Err(e) => {
            log::error!("Error While finding user: {:?}", e);

            let response: ApiResponse<User> =
                ApiResponse::failure_response(response_messages::SERVER_ERROR);
            Err(ServiceError {
                response,
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            })
        }
    }
}

pub async fn authenticate(client: &Database, login_reuqest: LoginRequest) -> ServiceResponse<AuthResponse>{

    let collection = Database::get_users_collection(client);

    // construct a query to check if the user already exists
    let filter = doc! {
        "email": &login_reuqest.email,
        "password": digest(&login_reuqest.password)
    };

    let is_user_exists = collection.find_one(filter.clone()).await
    .map_err(handle_db_error::<AuthResponse>)?;

    if is_user_exists.is_none() {
        return Err(ServiceError {
            response: ApiResponse::failure_response(USER_NOT_FOUND),
            status_code: StatusCode::UNAUTHORIZED,
        });
    }

    let user = is_user_exists.unwrap();
    let token = encode_jwt(user.email.clone(), user.user_id.clone());
    
    let auth_response = AuthResponse{
        email: user.email.clone(),
        user_id: user.user_id.clone(),
        token: token.unwrap(),
    };
    let success_response =
        ApiResponse::success_response(LOGIN_SUCCESS, auth_response);
    Ok(success_response)
}

pub async fn get_all_users(data: &Database) -> ServiceResponse<Vec<User>> {
    // Reference to the collection in the database
    // let collection = client.database(&MONGO_DB_NAME).collection("users");
    let collection = Database::get_users_collection(data);

    let cursor = collection
        .find(doc! {})
        .await
        .map_err(handle_db_error::<Vec<User>>)?;

    // let users: Vec<User> = match cursor.try_collect().await {
    //     Ok(users) => users,
    //     Err(e) => {
    //         log::error!("Error while converting cursor to vector: {:?}", e);
    //         let response: ApiResponse<Vec<User>> =
    //             ApiResponse::<Vec<User>>::failure_response(response_messages::SERVER_ERROR);
    //         return Err(ServiceError {
    //             response,
    //             status_code: StatusCode::INTERNAL_SERVER_ERROR,
    //         });
    //     }
    // };

    let users: Vec<User> = cursor
        .try_collect()
        .await
        .map_err(handle_conversion_error::<Vec<User>>)?;

    if users.is_empty() {
        let response: ApiResponse<Vec<User>> =
            ApiResponse::<Vec<User>>::failure_response(response_messages::NO_DATA_FOUND);
        return Err(ServiceError {
            response,
            status_code: StatusCode::NOT_FOUND,
        });
    }
    let response =
        ApiResponse::success_response(response_messages::ALL_USER_FETCHED_SUCCESSFULLY, users);
    Ok(response)
}

pub async fn delete_user_by_id(client: &Database, user_id: &str) -> ServiceResponse<()> {
    // Reference to the collection in the database
    // let collection: Collection<User> = client.database(&MONGO_DB_NAME).collection("users");
    let collection = Database::get_users_collection(client);

    let filter = doc! {
        "user_id": user_id
    };

    let existin_user = collection
        .find_one(filter.clone())
        .await
        .map_err(handle_db_error::<()>)?;

    if existin_user.is_none() {
        return Err(ServiceError {
            response: ApiResponse::failure_response(response_messages::USER_NOT_FOUND),
            status_code: StatusCode::NOT_FOUND,
        });
    }

    let is_user_deleted = collection
        .delete_one(filter)
        .await
        .map_err(handle_db_error::<()>)?;

    if is_user_deleted.deleted_count == 0 {
        return Err(ServiceError {
            response: ApiResponse::failure_response(response_messages::UNABLE_TO_DELETE_USER),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        });
    }

    let success_response =
        ApiResponse::success_response(response_messages::USER_DELETD_SUCCESSFULLY, ());
    Ok(success_response)
}

pub async fn get_user_by_id(client: &Database, user_id: &str) -> ServiceResponse<User> {
    // Reference to the collection in the database
    // let collection: Collection<User> = client.database(&MONGO_DB_NAME).collection("users");
    let collection = Database::get_users_collection(client);

    let filter = doc! {
        "user_id": user_id
    };

    let user = collection
        .find_one(filter.clone())
        .await
        .map_err(handle_db_error::<User>)?;

    match user {
        Some(user) => {
            let response =
                ApiResponse::success_response(response_messages::USER_FETCHED_SUCCESSFULLY, user);
            Ok(response)
        }
        None => {
            let response = ApiResponse::failure_response(response_messages::USER_NOT_FOUND);
            Err(ServiceError {
                response,
                status_code: StatusCode::NOT_FOUND,
            })
        }
    }
}

pub async fn update_user(
    client: &Database,
    user_update_requset: UserUpdateRequest,
) -> ServiceResponse<User> {
    // Reference to the collection in the database
    // let collection = data.database(&MONGO_DB_NAME).collection("users");
    let collection = Database::get_users_collection(client);

    // construct a query to check if the user already exists
    let is_user_exist = doc! {
        "user_id": &user_update_requset.user_id
    };

    let existin_user = collection
        .find_one(is_user_exist.clone())
        .await
        .map_err(handle_db_error::<User>)?;

    if existin_user.is_none() {
        return Err(ServiceError {
            response: ApiResponse::failure_response(response_messages::USER_NOT_FOUND),
            status_code: StatusCode::NOT_FOUND,
        });
    }

    let email_check_filter = doc! {
        "email": &user_update_requset.email,
        "user_id": { "$ne": &user_update_requset.user_id }
    };

    let email_exists = collection
        .find_one(email_check_filter)
        .await
        .map_err(handle_db_error::<User>)?;
    if email_exists.is_some() {
        return Err(ServiceError {
            response: ApiResponse::failure_response(response_messages::USER_EMAIL_EXISTS),
            status_code: StatusCode::CONFLICT,
        });
    }

    let update_data = doc! {"$set": {
        "name": &user_update_requset.name,
        "email": &user_update_requset.email,
        "common_fields.updated_at": current_timestamp(), // Include timestamp for last updated
    }};

    let update_result = collection
        .update_one(is_user_exist.clone(), update_data)
        .await
        .map_err(handle_db_error::<User>)?;

    if update_result.matched_count == 0 {
        return Err(ServiceError {
            response: ApiResponse::failure_response(response_messages::UNABLE_TO_UPDATE_USER),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        });
    }

    let updated_user = collection
        .find_one(is_user_exist)
        .await
        .map_err(handle_db_error::<User>)?;

    match updated_user {
        Some(user) => {
            let response =
                ApiResponse::success_response(response_messages::USER_UPDATED_SUCCESSFULLY, user);
            Ok(response)
        }
        None => {
            let response = ApiResponse::failure_response(response_messages::UNABLE_TO_UPDATE_USER);
            Err(ServiceError {
                response,
                status_code: StatusCode::NOT_FOUND,
            })
        }
    }
}
