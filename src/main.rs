use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BannerQuery {
    fg: Option<String>,
    bg: Option<String>,
    symbol: Option<String>,
}

#[derive(Deserialize)]
pub struct BannerPath {
    title: String,
    text: String,
}


#[get("/banner/{title}/{text}")]
async fn banner(data: web::Path<BannerPath>, query: web::Query<BannerQuery>) -> impl Responder {
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
