use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(
    version, author, about,
    arg_required_else_help = true,
)]

pub struct Cli {
    #[clap(short = 'm', long = "mode", default_value_t = RunMode::Auto, value_name = "MODE", required = false, ignore_case = true, value_enum, help = "archive,extractモードのどちらかを選択する")]
    pub mode: RunMode,
    
    #[clap(short = 'f', long = "format", value_name = "FORMAT", required = false, ignore_case = true, value_enum, help = "フォーマットを選択する．")]
    pub output: Format,

    #[clap(short = 'd', long = "dest", alias = "dest", value_name = "DEST", required = false, help = "出力先のディレクトリを指定する")]
    pub dest: Option<PathBuf>,

    #[clap(value_name = "INPUT", help = "アーカイブもしくは展開したいファイルやディレクトリを指定")]
    pub input: Vec<PathBuf>,
}

#[derive(Clone, ValueEnum, Debug, PartialEq)]
pub enum RunMode {
  Auto,
  Archive,
  Extract,
}

#[derive(Clone, ValueEnum, Debug, PartialEq)]
pub enum Format{
  Zip,
  Tar,
  //これから増やしていく
}

//エラーの実装

impl Cli{
  pub fn execute_mode(&self) -> Result<RunMode, String>{

    if self.input.len() == 0{
      return Err(String::from("ファイルが指定されていません."))
    }else{
      return Ok(self.mode.clone())
    }
    
    // if self.mode == RunMode::Auto || self.mode == RunMode::Archive || self.mode == RunMode::Extract{
    //   return Ok(self.mode.clone())
    // }else{
    //   return Err(String::from("そのようなモードは存在しません．"))
    // }  
  }
}


//テストコード
