use std::path::Path;

use clap::Parser;

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
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short,long,value_parser=verify_input_file)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(filename: &str) -> anyhow::Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err(format!("文件{}不存在", filename))
    }
}
// fn verify_input_file(filename: &str) -> Result<String,&'static str> {
//     if Path::new(filename).exists() {
//         Ok(filename.into())
//     } else {
//         Err("文件不存在")
//     }
// }
