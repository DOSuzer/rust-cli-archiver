use std::path::{Path, PathBuf};
use std::fs::{File, create_dir_all};
use std::io::prelude::*;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};
use clap::{Parser, Subcommand};
use std::io;
use unrar::Archive;
use sevenz_rust::*;

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
    },
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Extract { file }) => {
            let file_result = file.as_deref();
            let result = match file_result {
                Some(archive) => {
                    println!("Archive path: {:?}", archive.to_str());
                    match archive.extension().and_then(|ext| ext.to_str()) {
                        Some("zip") => extract_zip(&archive),
                        Some("rar") => extract_rar(&archive),
                        Some("7z") => extract_sevenz(&archive),
                        Some(_) => panic!("Unknown archive type."),
                        None => panic!("Couldn't check archive type."),
                    }
                },
                None => panic!("Please provide path to archive, e.g. --file </path/to>."),
            };
            if let Err(e) = result {
                eprintln!("Error extracting archive: {}", e);
            } else {
                println!("Archive extracted successfully.");
            }
        }
        None => {
            let archive_name: String;

            let files_result = args.files.as_deref();
            let files = match files_result {
                Some(files) => {
                    println!("Files path: {:?}", files.to_str());
                    files
                },
                None => panic!("Please provide path to files."),
            };


            if let Some(name) = args.name.as_ref() {
                println!("Archive path: {name}");
                archive_name = name.to_string();
            } else {
                archive_name = "new_archive.zip".to_string();
                println!("Using default archive name: {archive_name}");
            }

            let path = Path::new(&archive_name);

            let result = match path.extension().and_then(|ext| ext.to_str()) {
                Some("7z") => create_7z(&archive_name, files),
                Some(_) => create_zip(&archive_name, files),
                None => create_zip(&archive_name, files),
            };
            if let Err(e) = result {
                eprintln!("Error creating archive: {}", e);
            }
        }
    }
}

fn create_zip(archive_name: &String, file: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(&archive_name);
    let checked_path: PathBuf;
    if path.extension().is_none() || path.extension().unwrap() != "zip" {
        checked_path = path.with_extension("zip");
    } else {
        checked_path = PathBuf::from(path);
    }

    let archive = File::create(checked_path).unwrap();
    let mut buffer = Vec::new();

    let mut zip = ZipWriter::new(archive);

    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o755);
    let mut f = File::open(file)?;
    zip.start_file(file.file_name().unwrap().to_str().unwrap(), options)?;
    f.read_to_end(&mut buffer)?;
    zip.write_all(&buffer)?;
    println!("File {} written to {:?}", file.to_str().unwrap(), path.with_extension("zip"));
    buffer.clear();

    zip.finish()?;
    Ok(())
}

fn create_7z(archive_name: &String, file: &Path) -> Result<(), Box<dyn std::error::Error>> {
    sevenz_rust::compress_to_path(&file, &archive_name).expect("compress ok");
    Ok(())
}

fn extract_zip(archive_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let fname = std::path::Path::new(archive_path);
    let file = File::open(fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path,
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {i} comment: {comment}");
            }
        }

        if file.is_dir() {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    create_dir_all(p).unwrap();
                }
            }
            let mut outfile = File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    Ok(())
}

fn extract_rar(archive_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut archive =
        Archive::new(archive_path)
            .open_for_processing()
            .unwrap();
    while let Some(header) = archive.read_header()? {
        println!(
            "{} bytes: {}",
            header.entry().unpacked_size,
            header.entry().filename.to_string_lossy(),
        );
        archive = if header.entry().is_file() {
            header.extract()?
        } else {
            header.skip()?
        };
    }
    Ok(())
}

fn extract_sevenz(archive_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    decompress_file(&archive_path, archive_path.parent().unwrap()).expect("complete");
    Ok(())
}
