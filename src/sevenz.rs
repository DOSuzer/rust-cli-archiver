use std::path::Path;
use sevenz_rust::{compress_to_path, decompress_file};

pub fn create_7z(archive_name: &Path, file: &Path) -> Result<(), Box<dyn std::error::Error>> {
    compress_to_path(&file, &archive_name).expect("compress ok");
    Ok(())
}

pub fn extract_sevenz(archive_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    decompress_file(&archive_path, std::env::current_dir().unwrap()).expect("complete");
    Ok(())
}
