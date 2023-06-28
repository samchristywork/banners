use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;


#[get("/banner/")]
async fn banner() -> impl Responder {
    HttpResponse::Ok().content_type("image/svg+xml").body("")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(banner)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
