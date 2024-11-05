use actix_web::{post, web, HttpResponse};
use validator::Validate;

use crate::{
    db::database::Database, request::product_request::ProductRequest, service::product_service,
    utils::validator::handle_validation_errors,
};

#[post("/add-product")]
async fn add_product(
    data: web::Data<Database>,
    product: web::Json<ProductRequest>,
) -> HttpResponse {
    if let Err(validation_errors) = product.validate() {
        return handle_validation_errors(&validation_errors);
    }

    match product_service::create_product(&data, product.into_inner()).await {
        Ok(response) => HttpResponse::Created().json(response),
        Err(err) => HttpResponse::build(err.status_code).json(err.response),
    }
}
