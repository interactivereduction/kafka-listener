mod processors;
mod utils;
mod schema;
mod types;

use std::env;
use futures::StreamExt;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use rdkafka::ClientConfig;
use rdkafka::error::KafkaError;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("Serde error {0}")]
    DeserializeError(#[from]serde_json::Error),
    #[error("Kafka error {0}")]
    KafkaError(#[from]KafkaError)
}

type Result<T> = std::result::Result<T, Error>;

struct DbInfo {
    name: String,
    username: String,
    password: String,
    ip: String
}

#[tokio::main]
async fn main() -> Result<()> {
    let db_info = DbInfo {
        name: "interactive-reduction".to_string(),
        username: env::var("DB_USERNAME").unwrap(),
        password: env::var("DB_PASSWORD").unwrap(),
        ip: env::var("DB_IP").unwrap(),
    };

    let database_url = format!("postgres://{}:{}@{}/{}", db_info.username, db_info.password, db_info.ip, db_info.name);
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let db_connection_pool = Pool::builder()
        .build(manager)
        .unwrap();

    let kafka_ip = env::var("KAFKA_IP").unwrap();

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", kafka_ip)
        .set("group.id", "kafka-listener")
        .create()?;

    consumer.subscribe(&["detected-runs", "completed-runs"])?;

    let mut message_stream = consumer.stream();

    while let Some(message) = message_stream.next().await {
        match message {
            Err(e) => eprintln!("Error receiving message: {:?}", e),
            Ok(msg) => if let Err(error_on_message_handle) = match msg.topic() {
                "detected-runs" => processors::detected_runs(msg.payload().unwrap(),
                                                             db_connection_pool.clone()).await,
                "completed-runs" => processors::completed_runs(msg.payload().unwrap(),
                                                               db_connection_pool.clone()).await,
                _ => Ok(()),
            }{
                // Called when there is an error with variable error_on_message_handle.
                eprintln!("Error processing message: {:?}", error_on_message_handle)
            } ,
        }
    }

    Ok(())
}
