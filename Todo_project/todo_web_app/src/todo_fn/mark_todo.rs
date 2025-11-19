use axum::{ extract::{Path, State,}, http::StatusCode, response::IntoResponse};

use crate::{db::db_conn::db_client, todo_fn::todo_struct::Todos};

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

pub async fn mark_v2(Path(id):Path<i32>)->impl IntoResponse{
    let pool=db_client().await;
    let q=sqlx::query!(r#"
    Update todos set completed=true where id=($1)
    "#,id).
    execute(&pool).await;
    // println!("{:?}",q);

    if let Ok(_) = q {
        ("Marked").into_response()        
    }else{
        ("Error").into_response()
    }
}
