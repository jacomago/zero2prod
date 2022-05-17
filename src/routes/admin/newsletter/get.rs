use actix_web::{http::header::ContentType, HttpResponse};
use tera::Context;

use crate::{
    routes::{e500, see_other, TEMPLATES},
    session_state::TypedSession,
};

pub async fn newsletter_form(session: TypedSession) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(e500)?.is_none() {
        return Ok(see_other("/login"));
    };

    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(
        TEMPLATES
            .render("admin/newsletters.html", &Context::new())
            .unwrap(),
    ))
}
