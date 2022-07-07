use crate::MySqlDb;
use std::convert::Infallible;
use warp::{self, Filter};

use crate::handlers;
use crate::models::Metric;

pub fn metric_routes(
    db: MySqlDb,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    add_metric(db.clone())
}

/// POST /metric
fn add_metric(
    db: MySqlDb,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("metrics")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::add_metric)
}

fn with_db(db: MySqlDb) -> impl Filter<Extract = (MySqlDb,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_body() -> impl Filter<Extract = (Metric,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}