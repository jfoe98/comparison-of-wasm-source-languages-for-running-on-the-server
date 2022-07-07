mod client;
pub mod models;

use client::MetricClient;
use models::Metric;
use super::Config;
use anyhow::Result;

pub async fn write_metric(config: &Config, metric: &Metric) -> Result<()>{
    let metric_client = MetricClient::new(config.metrics_base_uri.clone());
    metric_client.post_metric(metric).await?;

    Ok(())
}

pub fn build_context(workload_name: &str, opt_level: &str, size: &str) -> String {
    return format!("{}_{}_{}", workload_name, opt_level, size);
}