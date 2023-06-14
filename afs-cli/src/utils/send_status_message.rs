use reqwest::{header::CONTENT_TYPE, Client};
use std::sync::Arc;

pub async fn send_message(run_code: Arc<String>, message: String) {
    let client = Client::new();
    client
        .post("http://172.30.106.6:3825")
        .header(CONTENT_TYPE, "application/json")
        .body(
            serde_json::to_string(&serde_json::json!({
                "run_code": run_code.as_str(),
                "message": message,
            }))
            .unwrap(),
        )
        .send()
        .await
        .unwrap();
}
