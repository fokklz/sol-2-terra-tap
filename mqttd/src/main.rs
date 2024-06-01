use rumqttd::{ Broker, Config };
use tracing::span;
use tracing_subscriber::{ EnvFilter, FmtSubscriber };

#[tokio::main]
async fn main() {
    // Use the RUST_LOG environment variable to set the log level
    let filter = EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()));
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(filter)
        .with_level(true)
        .with_target(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Create a span for the broker and enter it
    let broker_span = span!(tracing::Level::INFO, "mqtt-server");
    let _ = broker_span.enter();

    // Configure the broker using the rumqttd configuration file
    // TODO: this should be handled differently to support for various configuration sources
    //      and to allow for more flexible configuration (not running from source)
    let raw_mqtt_config = config::Config
        ::builder()
        .add_source(config::File::with_name("mqttd/rumqttd.toml"))
        .build()
        .unwrap();
    let mqtt_config: Config = raw_mqtt_config.try_deserialize().unwrap();

    // Create the final instance of the broker and start it
    let mut broker = Broker::new(mqtt_config);
    tokio
        ::spawn(async move {
            broker.start().unwrap();
        }).await
        .unwrap();
}
