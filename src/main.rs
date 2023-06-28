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
    let title = data.title.clone();
    let text = data.text.clone();

    let bg = query.bg.clone().unwrap_or("999999".to_string());
    let fg = query.fg.clone().unwrap_or("000000".to_string());
    let symbol = query.symbol.clone().unwrap_or("question_mark".to_string());

    let filename = format!("icons/outlined/{symbol}.svg");
    let icon = std::fs::read_to_string(filename).unwrap();
    let icon = icon.replace("xmlns", format!("fill=\"#{fg}\" xmlns").as_str());

    let height = 72;
    let width = 500;

    let res = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">
<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" width=\"{width}\" height=\"{height}\">
</svg>");

    HttpResponse::Ok().content_type("image/svg+xml").body(res)
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
