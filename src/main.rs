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

fn darken(hex: &str) -> String {
    let red_component = hex_color(hex).unwrap().1.red;
    let green_component = hex_color(hex).unwrap().1.green;
    let blue_component = hex_color(hex).unwrap().1.blue;

    let amount = 0.3;

    let red = red_component as f32 * amount;
    let green = green_component as f32 * amount;
    let blue = blue_component as f32 * amount;

    let red = red as u8;
    let green = green as u8;
    let blue = blue as u8;

    format!("{:02x}{:02x}{:02x}", red, green, blue)
}

#[get("/banner/{title}/{text}")]
async fn banner(data: web::Path<BannerPath>, query: web::Query<BannerQuery>) -> impl Responder {
    let title = data.title.clone();
    let text = data.text.clone();

    let bg = query.bg.clone().unwrap_or("999999".to_string());
    let fg = query.fg.clone().unwrap_or(darken(bg.as_str()));
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

#[get("/list_icons")]
async fn list_icons() -> impl Responder {
    let mut res = String::from("{\"icons\": [");
    let mut first = true;
    for entry in std::fs::read_dir("icons/outlined").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let filename = path.file_stem().unwrap().to_str().unwrap();
        if first {
            first = false;
        } else {
            res.push_str(",");
        }
        res.push_str(format!("\"{}\"", filename).as_str());
    }
    res.push_str("]}");

    HttpResponse::Ok().content_type("application/json").body(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(banner)
            .service(list_icons)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
