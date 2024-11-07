use std::borrow::BorrowMut;

use crate::db::AppState;
use crate::tasks::dto::{TaskBody, TaskDone, TaskError};
use crate::tasks::models::Task;
use actix_web::http::header::ContentType;
use actix_web::{delete, get, post, put, web, HttpResponse, Scope};

#[post("/add")]
async fn create_task(req: web::Json<TaskBody>, data: web::Data<AppState>) -> HttpResponse {
    let task = Task::create(req.text.to_owned());

    let mut tasks = data.task.lock().unwrap();

    let response = serde_json::to_string(&task).unwrap();
    tasks.push(task);

    HttpResponse::Created()
        .content_type(ContentType::json())
        .body(response)
}

#[put("/{id}")]
async fn update_task(
    id: web::Path<String>,
    req: web::Json<TaskBody>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let task_id = id.into_inner();
    let mut tasks = data.task.lock().unwrap();

    let index = tasks.iter().position(|x| x.id == task_id);

    match index {
        Some(idx) => {
            let new_task = tasks[idx].borrow_mut();

            Task::update_text(new_task, req.text.to_owned());

            HttpResponse::Ok()
                .content_type(ContentType::json())
                .json(new_task)
        }
        None => HttpResponse::NotFound()
            .content_type(ContentType::json())
            .json(TaskError {
                error: String::from("not-found"),
                message: String::from("Task not found."),
            }),
    }
}

#[put("/{id}/done")]
async fn done_task(
    id: web::Path<String>,
    req: web::Json<TaskDone>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let task_id = id.into_inner();
    let mut tasks = data.task.lock().unwrap();

    let index = tasks.iter().position(|x| x.id == task_id);

    match index {
        Some(idx) => {
            let new_task = tasks[idx].borrow_mut();

            Task::completed(new_task, req.done);

            HttpResponse::Ok()
                .content_type(ContentType::json())
                .json(new_task)
        }
        None => HttpResponse::NotFound()
            .content_type(ContentType::json())
            .json(TaskError {
                error: String::from("not-found"),
                message: String::from("Task not found."),
            }),
    }
}

#[get("/{id}")]
async fn get_task(id: web::Path<String>, data: web::Data<AppState>) -> HttpResponse {
    let task_id = id.into_inner();
    let tasks = data.task.lock().unwrap();

    let task: Vec<_> = tasks.iter().filter(|x| x.id == task_id).collect();

    if !task.is_empty() {
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(task[0])
    } else {
        HttpResponse::NotFound()
            .content_type(ContentType::json())
            .json(TaskError {
                error: String::from("not-found"),
                message: String::from("Task not found."),
            })
    }
}

#[get("/list")]
async fn get_all_task(data: web::Data<AppState>) -> HttpResponse {
    let tasks = data.task.lock().unwrap();

    let response = serde_json::to_string(&(*tasks)).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response)
}

#[delete("/{id}")]
async fn delete_task(id: web::Path<String>, data: web::Data<AppState>) -> HttpResponse {
    let task_id = id.into_inner();
    let mut tasks = data.task.lock().unwrap();

    let index = tasks.iter().position(|x| x.id == task_id);

    match index {
        Some(idx) => {
            let deleted_task = tasks.remove(idx);

            HttpResponse::Ok()
                .content_type(ContentType::json())
                .json(deleted_task)
        }
        None => HttpResponse::NotFound()
            .content_type(ContentType::json())
            .json(TaskError {
                error: String::from("not-found"),
                message: String::from("Task not found."),
            }),
    }
}

pub fn routes() -> Scope {
    web::scope("tasks")
        .service(get_all_task)
        .service(get_task)
        .service(create_task)
        .service(update_task)
        .service(done_task)
        .service(delete_task)
}
