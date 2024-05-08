use tracing::{dispatcher::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

// Compose multiple layers into a `tracing`'s subscriber.
//
// Using `impl Subscriber` as return type to avoid needing to spell out all the details of the
// subscriber return type.
pub fn get_subscriber<M>(name: String, env_filter: String, sink: M) -> impl Subscriber + Send + Sync
where
    M: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    // redirects all `log`'s events to our subscriber
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

// Register a subscriber as global default to process span data. Should only be called once.
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger.");
    set_global_default(subscriber.into()).expect("Failed to set subscriber.");
}
