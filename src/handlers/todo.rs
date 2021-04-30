use crate::{
    db::DB,
    models::todo::{SearchQuery, TodoRequest, TodoResponse, TodoUpdateRequest},
};
use warp::{hyper::StatusCode, reject, reply::json, Rejection, Reply};

pub async fn create_todo_handler(body: TodoRequest, db: DB) -> Result<impl Reply, Rejection> {
    Ok(json(&TodoResponse::of(
        db.todo.create(body).await.map_err(|e| reject::custom(e))?,
    )))
}

pub async fn list_todos_handler(query: SearchQuery, db: DB) -> Result<impl Reply, Rejection> {
    let todos = db
        .todo
        .fetch(query.search)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(json::<Vec<_>>(
        &todos.into_iter().map(|t| TodoResponse::of(t)).collect(),
    ))
}

pub async fn update_todo_handler(
    id: i32,
    body: TodoUpdateRequest,
    db: DB,
) -> Result<impl Reply, Rejection> {
    Ok(json(&TodoResponse::of(
        db.todo
            .update(id, body)
            .await
            .map_err(|e| reject::custom(e))?,
    )))
}

pub async fn delete_todo_handler(id: i32, db: DB) -> Result<impl Reply, Rejection> {
    db.todo.delete(id).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}
