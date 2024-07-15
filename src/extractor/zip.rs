use std::fs::File;
use std::path::PathBuf;
use std::env;
use std::fs;
use std::io;

pub fn execute_unzip(unzip_files: Vec<PathBuf>, mut dest: Option<PathBuf>) -> Result<(), String> {
    // デフォルトの出力先ディレクトリを設定
    if dest.is_none() {
        dest = match env::current_dir() {
            Ok(current_dir) => Some(current_dir),
            Err(error) => return Err(format!("カレントディレクトリの取得に失敗しました: {}", error)),
        }
    } else {
        match fs::create_dir_all(dest.clone().unwrap()) {
            Ok(_) => (),
            Err(error) => return Err(format!("ディレクトリの作成に失敗しました: {}", error)),
        }
    }

    for unzip_file in unzip_files {
        //println!("{}", unzip_file.display());
        let file_object = match File::open(&unzip_file) {
            Ok(file_obj) => file_obj,
            Err(error) => return Err(format!("ファイルを開くことができません {}: {}", unzip_file.display(), error)),
        };

        let mut zip = match zip::ZipArchive::new(file_object) {
            Ok(zip_archive) => zip_archive,
            Err(error) => return Err(format!("zipファイルの読み込みに失敗しました {}: {}", unzip_file.display(), error)),
        };

        for index in 0..zip.len() {
            let mut file = match zip.by_index(index) {
                Ok(a_file) => a_file,
                Err(error) => return Err(format!("zipファイルのインデックス{}へのアクセスに失敗しました: {}", index, error)),
            };

            // メタデータの排除
            let name = file.name();
            if name.starts_with("__MACOSX") || name == ".DS_Store" {
                continue;
            }
            let extract_path = dest.as_ref().unwrap().join(&file.name().to_string());

            if file.is_file() {
                match File::create(&extract_path) {
                    Ok(mut outfile) => {
                        //println!("File details1: {:?}", file.name());
                        match io::copy(&mut file, &mut outfile) {
                            Ok(_) => (),
                            Err(error) => return Err(format!("ファイルコピーに失敗しました {}: {}", file.name(), error)),
                        }
                    }
                    
                    Err(error) => return Err(format!("ファイルの作成に失敗しました {}: {}", extract_path.display(), error)),
                }
            } else {
                return Err(format!("{}は存在しない", unzip_file.display()));
            }
        }
    }

    Ok(())
}

//テストコード