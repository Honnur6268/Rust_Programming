use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserRequest {
    #[validate(length(min = 3, message = "Name must be at least 3 characters"))]
    pub name: String,

    #[validate(email(message = "Invalid email address"))]
    pub email: String,
}