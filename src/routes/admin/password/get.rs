use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use std::fmt::Write;
use actix_web_flash_messages::IncomingFlashMessages;

use crate::routes::TEMPLATES;

pub async fn change_password_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut error_message = String::new();
    for m in flash_messages.iter() {
        writeln!(error_message, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    let mut context = tera::Context::new();
    context.insert("error_message", &error_message);

    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(
        TEMPLATES
            .render("admin/password/change_password.html", &context)
            .unwrap(),
    ))
}
