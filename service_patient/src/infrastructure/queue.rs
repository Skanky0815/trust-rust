use lapin::{Channel, Connection, ConnectionProperties};

pub async fn setup_queue() -> Channel {
    let queue_url = std::env::var("QUEUE_URL").expect("QUEUE_URL must be set!");

    let queue_connection = Connection::connect(&queue_url, ConnectionProperties::default())
        .await
        .expect("Failed to connect to queue!");

    let channel = queue_connection
        .create_channel()
        .await
        .expect("Failed to create channel!");

    channel
}
