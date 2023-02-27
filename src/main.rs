mod redis_client;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        // enable everything
        .with_max_level(tracing::Level::ERROR)
        // output to stdout
        .with_writer(std::io::stdout)
        // sets this to be the default, global collector for this application.
        .init();

    let redis_client = redis_client::RedisClient::new("127.0.0.1".to_string(), 6379);
    redis_client.listen().await;
}
