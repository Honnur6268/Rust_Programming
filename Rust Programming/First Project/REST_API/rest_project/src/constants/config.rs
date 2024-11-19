use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref MONGODB_URI: String = set_mongo_uri();
    pub static ref MONGO_DB_NAME: String = set_mongo_db_name();
    pub static ref SECRET_KEY: String = set_secret_key();

    #[derive(Debug)]
    pub static ref EXCLUDE_PATHS: Vec<String> = set_excluded_paths();
}

fn set_mongo_uri() -> String {
    dotenv::dotenv().ok();
    env::var("MONGODB_URI").expect("MongoDb_URI not found in env file")
}

fn set_mongo_db_name() -> String {
    dotenv::dotenv().ok();
    env::var("MONGODB_DATABASE_NAME").expect("MONGODB_DATABASE_NAME not found in env file")
}

fn set_secret_key() -> String {
    dotenv::dotenv().ok();
    env::var("SECRET_KEY").expect("SECRET_KEY not found in env file")
}

fn set_excluded_paths() -> Vec<String> {
    dotenv::dotenv().ok();
    env::var("EXCLUDE_PATHS")
        .expect("EXCLUDE_PATHS not found in env file")
        .split(',')
        .map(|s| s.trim().to_string())
        .collect()
}
