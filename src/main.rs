use dotenv::from_filename;
use log::info;

use actix_files::NamedFile;
use actix_web::{App, HttpRequest, HttpServer, Responder, web};

async fn index(req: HttpRequest) -> impl Responder {
  NamedFile::open("./client/home.html").unwrap().into_response(&req)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  from_filename(".env").ok();

  let addr = format!("0.0.0.0:{}", std::env::var("PORT").unwrap());
  let server = HttpServer::new(|| App::new().service(web::resource("/").to(index)))
    .bind(&addr)
    .unwrap();

  info!("Starting http server: {}", &addr);
  server.run().await
}
