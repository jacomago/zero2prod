use actix_web::{http::header::ContentType, HttpResponse};
use tera::Context;

use crate::routes::TEMPLATES;

pub async fn newsletter_form() -> HttpResponse {
    HttpResponse::Ok().content_type(ContentType::html()).body(
        TEMPLATES
            .render("admin/newsletters.html", &Context::new())
            .unwrap(),
    )
}
