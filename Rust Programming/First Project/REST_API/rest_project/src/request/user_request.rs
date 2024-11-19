use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserRequest {
    #[validate(length(min = 3, message = "Name must be at least 3 characters"))]
    pub name: String,

    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(min = 10, message = "Invalid Mobile Number"))]
    pub mobile_number: String,

    #[validate(length(min = 8, message = "Password must be at least 8 character."))]
    pub password: String,
}
