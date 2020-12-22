use dotenv::from_filename;
use sqlx::postgres::PgPoolOptions;

use actix_files::Files;
use actix_web::{web, App, HttpServer};

mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let env_file =
    if std::env::var("ENVIRONMENT").unwrap_or("development".to_string()) == "production" {
      ".env"
    } else {
      ".local.env"
    };

  let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&dotenv::var("DATABASE_URL").unwrap())
    .await
    .unwrap();

  from_filename(env_file).ok();
  env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

  let addr = format!("0.0.0.0:{}", std::env::var("PORT").unwrap());
  HttpServer::new(|| {
    App::new()
      .service(web::resource("/").to(controllers::index))
      .service(Files::new("/static", "./client").prefer_utf8(true))
  })
  .bind(&addr)
  .unwrap()
  .run()
  .await
}
