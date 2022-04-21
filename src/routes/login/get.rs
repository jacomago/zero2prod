use actix_web::{http::header::ContentType, web, HttpResponse};
use tera::Context;

use crate::routes::TEMPLATES;

#[derive(serde::Deserialize)]
pub struct QueryParams {
    error: Option<String>,
}

pub async fn login_form(query: web::Query<QueryParams>) -> HttpResponse {
    let mut login_context = Context::new();
    login_context.insert(
        "error_message",
        &match query.0.error {
            None => "".into(),
            Some(message) => message,
        },
    );
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(TEMPLATES.render("login.html", &login_context).unwrap())
}
