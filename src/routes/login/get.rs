use actix_web::{
    cookie::{time::Duration, Cookie},
    http::header::ContentType,
    HttpRequest, HttpResponse,
};
use tera::Context;

use crate::routes::TEMPLATES;

pub async fn login_form(request: HttpRequest) -> HttpResponse {
    let mut login_context = Context::new();
    let error_message = match request.cookie("_flash") {
        None => "".into(),
        Some(cookie) => cookie.value().to_string(),
    };
    login_context.insert("error_message", &error_message);
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .cookie(Cookie::build("_flash", "").max_age(Duration::ZERO).finish())
        .body(TEMPLATES.render("login.html", &login_context).unwrap())
}
