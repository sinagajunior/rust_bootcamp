use actix_web::{get, App, HttpResponse, HttpServer, Responder};





#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server...at http://localhost:8080");
    HttpServer::new(|| App::new().service(hello))
        .bind("localhost:8080")?
        .run()
        .await
}
