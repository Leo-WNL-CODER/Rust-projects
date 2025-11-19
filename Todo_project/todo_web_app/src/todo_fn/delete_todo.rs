
use axum::{ extract::{Path, State}, http::StatusCode, response::IntoResponse};

use crate::{db::db_conn::db_client, todo_fn::todo_struct::Todos};

pub async fn del_todo(
    State(todo_list):State<Todos>,
    Path(id):Path<u32>
)->impl IntoResponse
{
    let mut v=todo_list.todo.lock().unwrap();
    let original_len = v.len();
    v.retain(|t|{t.id!=id});  
    if v.len() < original_len {
        (StatusCode::OK, "Deleted").into_response()
    } else {
        (StatusCode::NOT_FOUND, "Task not found").into_response()
    }
}


pub async fn del_todo_v2(Path(id):Path<i32>)->impl IntoResponse{
    let pool=db_client().await;
    let q=sqlx::query!(r#"
    Delete from todos where id=($1)
    "#,id).
    fetch_one(&pool).await;

    if let Ok(_) = q {
        ("Updated").into_response()        
    }else{
        ("Error").into_response()
    }
}