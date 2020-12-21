use dotenv::from_filename;
use mime::TEXT_HTML_UTF_8;

use actix_files::{NamedFile, Files};
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn index(req: HttpRequest) -> impl Responder {
  NamedFile::open("./client/home.html")
    .unwrap()
    .set_content_type(TEXT_HTML_UTF_8)
    .into_response(&req)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let env_file = if std::env::var("ENVIRONMENT").unwrap_or("development".to_string()) == "production" {
    ".env"
  } else {
    ".local.env"
  };

  from_filename(env_file).ok();
  env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

  let addr = format!("0.0.0.0:{}", std::env::var("PORT").unwrap());
  HttpServer::new(|| {
    App::new()
      .service(web::resource("/").to(index))
      .service(Files::new("/static", "./client").prefer_utf8(true))
  })
  .bind(&addr)
  .unwrap()
  .run()
  .await
}
