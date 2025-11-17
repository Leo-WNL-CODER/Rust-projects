use axum::{ extract::{Path, State,}, http::StatusCode, response::IntoResponse};

use crate::todo_fn::todo_struct::{ Todos};

pub async fn mark(State(todo_list):State<Todos>,
Path(id):Path<u32>
)
->impl IntoResponse
{
    let mut v=todo_list.todo.lock().unwrap();

    if let Some(task) = v.iter_mut().find(|t|t.id==id) {
        task.completed=true;
        (StatusCode::OK, "Ok").into_response()
    } else {
        (StatusCode::NOT_FOUND, "Task not found").into_response()
    }
    
}
