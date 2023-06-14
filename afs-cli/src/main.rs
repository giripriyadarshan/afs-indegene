use clap::Parser;
use std::io::Read;
use std::sync::Arc;
use std::{collections::HashSet, fs::File};

mod models;
mod utils;

use models::{args, config};
use tokio::task::JoinSet;
use utils::{
    check_veeva_session_id::check_veeva_session_id, compress::compress_folder_contents,
    get_key_messages::get_key_messages, get_keymessage_ids::get_keymessage_id,
    send_status_message::send_message,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = args::Arguments::parse();
    let args = Arc::new(args.run_code);

    // check if config.toml exists
    if !std::path::Path::new("config.toml").exists() {
        send_message(
            args.clone(),
            "ALL | FAILED | config.toml not found".to_string(),
        )
        .await;
        std::process::exit(1);
    }

    // read config.toml
    let mut file = File::open("config.toml").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let config: config::Config = toml::from_str(contents.as_str()).unwrap();

    // set session id and keymessages
    let session_id = check_veeva_session_id(args.clone(), config.vault.link.clone()).await;
    let key_messages_list_with_id = get_keymessage_id(
        args.clone(),
        config.vault.binder_id.clone(),
        config.vault.link.clone(),
        session_id.clone(),
    )
    .await;

    // list out only the keymessage names
    let all_key_messages_list: HashSet<String> = key_messages_list_with_id
        .clone()
        .into_iter()
        .map(|(k, _)| k)
        .collect();

    let key_messages = get_key_messages(all_key_messages_list);

    match key_messages {
        Some(key_messages) => {
            let key_message_ids_arc = Arc::new(key_messages_list_with_id);
            let session_id_arc = Arc::new(session_id);
            let vault_url_arc = Arc::new(config.vault.link);

            if !std::path::Path::new("output").exists() {
                std::fs::create_dir("output").unwrap();
            }
            let mut processes = JoinSet::new();
            for km in key_messages {
                let output = format!("output/{}.zip", km);
                let kmid = key_message_ids_arc.clone();
                let sid = session_id_arc.clone();
                let vua = vault_url_arc.clone();
                let args_run_code_clone = args.clone();
                processes.spawn(async move {
                    // check if km folder exists
                    if !std::path::Path::new(km.as_str()).exists() {
                        send_message(args_run_code_clone, format!("{} | FAILED | not found", km))
                            .await;
                        return;
                    }
                    // compress the km
                    compress_folder_contents(args_run_code_clone, km, output, vua, kmid, sid).await;
                });
            }

            while let Some(res) = processes.join_next().await {
                if res.is_err() {
                    send_message(args.clone(), format!("DEV | FAILED | {:?}", res)).await;
                }
            }

            // delete the output folder
            std::fs::remove_dir_all("output").unwrap();
        }
        None => {
            send_message(
                args.clone(),
                "ALL | SUCCESS | no new keymessages found".to_string(),
            )
            .await;
        }
    }

    Ok(())
}
