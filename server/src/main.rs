use actix_web::{post, web, App, HttpServer, Responder, get};
use serde_json::Value;


#[post("/submit")]
async fn submit(data: web::Json<Value>) -> impl Responder {
    println!("Post Error has been submitted");
    println!("{:?}", data);
    format!("Hello, World!")
}

#[get("/submit")]
async fn submit_get() -> impl Responder {
    println!("Get Error has been submitted");
    format!("Hello, World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(submit)
            .service(submit_get)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
