use clap::Parser;

mod utils;
mod args;

fn main() {
    let args = args::Arguments::parse();

    let folder_path = "./my_folder";
    let zip_file_path = "./my_archive.zip";

    match utils::compress::compress_folder_contents(folder_path, zip_file_path) {
        Ok(()) => println!("Archive created successfully"),
        Err(e) => eprintln!("Error creating archive: {}", e),
    }
}
