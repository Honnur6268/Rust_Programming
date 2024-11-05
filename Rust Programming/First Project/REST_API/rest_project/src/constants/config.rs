use std::env;
use once_cell::sync::Lazy;

pub static MONGODB_URI: Lazy<String> = Lazy::new(|| {
    env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string())
});

pub static MONGO_DB_NAME: Lazy<String> =
    Lazy::new(|| env::var("MONGODB_DATABASE_NAME").unwrap_or_else(|_| "RustDb".to_string()));
