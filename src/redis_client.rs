use futures_util::StreamExt as _;
use redis::{aio::Connection, Client, Msg};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::bigquery;

pub struct RedisClient {
    client: Client,
}

impl RedisClient {
    pub fn new(host: String, port: u16) -> Self {
        Self {
            client: redis::Client::open(format!("redis://{host}:{port}")).unwrap(),
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
            if con.subscribe("sync").await.is_err() {
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
                Self::handle_redis_message(msg).await;
            }
        }
    }
    /// Handle a message received from redis
    async fn handle_redis_message(msg: Msg) {
        // Read message payload
        let payload = match msg.get_payload::<String>() {
            Ok(payload) => payload,
            Err(e) => {
                error!("Error getting payload from redis message: {0}", e);
                return;
            }
        };
        // Deserialize message payload
        let redis_message: RedisMessage = match serde_json::from_str(&payload) {
            Ok(msg) => msg,
            Err(e) => {
                error!("Error deserializing redis message: {0}", e);
                return;
            }
        };
        // Log the message
        bigquery::log_in_bq(
            redis_message.topic,
            String::from_utf8(redis_message.payload).expect("Could not decode payload (not utf-8)"),
        )
        .await;
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
