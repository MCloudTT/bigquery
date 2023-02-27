use futures_util::StreamExt as _;
use redis::{aio::Connection, Client, Msg};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

pub struct RedisClient {
    client: Client,
}

impl RedisClient {
    pub fn new(host: String, port: u16) -> Self {
        Self {
            client: redis::Client::open(format!("redis://{0}:{1}", host, port)).unwrap(),
        }
    }
    /// Get a connection to redis instance
    async fn get_connection(&self) -> Connection {
        loop {
            match self.client.get_async_connection().await {
                Ok(con) => return con,
                Err(e) => {
                    error!("Error connecting to redis: {0} (retrying in one second)", e);
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }
        }
    }
    /// Receive messages from redis
    pub async fn listen(&self) {
        loop {
            // Get a connection to redis instance
            let mut con = self.get_connection().await.into_pubsub();
            // Subscribe to redis pubsub channel "sync"
            if let Err(_) = con.subscribe("sync").await {
                error!("Error subscribing to redis channel");
                continue;
            }
            let mut pubsub_stream = con.on_message();

            loop {
                // Wait for a message from redis
                let msg = match pubsub_stream.next().await {
                    Some(msg) => msg,
                    None => {
                        error!("Error receiving message from redis");
                        break;
                    }
                };
                // Handle the message
                Self::handle_redis_message(msg);
            }
        }
    }
    /// Handle a message received from redis
    fn handle_redis_message(msg: Msg) {
        println!(
            "Received message from redis: {0}",
            msg.get_payload::<String>().unwrap()
        );
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct RedisMessage {
    sender_id: String,
    topic: String,
    payload: Vec<u8>,
    qos: u8,
    retain: bool,
}
