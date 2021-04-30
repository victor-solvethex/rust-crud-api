use crate::{
    db::DB,
    handlers::{
        rejection::handle_rejection,
        todo::{create_todo_handler, delete_todo_handler, list_todos_handler, update_todo_handler},
    },
};
use std::convert::Infallible;
use warp::{Filter, Reply};

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub fn router(db: DB) -> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone {
    let todo = warp::path("todo");
    let todo_routes = todo
        .and(warp::get())
        .and(warp::query())
        .and(with_db(db.clone()))
        .and_then(list_todos_handler)
        .or(todo
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(create_todo_handler))
        .or(todo
            .and(warp::put())
            .and(warp::path::param())
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(update_todo_handler))
        .or(todo
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(delete_todo_handler));

    let routes = todo_routes
        .with(warp::cors().allow_any_origin())
        .recover(handle_rejection);
    routes
}
