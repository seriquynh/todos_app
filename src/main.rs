use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

#[get("/healthz")]
async fn healthz() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[get("/todos")]
async fn todos_index() -> impl Responder {
    HttpResponse::Ok().body("todos list")
}

#[post("/todos")]
async fn todos_store() -> impl Responder {
    // TODO: add new task to data.todos
    HttpResponse::Ok().body("todos list")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(healthz)
            .service(todos_index)
            .service(todos_store)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
