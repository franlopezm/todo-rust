pub mod tasks;

use actix_web::{web, Scope};

pub fn routes() -> Scope {
    web::scope("api").service(tasks::routes())
}
