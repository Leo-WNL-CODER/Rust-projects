use std::{ sync::{Arc, Mutex}};
mod db;
use axum::{ Router,routing::{delete, get, post, put}};

use crate::{todo_fn::{create_todo::{add_todo, add_todo_v2}, 
delete_todo::{del_todo, del_todo_v2}, mark_todo::{mark, mark_v2}, 
read_todo::{r_todo, r_todo_v2}, todo_struct::{ Todos}}};


mod todo_fn;

use tower_http::cors::{CorsLayer, Any};




#[tokio::main]
async fn main() {
    
    let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any);
 
    let todo_list=Todos{
        todo:Arc::new(Mutex::new(Vec::new())),
        next_id:Arc::new(Mutex::new(1))
        };
    
    let app:Router=Router::new()
    .route("/v2", get(r_todo_v2))

    .route("/v2/addTodo", post(add_todo_v2))

    .route("/v2/deleteTodo/{id}",delete(del_todo_v2))

    .route("/v2/markDone/{id}", put(mark_v2))

    .route("/v1", get(r_todo))

    .route("/v1/addTodo", post(add_todo))

    .route("/v1/deleteTodo/{id}",delete(del_todo))

    .route("/v1/markDone/{id}", put(mark))

    .with_state(todo_list)

    .layer(cors);
    
    
    let listner = tokio::net::TcpListener::bind("localhost:3000")
    .await.unwrap();


    axum::serve(listner,app)
    .await.unwrap();

}

