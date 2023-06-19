use reqwest::Client;

#[tauri::command]
pub async fn send_url(svn_url: &str) -> Result<String, String> {
    let client = Client::new();
    let res = client
        .post("http://172.30.106.6:8008/")
        // .post("http://localhost:8008/")
        .body(svn_url.to_string())
        .send()
        .await
        .unwrap();

    if res.status().is_success() {
        Ok(res.text().await.unwrap())
    } else {
        Err("Error".to_string())
    }
}
