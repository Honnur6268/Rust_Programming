use actix_web::http::StatusCode;
use mongodb::bson::{doc, oid::ObjectId};
use uuid::Uuid;

use crate::{
    constants::{response_messages, timestamp::current_timestamp},
    db::database::Database,
    model::{common_fields::CommonFields, product_model::Product},
    request::product_request::ProductRequest,
    response::api_response::ApiResponse,
    utils::error::{ServiceError, ServiceResponse},
};

pub async fn create_product(
    client: &Database,
    product_request: ProductRequest,
) -> ServiceResponse<Product> {
    let collection = Database::get_products_collection(client);

    // construct a query to check if the product already exists
    let is_product_exists = doc! {
        "product_sku": &product_request.product_sku
    };

    // check if the Product already exists
    match collection.find_one(is_product_exists.clone()).await {
        Ok(Some(_)) => {
            log::info!(
                "Product with SKU Id {:?} already exists",
                &product_request.product_sku
            );
            let response: ApiResponse<Product> =
                ApiResponse::<Product>::failure_response(response_messages::PRODUCT_ALREADY_EXISTS);
            return Err(ServiceError {
                response,
                status_code: StatusCode::CONFLICT,
            });
        }
        Ok(None) => {
            log::info!("Product does not exist, creating new Product");

            let new_product = Product {
                _id: ObjectId::new(),
                product_id: Uuid::new_v4().to_string(),
                product_name: product_request.product_name.clone(),
                product_sku: product_request.product_sku.clone(),
                price: product_request.price.clone(),
                common_fields: CommonFields {
                    created_at: current_timestamp(),
                    updated_at: current_timestamp(),
                },
            };

            match collection.insert_one(new_product.clone()).await {
                Ok(_) => {
                    let response = ApiResponse::success_response(
                        response_messages::PRODUCT_ADDED_SUCCESSFULLY,
                        new_product,
                    );
                    Ok(response)
                }
                Err(e) => {
                    log::error!("Error While adding new product: {:?}", e);
                    let response: ApiResponse<Product> =
                        ApiResponse::failure_response(response_messages::ERROR_ADDING_NEW_PRODUCT);
                    Err(ServiceError {
                        response,
                        status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    })
                }
            }
        }
        Err(e) => {
            log::error!("Error While finding product: {:?}", e);

            let response: ApiResponse<Product> =
                ApiResponse::failure_response(response_messages::SERVER_ERROR);
            Err(ServiceError {
                response,
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            })
        }
    }
}
