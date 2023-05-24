use clap::Parser;
use std::fs::File;
use std::io::Read;

mod args;
mod config;
mod utils;

fn main() {
    // check if config.toml exists
    if !std::path::Path::new("config.toml").exists() {
        eprintln!("config.toml not found");
        std::process::exit(1);
    }

    // let args = args::Arguments::parse();
    // println!("{:?}", args);
    // let mut file = File::open("config.toml").unwrap();
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).expect("Failed to read config file");
    // let config: config::Config = toml::from_str(contents.as_str()).unwrap();

    // println!("{:?} {:?}", args, config);

    // let folder_path = "./my_folder";
    // let zip_file_path = "./my_archive.zip";

    // match utils::compress::compress_folder_contents(folder_path, zip_file_path) {
    //     Ok(()) => println!("Archive created successfully"),
    //     Err(e) => eprintln!("Error creating archive: {}", e),
    // }
}
