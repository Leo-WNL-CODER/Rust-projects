use axum::{ Json, extract::{State, rejection::{ JsonRejection}}, http::StatusCode, response::{IntoResponse}};

use crate::{db::db_conn::db_client, todo_fn::todo_struct::{PayloadRequest, Task, Todos}};



pub async fn add_todo(State(todo_list):State<Todos>,
payload:Result<Json<PayloadRequest>,JsonRejection>)
-> impl IntoResponse
{
   
    let mut v=todo_list.todo.lock().unwrap();

    
    match payload {
        Ok(p)=>{
            let mut id=todo_list.next_id.lock().unwrap();

            let task=Task{
                id:*id,
                description:p.description.to_string(),
                completed:false
            };
            v.push(task);
            *id+=1;
            (StatusCode::OK, "OK").into_response()

        },
        Err(_)=>{
            ("Erro Occured").into_response()
        }
    };

}

pub async fn add_todo_v2(payload:Result<Json<PayloadRequest>,JsonRejection>)->impl IntoResponse{
    let pool=db_client().await;
    match payload {
        Ok(p)=>{
            let _=sqlx::query!(r#"
            Insert into todos (description) values ($1)
            "#,p.description).execute   (&pool).await.expect("failed to insert");
            ("Added").into_response()
        },
        Err(_)=>{
            ("Error").into_response()
        }
    }
    
}