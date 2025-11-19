use axum::{Json, extract::State};

use crate::{db::db_conn::db_client, todo_fn::todo_struct::{Task, Todos}};

pub async fn r_todo(State(todo_list):State<Todos>,)->Json<Vec<Task>>{
    let tl=todo_list.todo.lock().unwrap();
   Json(tl.clone())
}
pub async fn r_todo_v2()->Json<Vec<Task>>{
    let pool=db_client().await;
    
    let q=sqlx::query!(r#"
    Select * from todos
    "#).map(|t|{Task{
        id:t.id as u32,
        description:t.description.unwrap(),
        completed:t.completed.unwrap()
    }}).fetch_all(&pool).await.expect("failed to read");
    Json(q)
}