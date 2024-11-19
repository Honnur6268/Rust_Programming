use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginRequest {

    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    
    #[validate(length(min = 8, message = "Password must be at least 8 character."))]
    pub password: String,
}