use actix_files as fs;
use actix_web::{get, web, App, HttpServer, Responder };

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(fs::Files::new("/static", "./static")))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
