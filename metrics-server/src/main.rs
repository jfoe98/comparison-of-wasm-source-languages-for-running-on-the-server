use crate::db::MySqlDb;

mod db;
mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    println!("Starting metrics server ...");

    let db = MySqlDb::new(String::from("root"), String::from("Passw0rd"), String::from("mysql"));
    let metrics_routes = routes::metric_routes(db);

    warp::serve(metrics_routes)
        .run(([0, 0, 0, 0], 3000))
        .await;
}