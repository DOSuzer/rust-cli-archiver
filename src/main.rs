use std::path::{Path, PathBuf};
use clap::{Parser, Subcommand};

use crate::models::Archive;

pub mod models;
pub mod zip;
pub mod rar;
pub mod sevenz;


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to file to be archived
    #[arg(short, long)]
    files: Option<PathBuf>,

    /// Archive name with extension
    #[arg(short, long)]
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Extract archive to current location
    Extract {
        /// Path to archive
        #[arg(short, long)]
        file: Option<PathBuf>,
        /// Path where to extract
        #[arg(short, long)]
        destination: Option<PathBuf>,
    },
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Extract { file, destination }) => {
            let mut existing_archive: Archive;

            match file {
                Some(archive_path) => {
                    existing_archive = Archive::new(archive_path.file_stem()
                        .and_then(|s| s.to_str())
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| panic!("Failed to find archive path")))
                        .expect("Failed to create Archive");
                    existing_archive.path = archive_path.to_path_buf();
                    existing_archive.archive_extension = match archive_path.extension().and_then(|ext| ext.to_str()) {
                        Some(ext) => ext.to_string(),
                        None => panic!("Couldn't check archive type."),
                    }
                },
                None => panic!("Please provide path to archive, e.g. --file </path/to>."),
            };

            match destination {
                Some(path) => existing_archive.extract_path = path.to_path_buf(),
                None => {
                    println!("Destination not provided, extracting to current folder.");
                    existing_archive.extract_path = std::env::current_dir().unwrap();
                },
            }

            existing_archive.extract().expect("Failed to extract archive");

        },
        None => {
            let mut new_archive: Archive;

            if let Some(name) = args.name.as_ref() {
                let path = Path::new(&name);
                new_archive = Archive::new(path.file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "default_name".to_string())).unwrap();
                if path.extension().is_none() {
                    new_archive.path = path.with_extension("zip");
                    new_archive.archive_extension = "zip".to_string();
                } else {
                    new_archive.path = PathBuf::from(path);
                    new_archive.archive_extension = match path.extension().and_then(|ext| ext.to_str()) {
                        Some(ext) => ext.to_string(),
                        None => panic!("Couldn't check archive type."),
                    };
                }
                println!("Archive name: {}, extension: {}", new_archive.archive_name, new_archive.archive_extension);
            } else {
                new_archive = Archive::new("new_archive.zip".to_string()).unwrap();
                new_archive.archive_extension = "zip".to_string();
                println!("Using default archive name: {}", new_archive.archive_name);
            }

            new_archive.file_path = match args.files {
                Some(files) => {
                    println!("Files path: {:?}", files.to_str());
                    files
                },
                None => panic!("Please provide path to files."),
            };

            new_archive.create().expect("Failed to create archive!");

        }
    }
}
