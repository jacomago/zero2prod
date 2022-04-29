use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::{IncomingFlashMessages, Level};
use std::fmt::Write;
use tera::Context;

use crate::routes::TEMPLATES;

pub async fn login_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut error_message = String::new();
    for m in flash_messages.iter().filter(|m| m.level() == Level::Error) {
        writeln!(error_message, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    let mut login_context = Context::new();
    login_context.insert("error_message", &error_message);

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(TEMPLATES.render("login.html", &login_context).unwrap())
}
