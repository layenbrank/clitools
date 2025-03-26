use std::{fmt, str::FromStr};

use clap::Parser;

use super::verify_input_file;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short,long,value_parser=verify_input_file)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long, value_parser=parse_format , default_value = "json")]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

// fn verify_input_file(filename: &str) -> Result<String,&'static str> {
//     if Path::new(filename).exists() {
//         Ok(filename.into())
//     } else {
//         Err("文件不存在")
//     }
// }

fn parse_format(format: &str) -> anyhow::Result<OutputFormat, anyhow::Error> {
    format.parse()
}

impl From<OutputFormat> for &'static str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            v => anyhow::bail!("不支持的格式:{}", v),
        }
    }
}

// impl TryFrom<&str> for OutputFormat {
//     type Error = anyhow::Error;
//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         match value.to_lowercase().as_str() {
//             "json" => Ok(OutputFormat::Json),
//             "yaml" => Ok(OutputFormat::Yaml),
//             v => anyhow::bail!("不支持的格式:{}", v),
//         }
//     }

// }

// impl From<&'static str> for OutputFormat {
//     fn from(value: &'static str) -> Self {
//         match value {
//             "json" => OutputFormat::Json,
//             "yaml" => OutputFormat::Yaml,
//             _ => panic!("不支持的格式:{}", value),
//         }
//     }
// }

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }

    // fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //     match self {
    //         OutputFormat::Json => write!(f, "json"),
    //         OutputFormat::Yaml => write!(f, "yaml"),
    //     }
    // }
}
