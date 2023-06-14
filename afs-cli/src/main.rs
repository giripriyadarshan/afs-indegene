// use clap::Parser;
use std::io::Read;
use std::sync::Arc;
use std::{collections::HashSet, fs::File};

mod models;
mod utils;

use models::config;
use tokio::task::JoinSet;
use utils::{
    check_veeva_session_id::check_veeva_session_id, compress::compress_folder_contents,
    get_key_messages::get_key_messages, get_keymessage_ids::get_keymessage_id,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // check if config.toml exists
    if !std::path::Path::new("config.toml").exists() {
        eprintln!("config.toml not found");
        std::process::exit(1);
    }

    // let args = args::Arguments::parse();
    // println!("{:?}", args);
    let mut file = File::open("config.toml").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read config file");
    let config: config::Config = toml::from_str(contents.as_str()).unwrap();

    // println!("{:?} {:?}", args, config);

    let session_id = check_veeva_session_id(config.vault.link.clone()).await;
    let key_messages_list_with_id = get_keymessage_id(
        config.vault.binder_id.clone(),
        config.vault.link.clone(),
        session_id.clone(),
    )
    .await;

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
                processes.spawn(async move {
                    // check if km folder exists
                    if !std::path::Path::new(km.as_str()).exists() {
                        eprintln!("{} not found", km);
                        std::process::exit(1);
                    }
                    // compress the km
                    compress_folder_contents(km, output, vua, kmid, sid).await;
                });
            }

            while let Some(res) = processes.join_next().await {
                if res.is_err() {
                    eprintln!("Error: {:?}", res);
                }
            }
            // delete the output folder
            // std::fs::remove_dir_all("output").unwrap();
        }
        None => {
            println!("No new key messages");
        }
    }

    // let folder_path = args.run_code.as_str();

    // match utils::compress::compress_folder_contents(folder_path, "zip_file_path").await {
    //     Ok(()) => println!("Archive created successfully"),
    //     Err(e) => eprintln!("Error creating archive: {}", e),
    // }

    Ok(())
}
