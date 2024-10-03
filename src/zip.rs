use std::path::{Path, PathBuf};
use std::fs::{File, create_dir_all};
use std::io::prelude::*;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};
use std::io;

pub fn create_zip(checked_path: &PathBuf, file: &Path) -> Result<(), Box<dyn std::error::Error>> {

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
    println!("File {} written to {:?}", file.to_str().unwrap(), checked_path.to_str().unwrap());
    buffer.clear();

    zip.finish()?;
    Ok(())
}

pub fn extract_zip(archive_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
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
