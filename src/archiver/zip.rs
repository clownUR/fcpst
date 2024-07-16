use std::fs::File;
use std::io::{Write, Read, BufReader, Seek};
use std::path::{Path, PathBuf};
use zip::write::{FileOptions, ZipWriter, ExtendedFileOptions};
use zip::CompressionMethod;
use std::env;
use std::fs::{self, read_dir};

pub fn zip_writer(files_to_zip: Vec<PathBuf>, mut dest: Option<PathBuf>) -> Result<(), String> {
    // デフォルトではカレントディレクトリの直下にzipファイルを配置する
    if dest.is_none() {
        dest = match env::current_dir() {
            Ok(dest) => Some(dest.join("fcpst.zip")),
            Err(error) => return Err(error.to_string()),
        };
    } else {
        if let Err(error) = fs::create_dir_all(dest.clone().unwrap()) {
            return Err(error.to_string());
        }
        dest = Some(dest.unwrap().join("fcpst.zip"));
    }

    let zip_path = dest.unwrap();
    let output_path = Path::new(&zip_path);
    let file = match File::create(&output_path) {
        Ok(file) => file,
        Err(error) => return Err(error.to_string()),
    };

    let mut zip = ZipWriter::new(file);
    let options: FileOptions<ExtendedFileOptions> = FileOptions::default().compression_method(CompressionMethod::Stored);

    //println!("{:?}", files_to_zip);

    for filename in files_to_zip {
        let file_path = Path::new(&filename);
        if file_path.is_dir() {
            if let Err(error) = directory_zip_archive(&mut zip, &file_path, &options, "") {
                return Err(error.to_string());
            }
        } else if file_path.is_file() {
            if let Err(error) = files_zip_archive(&mut zip, &file_path, &options, "") {
                return Err(error.to_string());
            }
        } else {
            return Err(format!("{}は存在しません", filename.display()));
        }
    }

    Ok(())
}

fn directory_zip_archive<W: Write + Seek>(zip: &mut ZipWriter<W>, dir_path: &Path, options: &FileOptions<ExtendedFileOptions>, base_path: &str) -> Result<(), String> {
    let dir_name = format!("{}/", dir_path.file_name().unwrap().to_string_lossy());
    let full_path = format!("{}{}", base_path, dir_name);
    if let Err(error) = zip.add_directory(&full_path, options.clone()) {
        return Err(error.to_string());
    }

    let entries = match read_dir(dir_path) {
        Ok(entries) => entries,
        Err(error) => return Err(error.to_string()),
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => return Err(error.to_string()),
        };
        let path = entry.path();
        if path.is_dir() {
            if let Err(error) = directory_zip_archive(zip, &path, options, &full_path) {
                return Err(error.to_string());
            }
        } else {
            if let Err(error) = files_zip_archive(zip, &path, options, &full_path) {
                return Err(error.to_string());
            }
        }
    }
    Ok(())
}

fn files_zip_archive<W: Write + Seek>(zip: &mut ZipWriter<W>, file_path: &Path, options: &FileOptions<ExtendedFileOptions>, base_path: &str) -> Result<(), String> {
    let file_name = file_path.file_name().unwrap().to_string_lossy();
    let full_path = format!("{}{}", base_path, file_name);

    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => return Err(error.to_string()),
    };

    let mut reader = BufReader::new(file);
    if let Err(error) = zip.start_file(&*full_path, options.clone()) {
        return Err(error.to_string());
    }

    let mut buffer = Vec::new();
    if let Err(error) = reader.read_to_end(&mut buffer) {
        return Err(error.to_string());
    }

    if let Err(error) = zip.write_all(&buffer) {
        return Err(error.to_string());
    }
    Ok(())
}