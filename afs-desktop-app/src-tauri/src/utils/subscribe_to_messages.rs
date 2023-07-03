use futures_lite::stream::StreamExt;
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};
// use std::future::Future;
use std::{thread, time};
use tauri::Manager;

async fn subscribe_to_messages(run_code: String, app: tauri::AppHandle) -> Result<String, String> {
    let addr = "amqp://client:password@localhost:5672/%2f";
    let x = async_global_executor::block_on(async {
        let mut s = String::new();
        let conn = Connection::connect(&addr, ConnectionProperties::default())
            .await
            .unwrap();

        let mut tries = 0;
        let mut consumer = loop {
            tries += 1;
            if tries > 100 {
                return Ok(s) as Result<String, ()>;
            }
            let channel = conn.create_channel().await.unwrap();
            let channel = channel
                .basic_consume(
                    run_code.as_str(),
                    run_code.as_str(),
                    BasicConsumeOptions::default(),
                    FieldTable::default(),
                )
                .await;
            match channel {
                Ok(consumer) => {
                    println!("Consumer created");
                    break consumer;
                }
                Err(e) => {
                    println!("{:?}", e);
                    thread::sleep(time::Duration::from_secs(1));
                    continue;
                }
            }
        };

        while let Some(delivery) = consumer.next().await {
            let delivery = delivery.expect("error in consumer");
            delivery.ack(BasicAckOptions::default()).await.unwrap();
            s = std::str::from_utf8(&delivery.data).unwrap().to_owned();
            println!("Received: {}", s);
            app.emit_all("CurrentMessage", s.clone()).unwrap();
        }
        Ok(s) as Result<String, ()>
    })
    .unwrap();

    Ok(x)
}

#[tauri::command]
pub async fn subscribe(run_code: String, app: tauri::AppHandle) -> Result<String, String> {
    let res = subscribe_to_messages(run_code, app).await.unwrap();
    Ok(res)
}
