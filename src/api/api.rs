use actix_web::web;
use actix_web::{web::{
    Data,
    Json,
}, get, post, put, delete, HttpResponse};
use crate::{models::todo::Todo, repository::database::Database};

#[get("/todos")]
pub async fn get_todos(db: Data<Database>) -> HttpResponse {
    let todos = db.get_todos();
    HttpResponse::Ok().json(todos)
}

#[get("/todos/{id}")]
pub async fn get_todo(db: Data<Database>, id: web::Path<String>) -> HttpResponse {
    let todo = db.get_todo(&id);

    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

#[post("/todos")]
pub async fn create_todo(db: Data<Database>, new_todo: Json<Todo>) -> HttpResponse {
    let todo = db.create_todo(new_todo.into_inner());
    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/todos/{id}")]
pub async fn update_todo(db: Data<Database>, id: web::Path<String>, updated_todo: web::Json<Todo>) -> HttpResponse {
    let todo = db.update_todo(&id, updated_todo.into_inner());

    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

#[delete("/todos/{id}")]
pub async fn delete_todo(db: Data<Database>, id: web::Path<String>) -> HttpResponse {
    let todo = db.delete_todo(&id);

    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("api")
            .service(get_todos)
            .service(get_todo)
            .service(create_todo)
            .service(update_todo)
            .service(delete_todo)
    );
}