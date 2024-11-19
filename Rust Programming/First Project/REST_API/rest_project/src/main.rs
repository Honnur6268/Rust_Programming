mod constants;
mod controller;
mod db;
mod middleware;
mod model;
mod request;
mod response;
mod service;
mod utils;

use std::env;

use actix_web::{
    get,
    middleware::{from_fn, Logger},
    web::{scope, Data},
    App, HttpResponse, HttpServer, Responder,
};
use controller::{
    product_controller::add_product,
    user_controller::{
        add_user, authenticate, delete_user_by_id, get_all_users, get_user_by_id, update_user,
    },
};
use db::database::Database;
use middleware::auth_middleware::jwt_auth_filter;

// Install: cargo install cargo-watch. Then, run command in terminal -> cargo watch -c -w src -x run (to watch the changes in the src folder and run the server automatically)

#[get("/health-check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let mongo_uri = env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

    // let mongo_client = Client::with_uri_str(MONGODB_URI.as_str())
    //     .await
    //     .expect("falied to connect to mongodb");

    // let database = Database::new(&mongo_client).await;
    dotenv::dotenv().ok();
    let db_init = Database::init_db().await;
    let db_data = Data::new(db_init);

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let listen_port = match env::var("SERVER.PORT") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading SERVER.PORT"),
    };

    log::info!("Server started at http://localhost:{}", listen_port);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .wrap(Logger::default())
            .service(health_check)
            .wrap(from_fn(jwt_auth_filter))
            .service(
                scope("/api/users")
                    .service(add_user)
                    .service(authenticate)
                    .service(get_all_users)
                    .service(delete_user_by_id)
                    .service(get_user_by_id)
                    .service(update_user),
            )
            .service(scope("/api/products").service(add_product))
    })
    .bind(("0.0.0.0", listen_port.parse().unwrap()))?
    .run()
    .await
}
