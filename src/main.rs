use actix_web::{get, web, App, HttpServer, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
struct Measurement {
    temperature: f32,
}

#[derive(Deserialize, Serialize)]
struct Event {
    id: Option<i32>,
    timestamp: f64,
    kind: String,
    tags: Vec<String>,
}

#[get("/hw")]
async fn index() -> impl Responder {
    "Hello, World!"
}

//Sample JSON deserialize/serialize
// async fn capture_event(evt: web::Json<Event>) -> impl Responder {
//     let new_event = store_in_db(evt.timestamp, &evt.kind, &evt.tags);
//     format!("got event {}", new_event.id.unwrap())
// }

#[get("/faketemp")]
async fn current_temperature() -> impl Responder {
    web::Json(Measurement { temperature: 42.3 })
}

#[get("/hw/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(index)
        .service(hello)
        .service(current_temperature))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}