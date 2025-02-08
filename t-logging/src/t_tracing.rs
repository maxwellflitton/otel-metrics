use opentelemetry::{
    global,
    trace::{TraceContextExt, TraceError, Tracer},
    KeyValue,
};
use opentelemetry_otlp::{WithExportConfig, WithHttpConfig};
use opentelemetry_sdk::trace::TracerProvider as SdkTracerProvider;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::runtime::Tokio;
use opentelemetry::trace::TracerProvider;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{Registry, prelude::*};


// pub fn init_tracer_provider() -> Result<SdkTracer, TraceError> {
pub fn init_tracer_provider() -> Result<(), TraceError> {
    let client = reqwest::Client::new();
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .with_endpoint("http://localhost:4318/v1/traces")
        .with_http_client(client)
        .build()?;
    let resource = Resource::new(vec![
        KeyValue::new("container.id", "abc123"),
        KeyValue::new("service.name", "my-rust-service"),
    ]);

    let tracer = SdkTracerProvider::builder()
        .with_batch_exporter(exporter, Tokio)
        .with_resource(resource)
        .build().tracer("trace_demo");

    let otel_layer = OpenTelemetryLayer::new(tracer);
    let subscriber = Registry::default().with(otel_layer);
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");
    Ok(())
}
