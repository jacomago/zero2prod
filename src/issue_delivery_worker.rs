use crate::{domain::SubscriberEmail, email_client::EmailClient};
use chrono::{Utc, Duration};
use sqlx::{PgPool, Postgres, Transaction};
use tracing::{field::display, Span};
use uuid::Uuid;

#[tracing::instrument( 
    skip_all,
    fields( 
        newsletter_issue_id=tracing::field::Empty,
        subscriber_email=tracing::field::Empty
    ),
    err
)]
async fn try_execute_task(pool: &PgPool, email_client: &EmailClient) -> Result<(), anyhow::Error> {
    if let Some((transaction, issue_id, email)) = dequeue_task(pool, 2).await? {
        Span::current()
            .record("newsletter_issue_id", &display(issue_id))
            .record("subscriber_email", &display(&email));
        match SubscriberEmail::parse(email.clone()) {
            Ok(email) => {
                let issue = get_issue(pool, issue_id).await?;
                if let Err(e) = email_client
                    .send_email(
                        &email,
                        &issue.title,
                        &issue.html_content,
                        &issue.text_content,
                    )
                    .await
                {
                    tracing::error!(
                        error.cause_chain = ?e,
                        error.message = %e,
                        "Failed to deliver issue to a confirmed subscriber. \
                         Skipping.",
                    );
                    retry_later_task(pool, issue_id, &email, 5).await?;
                }
            }
            Err(e) => {
                tracing::error!(
                error.cause_chain = ?e, error.message = %e,
                "Skipping a confirmed subscriber. \
                 Their stored contact details are invalid", );
            }
        }

        delete_task(transaction, issue_id, &email).await?;
    }
    Ok(())
}

type PgTransaction = Transaction<'static, Postgres>;

#[tracing::instrument(skip_all)]
async fn dequeue_task(
    pool: &PgPool,
    max_retries: i16,
) -> Result<Option<(PgTransaction, Uuid, String)>, anyhow::Error> {
    let mut transaction = pool.begin().await?;
    let r = sqlx::query!(
        r#"
            SELECT newsletter_issue_id, subscriber_email
              FROM issue_delivery_queue
             WHERE n_retries     < $1
               AND execute_after > CURRENT_TIMESTAMP
            FOR UPDATE
            SKIP LOCKED
            LIMIT 1
        "#,
        max_retries
    )
    .fetch_optional(&mut transaction)
    .await?;

    if let Some(r) = r {
        Ok(Some((
            transaction,
            r.newsletter_issue_id,
            r.subscriber_email,
        )))
    } else {
        Ok(None)
    }
}

async fn retry_later_task(
    pool: &PgPool,
    issue_id: Uuid,
    email: &SubscriberEmail,
    seconds: i64
) -> Result<(), anyhow::Error> {
    let execute_after = Utc::now() + Duration::seconds(seconds);
    sqlx::query!(
        r#"
            UPDATE issue_delivery_queue
            SET n_retries = (SELECT n_retries
                               FROM issue_delivery_queue
                              WHERE newsletter_issue_id = $1
                                AND subscriber_email = $2 ) + 1,
                execute_after = $3
            WHERE newsletter_issue_id = $1
              AND subscriber_email = $2 
        "#,
        issue_id,
        email.as_ref(),
        execute_after
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[tracing::instrument(skip_all)]
async fn delete_task(
    mut transaction: PgTransaction,
    issue_id: Uuid,
    email: &str,
) -> Result<(), anyhow::Error> {
    sqlx::query!(
        r#"
        DELETE FROM issue_delivery_queue
        WHERE
            newsletter_issue_id = $1 AND
            subscriber_email = $2 
        "#,
        issue_id,
        email
    )
    .execute(&mut transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

struct NewsletterIssue {
    title: String,
    text_content: String,
    html_content: String,
}

#[tracing::instrument(skip_all)]
async fn get_issue(pool: &PgPool, issue_id: Uuid) -> Result<NewsletterIssue, anyhow::Error> {
    let issue = sqlx::query_as!(
        NewsletterIssue,
        r#"
        SELECT title, text_content, html_content
        FROM newsletter_issues
        WHERE
            newsletter_issue_id = $1 
        "#,
        issue_id
    )
    .fetch_one(pool)
    .await?;
    Ok(issue)
}
