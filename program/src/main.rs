use myotel::tracing::{info, span, Level};
use myotel::opentelemetry::KeyValue;
use myotel::opentelemetry::global;
use tokio::time::sleep;
use std::time::Duration;

use t_logging as myotel; // Import your OpenTelemetry lib

#[tokio::main]
async fn main() {
    // Initialize OpenTelemetry
    myotel::t_tracing::init_tracer_provider().expect("Failed to initialize OTLP HTTP tracer");
    let meter_provider = myotel::metrics::provider::init_meter_provider();

    let meter = global::meter("mylibraryname");

    // these basically get wiped everytime there is a new biuld so we need to make them static globcal variables
    let counter = meter.u64_counter("my_new_counter").build();
    println!("Recording metric: my_counter with value 10");
    counter.add(
        10,
        &[
            // KeyValue::new("mykey1", "myvalue1"),
            // KeyValue::new("mykey2", "myvalue2"),
            // KeyValue::new("instance", "otel-collector"),
            // KeyValue::new("job", "otel-collector"),
            // KeyValue::new("prometheus_metric", "true"),
        ],
    );
    counter.add(
        10,
        &[
            // KeyValue::new("mykey1", "myvalue1"),
            // KeyValue::new("mykey2", "myvalue2"),
            // KeyValue::new("instance", "otel-collector"),
            // KeyValue::new("job", "otel-collector"),
            // KeyValue::new("prometheus_metric", "true"),
        ],
    );

    // // Metrics are exported by default every 30 seconds when using stdout exporter,
    // // however shutting down the MeterProvider here instantly flushes
    // // the metrics, instead of waiting for the 30 sec interval.
    // // meter_provider.shutdown().unwrap();
    meter_provider.force_flush().unwrap();
    println!("Metrics flushed.");

    // Create a new trace span
    let root_span = span!(Level::INFO, "root_span", container_id = "abc123");
    let _guard = root_span.enter();

    info!("Starting tracing...");

    // Simulate async work
    async_task().await;

    info!("Tracing complete.");
    sleep(Duration::from_secs(10)).await;
}

// #[myotel::instrument(level = "info", fields(user_id = 42))]
async fn async_task() {
    let span = span!(Level::INFO, "async_task", task_id = "task_123");
    let _guard = span.enter();

    info!("Inside async task");
    sleep(Duration::from_secs(10)).await;
    info!("Async task completed");
}
