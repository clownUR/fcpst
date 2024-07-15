use std::fs::File;
use std::path::PathBuf;
use tar::Archive;
use std::env;
use std::fs;

/// Tarファイルを指定されたディレクトリに展開する
pub fn extract_tar(extract_files: Vec<PathBuf>, mut dest: Option<PathBuf>) -> Result<(), String> {
  if dest.is_none() {
    dest = match env::current_dir() {
        Ok(current_dir) => Some(current_dir),
        Err(error) => return Err(format!("Failed to get current directory: {}", error)),
    }
  }else{
    match fs::create_dir_all(dest.clone().unwrap()) {
      Ok(_) => (),
      Err(error) => return Err(format!("Failed to create directory: {}", error)),
    }
  }

  for a_file in extract_files{
    let file = match File::open(&a_file) {
      Ok(file) => file,
      Err(error) => return Err(format!("Failed to open file {}: {}", a_file.display(), error)),
    };
    
    let mut tar_archive = Archive::new(file);
    match tar_archive.unpack(dest.clone().unwrap()) {
      Ok(_) => (),
      Err(error) => return Err(format!("Failed to unpack tar archive: {}", error)),
    };
  }

  Ok(())
}
