use std::env;

use mongodb::{Client, Collection};

use crate::{
    constants::config::{MONGODB_URI, MONGO_DB_NAME},
    model::{product_model::Product, user_model::User},
};

pub struct Database {
    users: Collection<User>,
    products: Collection<Product>,
}

impl Database {
    pub async fn init_db() -> Self {
        let mongo_uri =
            env::var(MONGODB_URI.clone()).unwrap_or_else(|_| "mongodb://localhost:27017".into());
        let mongo_client = Client::with_uri_str(mongo_uri)
            .await
            .expect("falied to connect to mongodb");

        let db = mongo_client.database(&MONGO_DB_NAME);

        let users_collection: Collection<User> = db.collection("users");
        let products_collection: Collection<Product> = db.collection("products");

        Database {
            users: users_collection,
            products: products_collection,
        }
    }

    pub fn get_users_collection(&self) -> &Collection<User> {
        &self.users
    }

    pub fn get_products_collection(&self) -> &Collection<Product> {
        &self.products
    }
}
