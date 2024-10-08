use std::path::PathBuf;
use zip::CompressionMethod;

use crate::zip::{extract_zip, create_zip};
use crate::rar::extract_rar;
use crate::sevenz::{create_7z, extract_sevenz};

enum CompressionMethods {
    Deflated,
    Stored,
    Zstd,
    Lzma,
    Xz
}

enum ContentType {
    Doc,
    Media,
    Mixed,
}

pub struct Archive {
    pub archive_name: String,
    pub path: PathBuf,
    pub archive_create_path: PathBuf,
    pub archive_extension: String,
    pub extract_path: PathBuf,
    pub files_names: Vec<String>,
    pub file_name: String,
    pub file_path: PathBuf,
    pub compression_method: CompressionMethod,
    pub password: String,
    pub content_type: ContentType,
}

impl Archive {
    pub fn new(archive_name: String) -> Result<Self, std::io::Error> {
        Ok(Archive {
            archive_name,
            path: PathBuf::new(),
            archive_create_path: PathBuf::new(),
            archive_extension: String::new(),
            extract_path: PathBuf::new(),
            files_names: Vec::new(),
            file_name: String::new(),
            file_path: PathBuf::new(),
            compression_method: CompressionMethod::Deflated, // Default compression method
            password: String::new(),
            content_type: ContentType::Mixed, // Default content type
        })
    }
    pub fn extract(&self) -> Result<(), Box<dyn std::error::Error>> {
        let result = match self.archive_extension.as_str() {
            "7z" => extract_sevenz(&self.path),
            "zip" => extract_zip(&self.path),
            "rar" => extract_rar(&self.path),
            _ => panic!("This archive type is not supported yet. Sorry:("),
        };
        if let Err(e) = result {
            eprintln!("Error extracting archive: {}", e);
        };
        Ok(())
    }
    pub fn create(&self) -> Result<(), Box<dyn std::error::Error>> {
        let result = match self.archive_extension.as_str() {
            "7z" => create_7z(&self.path, &self.file_path),
            _ => create_zip(&self.path, &self.file_path),
        };
        if let Err(e) = result {
            eprintln!("Error creating archive: {}", e);
        };
        Ok(())
    }
    pub fn encrypt(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    pub fn get_archive_params(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    pub fn read_content(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
