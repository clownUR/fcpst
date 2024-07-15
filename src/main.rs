mod cli;
mod archiver;
mod extractor;

use clap::Parser;
use crate::cli::{Cli, RunMode};
use crate::archiver::for_archiver;
use crate::extractor::for_extractor;

//Result<T, E> この2つが返り値Ok(T), Err(E)
fn execute(cli_option: &Cli) -> Result<(), String> {
    match cli_option.execute_mode() {
        Ok(RunMode::Archive) => {
            //for_archiverの返り値をResult<(),String>にしたらerrになった時の操作をmatch文で行える？
            match for_archiver(cli_option.input.clone(), cli_option.dest.clone(), cli_option.output.clone()){
                Ok(_) => Ok(()),
                Err(error) => return Err(error)
            }
            
        },
        Ok(RunMode::Extract) => {
            match for_extractor(cli_option.input.clone(), cli_option.dest.clone(), cli_option.output.clone()){
                Ok(_) => Ok(()),
                Err(error) => return Err(error)
            }
           
        },
        Ok(RunMode::Auto) => {
            return Err(String::from("Autoモードは実装中です."))
        },
        Err(error) => return Err(format!("{}", error)),
    }
}

fn main() {
    let args = Cli::parse();

    match execute(&args){
        Ok(_) => {
            println!("正常終了しました");
        },
        Err(error) => {
            panic!("{:?}", error);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use clap::Parser;
    use crate::cli::{Cli, RunMode, Format}; // ここを修正

    #[test]
    fn test_run() {
        let opts = Cli::parse_from(&[
            "fcpst_test", 
            "-d", 
            "test.zip", 
            "--format", "zip",
            "src", 
            "LICENSE", 
            "README.md", 
            "Cargo.toml"
        ]);
        
        assert_eq!(opts.mode, RunMode::Auto);
        assert_eq!(opts.output, Format::Zip);
        assert_eq!(opts.input.len(), 4);
        assert_eq!(opts.input, vec![
            PathBuf::from("src"), 
            PathBuf::from("LICENSE"), 
            PathBuf::from("README.md"), 
            PathBuf::from("Cargo.toml")
        ]);
    }
}
