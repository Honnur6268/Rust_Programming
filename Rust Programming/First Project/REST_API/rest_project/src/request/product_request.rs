use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct ProductRequest {
    #[validate(length(min=1, message="Product id is required"))]
    pub product_name: String,

    #[validate(length(min=1, message="Product SKU is required"))]
    pub product_sku: String,
    
    pub price: f64
}