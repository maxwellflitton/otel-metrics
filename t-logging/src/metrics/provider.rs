// use opentelemetry::{global, KeyValue};
// // use opentelemetry_sdk::error::OTelSdkError;
// use opentelemetry_sdk::metrics::{PeriodicReader, SdkMeterProvider};
// use opentelemetry_sdk::Resource;
// use std::error::Error;
// use std::vec;
// use opentelemetry_otlp::WithExportConfig;
// use opentelemetry_otlp::WithHttpConfig;
// use opentelemetry_sdk::metrics::reader::MetricReader;


// pub fn init_meter_provider() -> opentelemetry_sdk::metrics::SdkMeterProvider {
    // let client = reqwest::Client::new();
    // let exporter = opentelemetry_otlp::MetricExporter::builder()
    //     .with_http()
    //     .with_endpoint("http://localhost:4318/v1/metrics")
    //     .with_http_client(client)
    //     .build().unwrap();
    // let resource = Resource::new(vec![
    //     KeyValue::new("container.id", "abc123"),
    //     KeyValue::new("service.name", "my-rust-service"),
    // ]);
//     let provider = SdkMeterProvider::builder()
//         // .with_exporter(exporter)
//         // .with_batch_exporter(exporter, Tokio)
//         .with_reader(exporter)
//         .with_resource(resource)
//         .build();
//     global::set_meter_provider(provider.clone());
//     provider
// }


use opentelemetry::global;
use opentelemetry::metrics::Meter;
use opentelemetry::Key;
use opentelemetry::KeyValue;
use opentelemetry_sdk::metrics::{
    Aggregation, Instrument, PeriodicReader, SdkMeterProvider, Stream, Temporality,
};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::runtime::Tokio;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_otlp::WithHttpConfig;

// pub fn init_meter_provider() -> Meter {
pub fn init_meter_provider() -> opentelemetry_sdk::metrics::SdkMeterProvider {
    // for example 1
    let my_view_rename_and_unit = |i: &Instrument| {
        if i.name == "my_histogram" {
            Some(
                Stream::new()
                    .name("my_histogram_renamed")
                    .unit("milliseconds"),
            )
        } else {
            None
        }
    };

    // for example 2
    let my_view_drop_attributes = |i: &Instrument| {
        if i.name == "my_counter" {
            Some(Stream::new().allowed_attribute_keys(vec![Key::from("mykey1")]))
        } else {
            None
        }
    };

    // for example 3
    let my_view_change_aggregation = |i: &Instrument| {
        if i.name == "my_second_histogram" {
            Some(
                Stream::new().aggregation(Aggregation::ExplicitBucketHistogram {
                    boundaries: vec![0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5],
                    record_min_max: false,
                }),
            )
        } else {
            None
        }
    };

    let client = reqwest::Client::new();
    let exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_http()
        .with_endpoint("http://localhost:4318/v1/metrics")
        // .with_endpoint("http://localhost:4318")
        .with_http_client(client)
        .with_temporality(Temporality::Delta)
        .build().unwrap();

    let reader = PeriodicReader::builder(exporter, Tokio).build();

    let resource = Resource::new(vec![
        KeyValue::new("container.id", "abc123"),
        KeyValue::new("service.name", "my-rust-metric-service"),
    ]);

    let provider = SdkMeterProvider::builder()
        .with_reader(reader)
        .with_resource(resource)
        .with_view(my_view_rename_and_unit)
        .with_view(my_view_drop_attributes)
        .with_view(my_view_change_aggregation)
        .build();
    global::set_meter_provider(provider.clone());
    // let meter = global::meter("mylibraryname");
    // meter
    provider
}