use std::sync::{Arc, Mutex};

use axum::{Json, Router, extract::State, routing::{delete, get, post, put}};

use crate::todo_fn::{create_todo::add_todo, delete_todo::del_todo, mark_todo::{ mark}, todo_struct::{Task, Todos}};
mod todo_fn;

use tower_http::cors::{CorsLayer, Any};

use dotenvy;



#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any);

    let todo_list=Todos{
        todo:Arc::new(Mutex::new(Vec::new())),
        next_id:Arc::new(Mutex::new(1))
        };
    
    let app:Router=Router::new()
    .route("/", get(r_todo))
    .route("/addTodo", post(add_todo))
    .route("/deleteTodo/{id}",delete(del_todo))
    .route("/markDone/{id}", put(mark))
    .with_state(todo_list)
    .layer(cors);
    
    
    let listner = tokio::net::TcpListener::bind("localhost:3000")
    .await.unwrap();


    axum::serve(listner,app)
    .await.unwrap();

}

async fn r_todo(State(todo_list):State<Todos>,)->Json<Vec<Task>>{
    let tl=todo_list.todo.lock().unwrap();
   Json(tl.clone())
}