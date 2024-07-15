pub mod zip;
pub mod tar;
use std::path::PathBuf;
use crate::cli::Format;

pub fn for_extractor(files: Vec<PathBuf>, dest: Option<PathBuf>, formatter:Format) -> Result<(), std::string::String>{
    match formatter {
        Format::Zip => {
            if let Err(error) = zip::execute_unzip(files, dest) {
                return Err(format!("zipファイルの展開に失敗しました: {:?}", error));
            }else{
                return Ok(());
            }
        },
        Format::Tar => {
            if let Err(error) = tar::extract_tar(files, dest) {
                return Err(format!("tarファイルの展開に失敗しました: {:?}", error));
            }else{
                return Ok(());
            }
        },
        // _ => {
        //     return Err(String::from("Unsupported format"));
        // }
      }
    //Ok(())
    
    // println!("{}", filepath.display());
}

//テストコード