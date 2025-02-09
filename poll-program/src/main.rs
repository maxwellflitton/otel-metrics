use reqwest;
use serde::ser;
use serde_json::Value;
use std::error::Error;
use std::thread;
use std::time::Duration;

const PROMETHEUS_URL: &str = "http://localhost:9090/api/v1/query";
const QUERY: &str = "errors_total{exported_job=\"my-rust-metric-service\", job=\"otel-collector\"}";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    loop {
        match fetch_prometheus_metric().await {
            Ok(metrics) => {
                if !metrics.is_empty() {
                    for metric in metrics {
                        if let Some(label1) = metric.get("metric").and_then(|m| m.get("label1")) {
                            println!("Metric detected with label1: {}", label1);
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error fetching metrics: {}", e),
        }
        thread::sleep(Duration::from_secs(10));
    }
}

async fn fetch_prometheus_metric() -> Result<Vec<Value>, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let res = client
        .get(PROMETHEUS_URL)
        .query(&[("query", QUERY)])
        .send()
        .await?;
    
    let json: Value = serde_json::from_str(&res.text().await?)?;
    println!("{:?}", json);
    if let Some(data) = json.get("data").and_then(|d| d.get("result")) {
        return Ok(data.as_array().cloned().unwrap_or_default());
    }
    
    Ok(vec![])
}
