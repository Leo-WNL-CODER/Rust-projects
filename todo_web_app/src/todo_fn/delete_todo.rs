
use axum::{ extract::{Path, State}, http::StatusCode, response::IntoResponse};

use crate::todo_fn::todo_struct::{ Todos};

pub async fn del_todo(
    State(todo_list):State<Todos>,
    Path(id):Path<u32>
)->impl IntoResponse
{
    println!("gdfg");
    let mut v=todo_list.todo.lock().unwrap();
    let original_len = v.len();
    v.retain(|t|{t.id!=id});  
    if v.len() < original_len {
        (StatusCode::OK, "Deleted").into_response()
    } else {
        (StatusCode::NOT_FOUND, "Task not found").into_response()
    }
}


