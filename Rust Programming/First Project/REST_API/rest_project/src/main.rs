mod constants;
mod controller;
mod db;
mod model;
mod request;
mod response;
mod service;
mod utils;

use actix_web::{
    get,
    middleware::Logger,
    web::{scope, Data},
    App, HttpResponse, HttpServer, Responder,
};
use controller::{
    product_controller::add_product,
    user_controller::{add_user, delete_user_by_id, get_all_users, get_user_by_id, update_user},
};
use db::database::Database;

// Install: cargo install cargo-watch. Then, run command in terminal -> cargo watch -c -w src -x run (to watch the changes in the src folder and run the server automatically)

#[get("/health-check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Working fine!!")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let mongo_uri = env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

    // let mongo_client = Client::with_uri_str(MONGODB_URI.as_str())
    //     .await
    //     .expect("falied to connect to mongodb");

    // let database = Database::new(&mongo_client).await;
    let db_init = Database::init_db().await;
    let db_data = Data::new(db_init);

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    log::info!("Server started at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .wrap(Logger::default())
            .service(health_check)
            .service(
                scope("/api/users")
                    .service(add_user)
                    .service(get_all_users)
                    .service(delete_user_by_id)
                    .service(get_user_by_id)
                    .service(update_user),
            )
            .service(scope("/api/products").service(add_product))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
