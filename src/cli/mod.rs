use std::path::Path;

use clap::Parser;

mod base64;
mod csv;
mod genpass;

pub use base64::*;
pub use csv::*;
pub use genpass::*;

#[derive(Debug, Parser)]
#[command(name="clitools",version,author,about,long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub command: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "展示csv内容,或者转换为json格式")]
    Csv(CsvOpts),

    // 随机密码
    #[command(name = "genpass", about = "生成随机密码")]
    GenPass(GenPassOpts),

    #[command(subcommand)]
    Base64(Base64SubCommand),
}

pub fn verify_input_file(filename: &str) -> anyhow::Result<String, String> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err(format!("文件{}不存在", filename))
    }
}

/**
 * config test
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(
            verify_input_file("fixtures/output.json"),
            Ok("fixtures/output.json".into())
        );
        assert_eq!(
            verify_input_file("not_exist.txt"),
            Err("文件not_exist.txt不存在".into())
        );
    }
}
