use std::vec;

use accept_language::intersection;
use log::{error, info, warn};
use mime::TEXT_HTML_UTF_8;

use actix_files::NamedFile;
use actix_web::{http::HeaderValue, HttpRequest, Responder};

const LANGUAGE_EN: &str = "en";
const LANGUAGE_TR: &str = "tr";

fn get_language(header_value_option: Option<&HeaderValue>) -> &str {
  match header_value_option {
    Some(header_value) => match header_value.to_str() {
      Ok(value) => {
        let languages = intersection(value, vec![LANGUAGE_EN, LANGUAGE_TR]);
        if languages.len() > 0 {
          if languages[0] == LANGUAGE_TR {
            LANGUAGE_TR
          } else {
            LANGUAGE_EN
          }
        } else {
          LANGUAGE_EN
        }
      }
      Err(e) => {
        error!("Error while casting header value to str: {}", e);
        LANGUAGE_EN
      }
    },
    None => {
      warn!("No accept language header");
      LANGUAGE_EN
    }
  }
}

pub async fn index(req: HttpRequest) -> impl Responder {
  let language = get_language(req.headers().get("Accept-Language"));
  info!("language: {}", language);
  NamedFile::open("./client/home.html")
    .unwrap()
    .set_content_type(TEXT_HTML_UTF_8)
    .into_response(&req)
}
