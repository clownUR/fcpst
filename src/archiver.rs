pub mod zip;
pub mod tar;
use std::path::PathBuf;
use crate::cli::Format;

pub fn for_archiver(files: Vec<PathBuf>, dest: Option<PathBuf>, formatter:Format) -> Result<(), String>{
  match formatter {
    Format::Zip => {
        if let Err(error) = zip::zip_writer(files, dest) {
            return Err(format!("zipファイルの作成に失敗しました: {}", error));
        }else{
            return Ok(());
        }
    },
    Format::Tar => {
        if let Err(error) = tar::tar_writer(files, dest) {
            return Err(format!("tarファイルの作成に失敗しました: {:?}", error));
        }else{
            return Ok(());
        }
    },
    // _ => {
    //     return Err(String::from("Unsupported format"));
    // }
  }

}

//テストコード
