use crate::models::AppState;
use actix_web::web::Data;
use deadpool_lapin::lapin::types::{AMQPValue, FieldTable, LongUInt, ShortString};
use deadpool_lapin::lapin::{
    options::{BasicPublishOptions, QueueBindOptions, QueueDeclareOptions},
    BasicProperties,
};

pub async fn send_message(
    state: Data<AppState>,
    message: String,
    run_code: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let connection = state.amqp_pool.get().await.unwrap();
    let channel = connection.create_channel().await.unwrap();
    let payload = message.as_bytes();
    let options = BasicPublishOptions::default();

    let properties = BasicProperties::default();
    let mut channel_args = FieldTable::default();

    channel_args.insert(
        ShortString::from("x-message-ttl"),
        AMQPValue::LongUInt(LongUInt::from(60000_u32)),
    );
    channel
        .queue_declare(
            run_code.as_str(),
            QueueDeclareOptions::default(),
            channel_args,
        )
        .await
        .unwrap();

    channel
        .queue_bind(
            run_code.as_str(),
            "afs-status",
            run_code.as_str(),
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    channel
        .basic_publish(
            "afs-status",
            run_code.as_str(),
            options,
            payload,
            properties,
        )
        .await?;
    Ok(())
}
