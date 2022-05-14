use actix_web::http::header::ContentType;
use actix_web::HttpResponse;

use crate::routes::{e500, see_other, TEMPLATES};
use crate::session_state::TypedSession;

pub async fn change_password_form(session: TypedSession) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(e500)?.is_none() {
        return Ok(see_other("/login"));
    };

    let context = tera::Context::new();
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(
        TEMPLATES
            .render("admin/password/change_password.html", &context)
            .unwrap(),
    ))
}
