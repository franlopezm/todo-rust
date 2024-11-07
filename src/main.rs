mod controllers;
mod db;
mod env_config;
mod tasks;

use std::sync::Mutex;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = env_config::get();

    println!(
        "Server run in http://{}:{}",
        config.service_ip, config.service_port
    );

    let app_state = web::Data::new(db::AppState {
        task: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(controllers::routes())
    })
    .bind((config.service_ip, config.service_port))?
    .run()
    .await
}
