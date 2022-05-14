use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use anyhow::Context;
use reqwest::header::LOCATION;
use sqlx::PgPool;
use uuid::Uuid;

use crate::routes::TEMPLATES;
use crate::session_state::TypedSession;

use super::e500;

pub async fn admin_dashboard(
    session: TypedSession,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let username = if let Some(user_id) = session.get_user_id().map_err(e500)? {
        get_username(user_id, &pool).await.map_err(e500)?
    } else {
        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/login"))
            .finish());
    };

    let mut admin_context = tera::Context::new();
    admin_context.insert("username", &username);
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(
        TEMPLATES
            .render("admin/dashboard.html", &admin_context)
            .unwrap(),
    ))
}

#[tracing::instrument(name = "Get username", skip(pool))]
async fn get_username(user_id: Uuid, pool: &PgPool) -> Result<String, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT username FROM users
        WHERE user_id = $1 "#,
        user_id,
    )
    .fetch_one(pool)
    .await
    .context("Failed to perform a query to retrieve a username.")?;
    Ok(row.username)
}
