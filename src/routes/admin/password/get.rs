use actix_web::http::header::ContentType;
use actix_web::HttpResponse;

use crate::routes::TEMPLATES;

pub async fn change_password_form() -> Result<HttpResponse, actix_web::Error> {
    let context = tera::Context::new();
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(
        TEMPLATES
            .render("admin/password/change_password.html", &context)
            .unwrap(),
    ))
}
