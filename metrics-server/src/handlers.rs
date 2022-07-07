use std::convert::Infallible;

use warp::{self, http::StatusCode};

use crate::db::MySqlDb;
use crate::models::Metric;

pub async fn add_metric(
    new_metric: Metric,
    db: MySqlDb,
) -> Result<impl warp::Reply, Infallible> {

    db.add_metric(new_metric).unwrap();

    Ok(StatusCode::CREATED)
}