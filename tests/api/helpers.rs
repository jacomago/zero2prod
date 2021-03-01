use sqlx::{Executor, Connection, PgConnection, PgPool};

use std::net::TcpListener;
use uuid::Uuid;

use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use zero2prod::{
    configuration::{get_configuration, DatabaseSettings},
    email_client::EmailClient,
};


// Ensure the 'tracing' stack is only initialised once using lazy_static
lazy_static::lazy_static! {
    static ref TRACING: () = {
        let subscriber = get_subscriber("test".into(), "debug".into());
        init_subscriber(subscriber);
    };
    }

    
pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database");

    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

pub async fn spawn_app() -> TestApp {
    // the first time initialize is invoked the code in 'TRACING' is executed.
    // All other invocations will instead skip execution.
    lazy_static::initialize(&TRACING);

    let host = "localhost";
    let listener = TcpListener::bind(format!("{}:0", host)).expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://{}:{}", host, port);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let connection_pool = configure_database(&configuration.database).await;

    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address.");
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
    );

    let server =
        run(listener, connection_pool.clone(), email_client).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}
