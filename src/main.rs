use newsletter_api::configuration::get_configuration;
use newsletter_api::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing::dispatcher::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // redirects all `log`'s events to our subscriber
    LogTracer::init().expect("Failed to set logger.");
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("newsletter_api".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    set_global_default(subscriber.into()).expect("Failed to set subscriber.");

    let configuration = get_configuration().expect("Failed to read configuration file.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    println!("Running on {}", address);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
