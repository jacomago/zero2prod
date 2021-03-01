use zero2prod::telemetry::{get_subscriber, init_subscriber};

use zero2prod::configuration::get_configuration;

use zero2prod::startup::Application;
//Get configuration and call the startup
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2rpod".into(), "info".into());
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
