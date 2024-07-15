use std::fs::File;
use std::path::{Path, PathBuf};
use tar::Builder;
use std::env;
use std::fs;

pub fn tar_writer(files_to_tar: Vec<PathBuf>, mut dest: Option<PathBuf>) -> Result<(), String> {

    if dest.is_none() {
        dest = match env::current_dir() {
            Ok(dest) => Some(dest.join("fcpst.tar")),
            Err(error) => return Err(error.to_string()),
        };
    } else {
        if let Err(error) = fs::create_dir_all(dest.clone().unwrap()) {
            return Err(error.to_string());
        }
        dest = Some(dest.unwrap().join("fcpst.tar"));
    }

    let tar_file_object = match File::create(dest.unwrap()) {
        Ok(file_object) => file_object,
        Err(error) => return Err(format!("tarファイルの作成に失敗しました: {}", error)),
    };

    let mut builder = Builder::new(tar_file_object);

    for a_file in files_to_tar {
        let path = Path::new(&a_file);

        if path.is_dir() {
            match builder.append_dir_all(path.file_name().unwrap(), path) {
                Ok(_) => (),
                Err(error) => return Err(format!("ディレクトリの追加に失敗しました: {}", error)),
            }
        } else if path.is_file() {
            match builder.append_path_with_name(path, path.file_name().unwrap()) {
                Ok(_) => (),
                Err(error) => return Err(format!("ファイルの追加に失敗しました: {}", error)),
            }
        } else {
            return Err(format!("{}は存在しません", a_file.display()));
        }
    }

    match builder.finish() {
        Ok(_) => Ok(()),
        Err(e) => return Err(format!("Failed to finish tar archive: {}", e)),
    }

}
