use reqwest::{header::CONTENT_TYPE, Client};
use std::sync::Arc;

pub async fn send_message(run_code: Arc<String>, message: String) {
    let client = Client::new();
    let res = client
        .post("http://localhost:3825")
        .header(CONTENT_TYPE, "application/json")
        .body(
            serde_json::to_string(&serde_json::json!({
                "run_code": run_code.as_str(),
                "message": message,
            }))
            .unwrap(),
        )
        .send()
        .await;

    match res {
        Ok(_) => {}
        Err(e) => {
            println!("Error sending message: {}", e);
        }
    }
}
