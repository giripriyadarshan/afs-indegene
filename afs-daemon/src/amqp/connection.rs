use actix_web::web::Data;
use amqprs::DELIVERY_MODE_PERSISTENT;
use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{BasicPublishArguments, Channel},
    connection::{Connection, OpenConnectionArguments},
    BasicProperties, FieldTable,
};
use std::time;

use crate::models::{RabbitConnect, AmqpState};

pub async fn connect_rabbitmq(connection_details: &RabbitConnect) -> Connection {
    let mut res = Connection::open(
        &OpenConnectionArguments::new(
            &connection_details.host,
            connection_details.port,
            &connection_details.username,
            &connection_details.password,
        )
        .virtual_host("/"),
    )
    .await;

    while res.is_err() {
        println!("trying to connect after error");
        std::thread::sleep(time::Duration::from_millis(2000));
        res = Connection::open(&OpenConnectionArguments::new(
            &connection_details.host,
            connection_details.port,
            &connection_details.username,
            &connection_details.password,
        ))
        .await;
    }

    let connection = res.unwrap();
    connection
        .register_callback(DefaultConnectionCallback)
        .await
        .unwrap();
    connection
}

pub async fn channel_rabbitmq(connection: &amqprs::connection::Connection) -> Channel {
    let channel = connection.open_channel(None).await.unwrap();
    channel
        .register_callback(DefaultChannelCallback)
        .await
        .unwrap();
    return channel;
}

pub async fn send(
    connection: &mut amqprs::connection::Connection,
    channel: &mut Channel,
    connection_details: &RabbitConnect,
    amqp_state: Data<AmqpState>,
    response: String,
    run_code: String,
) {
    if !connection.is_open() {
        println!("Connection not open");
        *connection = connect_rabbitmq(connection_details).await;
        *channel = channel_rabbitmq(&connection).await;
        println!("{}", connection);
    }

    if !channel.is_open() {
        println!("channel is not open, does exchange systemmonitor exist on rabbitMQ?");
        *channel = channel_rabbitmq(&connection).await;
    } else {
        let mut headers = FieldTable::new();
        headers.insert("run_code".try_into().unwrap(), run_code.try_into().unwrap());
        let args = BasicPublishArguments::new("afs-status", "");
        channel
            .basic_publish(
                BasicProperties::default()
                    .with_delivery_mode(DELIVERY_MODE_PERSISTENT)
                    .with_headers(headers)
                    .finish(),
                response.into(),
                args,
            )
            .await
            .unwrap();
    }
}
