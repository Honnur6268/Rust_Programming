use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommonFields {
    pub created_at: String,
    pub updated_at: String,
}