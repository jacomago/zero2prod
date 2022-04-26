use actix_web::{cookie::Cookie, http::header::ContentType, HttpRequest, HttpResponse};
use tera::Context;

use crate::routes::TEMPLATES;

pub async fn login_form(request: HttpRequest) -> HttpResponse {
    let mut login_context = Context::new();
    let error_message = match request.cookie("_flash") {
        None => "".into(),
        Some(cookie) => cookie.value().to_string(),
    };
    login_context.insert("error_message", &error_message);
    let mut response = HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(TEMPLATES.render("login.html", &login_context).unwrap());
    response
        .add_removal_cookie(&Cookie::new("_flash", ""))
        .unwrap();
    response
}
