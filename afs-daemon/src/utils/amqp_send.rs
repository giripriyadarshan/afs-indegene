use crate::models::AppState;
use actix_web::web::Data;
use deadpool_lapin::lapin::types::FieldTable;
use deadpool_lapin::lapin::types::LongString;
use deadpool_lapin::lapin::{options::BasicPublishOptions, BasicProperties};

pub async fn send_message(
    state: Data<AppState>,
    message: String,
    run_code: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let connection = state.amqp_pool.get().await.unwrap();
    let channel = connection.create_channel().await.unwrap();
    let payload = message.as_bytes();
    let options = BasicPublishOptions::default();
    let mut header = FieldTable::default();
    header.insert(
        "run_code".into(),
        lapin::types::AMQPValue::LongString(LongString::from(run_code)),
    );
    let properties = BasicProperties::default().with_headers(header);
    channel
        .basic_publish("afs-status", "", options, payload, properties)
        .await?;
    Ok(())
}
