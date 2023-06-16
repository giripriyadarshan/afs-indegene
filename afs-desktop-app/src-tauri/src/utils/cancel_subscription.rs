use lapin::{options::*, Connection, ConnectionProperties};

#[tauri::command]
pub async fn unsubscribe(run_code: String) -> Result<String, String> {
    let addr = "amqp://client:password@localhost:5672/%2f";

    let conn = Connection::connect(&addr, ConnectionProperties::default())
        .await
        .unwrap();
    let channel = conn.create_channel().await.unwrap();
    channel
        .basic_cancel(run_code.as_str(), BasicCancelOptions::default())
        .await
        .unwrap();

    Ok("".to_string())
}
