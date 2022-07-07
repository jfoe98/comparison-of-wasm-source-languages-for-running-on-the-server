use reqwest;
use url::{Url, ParseError};
use super::models::{Metric};
use anyhow::Result;

pub struct MetricClient {
    base_uri: String,
}

impl MetricClient {
    pub fn new(base_uri: String) -> MetricClient {
        MetricClient {base_uri}
    }

    pub async fn post_metric(&self, metric: &Metric) -> Result<()> {
        let url = match self.build_metrics_url() {
            Ok(url) => url,
            Err(_) => anyhow::bail!("Error while building metrics uri"),
        };

        let response = match self.perform_http_post_request(url.as_str(), metric).await {
            Ok(resp) => resp,
            Err(_) => anyhow::bail!("Error while performing http request"),
        };

        match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            reqwest::StatusCode::CREATED => Ok(()),
            _ => anyhow::bail!("Error while fetching resource. Status was not OK."),
        }
    }

    fn build_metrics_url(&self) -> Result<Url, ParseError> {    
        let base = Url::parse(&self.base_uri)?;
        let joined = base.join("/metrics")?;
    
        Ok(joined)
    }

    async fn perform_http_post_request(&self, uri: &str, metric: &Metric) -> Result<reqwest::Response, reqwest::Error> {
        let client = reqwest::Client::new();

        client.post(uri)
            .json(&metric)
            .send()
            .await
    }
}