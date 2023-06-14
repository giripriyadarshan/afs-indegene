use std::fs::File;
use std::sync::Arc;
use toml::Table;
use walkdir::WalkDir;
use zip::ZipWriter;

use crate::utils::upload_to_veeva::upload_to_vault;

pub async fn compress_folder_contents(
    run_code: Arc<String>,
    folder_path: String,
    zip_file_path: String,
    vault_url: Arc<String>,
    key_message_id: Arc<Table>,
    session_id: Arc<String>,
) {
    let folder_path = folder_path.as_str();
    // Create a new zip file at the specified path
    let zip_file = File::create(zip_file_path.clone()).unwrap();
    let mut zip_writer = ZipWriter::new(zip_file);

    // Recursively iterate over each file and folder in the specified folder and add them to the zip archive
    for entry in WalkDir::new(folder_path).follow_links(true) {
        let entry = entry.unwrap();
        let entry_path = entry.path();

        if entry_path.is_file() {
            // If the path is a file, add it to the archive
            let relative_path = entry_path.strip_prefix(folder_path).unwrap();
            let mut file = File::open(entry_path).unwrap();
            zip_writer
                .start_file(
                    relative_path.to_str().unwrap().to_owned(),
                    Default::default(),
                )
                .unwrap();
            std::io::copy(&mut file, &mut zip_writer).unwrap();
        } else if entry_path.is_dir() {
            // If the path is a directory, create a directory entry in the archive
            let relative_path = entry_path.strip_prefix(folder_path).unwrap();
            zip_writer
                .add_directory(
                    relative_path.to_str().unwrap().to_owned(),
                    Default::default(),
                )
                .unwrap();
        }
    }

    // Finish writing the zip file
    zip_writer.finish().unwrap();

    let upload_process = upload_to_vault(
        run_code,
        folder_path.to_owned(),
        vault_url,
        zip_file_path,
        key_message_id,
        session_id,
    );

    upload_process.await;
}
