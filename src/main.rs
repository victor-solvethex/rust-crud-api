mod data;
mod db;
mod handlers;
mod models;
mod routes;

use db::DB;
use routes::router;

pub type Result<T> = std::result::Result<T, warp::Rejection>;

#[tokio::main]
async fn main() {
    let db: DB = DB::connect().await.expect("Error creating the db");
    let routes = router(db);
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
