use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::common_fields::CommonFields;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Product {
    pub _id: ObjectId,
    pub product_id: String,
    pub product_name: String,
    pub product_sku: String,
    pub price: f64,
    pub common_fields: CommonFields,
}