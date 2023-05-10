use std::fs::{self, File};
use zip::{result::ZipError, ZipWriter};

pub fn compress_folder_contents(folder_path: &str, zip_file_path: &str) -> Result<(), ZipError> {
    // Create a new zip file at the specified path
    let zip_file = File::create(zip_file_path)?;
    let mut zip_writer = ZipWriter::new(zip_file);

    // Recursively iterate over each file and folder in the specified folder and add them to the zip archive
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // If the path is a directory, recursively add its contents to the archive
            let folder_name = path.file_name().unwrap().to_str().unwrap();
            zip_writer.add_directory(folder_name, Default::default())?;
            compress_folder_contents(path.to_str().unwrap(), zip_file_path)?;
        } else {
            // Otherwise, add the file to the archive
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let mut file = File::open(&path)?;
            zip_writer.start_file(file_name, Default::default())?;
            std::io::copy(&mut file, &mut zip_writer)?;
        }
    }

    // Finish writing the zip file and return Ok if successful
    zip_writer.finish()?;
    Ok(())
}
