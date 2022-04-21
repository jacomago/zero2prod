use actix_web::{http::header::ContentType, web, HttpResponse};
use tera::Context;

use crate::routes::TEMPLATES;

#[derive(serde::Deserialize)]
pub struct QueryParams {
    error: Option<String>,
}

pub async fn login_form(query: web::Query<QueryParams>) -> HttpResponse {
    let _error = query.0.error;
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(TEMPLATES.render("login.html", &Context::new()).unwrap())
}
