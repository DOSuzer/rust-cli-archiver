use std::path::{Path, PathBuf};
use std::fs::{File, create_dir_all};
use std::io::prelude::*;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};
use zip::result;
use clap::{Parser, Subcommand};
use std::io;
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to file to be archived
    #[arg(short, long)]
    files: Option<PathBuf>,

    /// Archive name
    #[arg(short, long)]
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Extract archive to same location
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
            let _archive = match file_result {
                Some(archive) => {
                    println!("Archive path: {:?}", archive.to_str());
                    extract_zip(archive)
                },
                None => panic!("Please provide path to archive."),
            };
        }
        None => {
            let mut archive_name = String::new();

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

            match doit(&archive_name, files) {
                Ok(_) => println!("Done!"),
                Err(e) => println!("Error: {e:?}"),
            }
        }
    }
}

fn doit(archive_name: &String, file: &Path) -> result::ZipResult<()> {
    let path = Path::new(&archive_name);
    let mut checked_path = PathBuf::new();
    if path.extension().is_none() || path.extension().unwrap() != "zip" {
        checked_path = path.with_extension("zip");
        println!("1: {:?}", checked_path)
    } else {
        checked_path = PathBuf::from(path);
        println!("2: {:?}", checked_path)
    }
    println!("3: {:?}", checked_path);

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

fn extract_zip(archive_path: &Path) -> result::ZipResult<()> {
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
