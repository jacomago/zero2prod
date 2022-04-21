use actix_web::{http::header::ContentType, HttpResponse};
use tera::Context;

use super::TEMPLATES;

pub async fn home() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(TEMPLATES.render("home.html", &Context::new()).unwrap())
}
