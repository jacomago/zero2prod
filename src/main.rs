use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

use zero2prod::telemetry::{get_subscriber, init_subscriber};
use zero2prod::{email_client::EmailClient, startup::run};

use zero2prod::configuration::get_configuration;
//Get configuration and call the startup
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2rpod".into(), "info".into());
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let server = build(configuration).await?;
    server.await?;
    Ok(())
}
