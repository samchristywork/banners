use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use nom::{bytes::complete::take_while_m_n, combinator::map_res, sequence::tuple, IResult};
use serde::Deserialize;

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

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

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

    Ok((input, Color { red, green, blue }))
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
<rect fill=\"#{bg}\" x=\"0\" y=\"0\" width=\"{width}\" height=\"{height}\"/>
<rect fill=\"#{fg}\" x=\"87\" y=\"11\" width=\"10\" height=\"50\"/>
<g transform=\"scale(1)\">
  <text fill=\"#{fg}\" x=\"110\" y=\"40\" font-size=\"33px\" font-weight=\"bold\" font-family=\"sans serif\">{title}</text>
  <text fill=\"#{fg}\" x=\"110\" y=\"55\" font-size=\"13px\">{text}</text>
  <g transform=\"translate(5 0) scale(3)\">
  {icon}
  </g>
</g></svg>");

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
