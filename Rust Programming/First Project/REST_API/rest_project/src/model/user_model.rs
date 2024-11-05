use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::common_fields::CommonFields;


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub _id: ObjectId,
    pub user_id: String,
    pub name: String,
    pub email: String,
    pub common_fields: CommonFields,
}