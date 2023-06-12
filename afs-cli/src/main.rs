// use clap::Parser;
use std::fs::File;
use std::io::Read;

mod models;
mod utils;

use models::config;
use tokio::task::JoinSet;
use utils::{compress::compress_folder_contents, get_key_messages::get_key_messages};

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

    let key_messages = get_key_messages(config.files.key_messages_file);

    match key_messages {
        Some(key_messages) => {
            if !std::path::Path::new("output").exists() {
                std::fs::create_dir("output").unwrap();
            }
            let mut processes = JoinSet::new();
            for km in key_messages {
                let output = format!("output/{}.zip", km);
                processes.spawn(async move {
                    // check if km folder exists
                    if !std::path::Path::new(km.as_str()).exists() {
                        eprintln!("{} not found", km);
                        std::process::exit(1);
                    }
                    // compress the km
                    compress_folder_contents(km, output);
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
