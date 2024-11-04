use actix_web::{get, web, App, HttpServer, Responder};
mod env_config;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = env_config::get();

    println!(
        "Server run in http://{}:{}",
        config.service_ip, config.service_port
    );

    HttpServer::new(move || App::new().service(greet))
        .bind((config.service_ip, config.service_port))?
        .run()
        .await
}
