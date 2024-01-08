use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

use crate::api::api_errors::ApiError;
use crate::models::task::{Task, TaskMsg};

#[get("/tasks")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let tasks = Task::find_all()?;
    Ok(HttpResponse::Ok().json(tasks))
}

#[get("/tasks/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, ApiError> {
    let task = Task::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(task))
}

#[post("/tasks")]
async fn create(task: web::Json<TaskMsg>) -> Result<HttpResponse, ApiError> {
    let task = Task::create(task.into_inner())?;
    Ok(HttpResponse::Ok().json(task))
}

#[put("/tasks/{id}")]
async fn update(id: web::Path<i32>, task: web::Json<TaskMsg>) -> Result<HttpResponse, ApiError> {
    let task = Task::update(id.into_inner(), task.into_inner())?;
    Ok(HttpResponse::Ok().json(task))
}

#[delete("/tasks/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, ApiError> {
    let num_deleted = Task::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": num_deleted })))
}
