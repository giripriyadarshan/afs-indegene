// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils;

use utils::{
    cancel_subscription::unsubscribe, send_svn_url_to_api_server::send_url,
    subscribe_to_messages::subscribe,
};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![send_url, subscribe, unsubscribe])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
