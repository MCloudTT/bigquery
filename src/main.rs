mod bigquery;
mod config;
mod redis_client;

use lazy_static::lazy_static;

use crate::config::Configuration;

// Load config
lazy_static! {
    static ref SETTINGS: Configuration = Configuration::load().unwrap();
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        // enable everything
        .with_max_level(tracing::Level::ERROR)
        // output to stdout
        .with_writer(std::io::stdout)
        // sets this to be the default, global collector for this application.
        .init();

    let redis_client =
        redis_client::RedisClient::new(SETTINGS.redis.host.clone(), SETTINGS.redis.port);
    redis_client.listen().await;
}
