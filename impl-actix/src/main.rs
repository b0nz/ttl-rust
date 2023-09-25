use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

struct AppState {
    counter: Mutex<i32>,
    todos: Mutex<Vec<String>>,
}

#[derive(Deserialize)]
struct Todo {
    name: String,
}

#[derive(Serialize)]
struct TodoResponse {
    data: Vec<String>,
}

#[get("/")]
async fn welcome() -> impl Responder {
    HttpResponse::Ok().body("Hi, Welcome Home!")
}

#[get("/say-hello/{name}")]
async fn say_hello(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    HttpResponse::Ok().body(format!("Hello {}!", name))
}

#[get("/counter")]
async fn counter(state: web::Data<AppState>) -> impl Responder {
    let mut counter = state.counter.lock().unwrap();
    *counter += 1;
    HttpResponse::Ok().body(format!("Counter: {counter}"))
}

#[get("/todos")]
async fn get_todos(state: web::Data<AppState>) -> impl Responder {
    let todos = state.todos.lock().unwrap();
    let obj = TodoResponse { data: todos.clone() };
    HttpResponse::Ok().json(obj)
}

#[post("/todo")]
async fn add_todo(req_body: web::Json<Todo>, state: web::Data<AppState>) -> impl Responder {
    let mut todos = state.todos.lock().unwrap();
    todos.push(req_body.name.clone());
    HttpResponse::Ok().body(format!("Todo {} added successfully!", req_body.name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        counter: Mutex::new(0),
        todos: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(welcome)
            .service(counter)
            .service(say_hello)
            .service(get_todos)
            .service(add_todo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}