use actix_web::body::MessageBody;
use actix_web::{post, web, App, HttpServer, Responder, get};
use serde::Deserialize;
use t_logging as myotel; // Import your OpenTelemetry lib
use myotel::tracing::{info, span, Level};
use myotel::opentelemetry::KeyValue;
use myotel::opentelemetry::global;
use myotel::opentelemetry::metrics::Counter;
// use std::sync::LazyLock;
use serde_json::Value;
use std::sync::{Arc, Mutex, LazyLock};


static COUNTER: LazyLock<Arc<Mutex<Counter<u64>>>> = LazyLock::new(|| {
    let meter = global::meter("mylibraryname");
    Arc::new(Mutex::new(meter.u64_counter("total_calls").build()))
});

static ERROR_COUNTER: LazyLock<Arc<Mutex<Counter<u64>>>> = LazyLock::new(|| {
    let meter = global::meter("mylibraryname");
    Arc::new(Mutex::new(meter.u64_counter("total_errors").build()))
});


static METER_PROVIDER: LazyLock<Arc<Mutex<myotel::opentelemetry_sdk::metrics::SdkMeterProvider>>> = LazyLock::new(|| {
    let meter_provider = myotel::metrics::provider::init_meter_provider();
    Arc::new(Mutex::new(meter_provider))
});


#[derive(Deserialize)]
struct InputData {
    message: String,
}

#[get("/increase_count")]
async fn increase() -> impl Responder {
    let counter = COUNTER.lock().unwrap();
    counter.add(1, &[]);
    METER_PROVIDER.lock().unwrap().force_flush().unwrap();
    format!("{:?}", counter)
}

#[get("/force_error")]
async fn force_error() -> impl Responder {
    let counter = ERROR_COUNTER.lock().unwrap();
    counter.add(1, &[]);
    METER_PROVIDER.lock().unwrap().force_flush().unwrap();
    format!("{:?}", counter)
}

#[post("/submit")]
async fn submit(data: web::Json<Value>) -> impl Responder {
    println!("Error has been submitted");
    println!("{:?}", data);
    format!("Hello, World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    {
        let _placeholder = METER_PROVIDER.lock().unwrap();
    }

    // myotel::t_tracing::init_tracer_provider().expect("Failed to initialize OTLP HTTP tracer");
    let _meter_provider = myotel::metrics::provider::init_meter_provider();
    HttpServer::new(|| {
        App::new()
            .service(increase)
            .service(submit)
            .service(force_error)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
