use sqlx::PgPool;
use std::net::TcpListener;

use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

use zero2prod::configuration::get_configuration;
//Get configuration and call the startup
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2rpod".into(), "info".into());
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect ot Postgres.");
    let address = format!("localhost:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
