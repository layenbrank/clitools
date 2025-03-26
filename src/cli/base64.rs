use core::fmt;
use std::str::FromStr;

use clap::Parser;

use super::verify_input_file;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "base64编码")]
    Encode(EncodeOpts),

    #[command(name = "decode", about = "base64解码")]
    Decode(DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct EncodeOpts {
    #[arg(short,long,value_parser=verify_input_file,default_value="-")]
    pub input: String,

    // #[arg(short, long)]
    // pub output: Option<String>,
    #[arg(short,long,value_parser=parse_format,default_value="Standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct DecodeOpts {
    #[arg(short,long,value_parser=verify_input_file,default_value="-")]
    pub input: String,

    // #[arg(short, long)]
    // pub output: Option<String>,
    #[arg(short,long,value_parser=parse_format,default_value="Standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    Urlsafe,
}

fn parse_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Standard" => Ok(Base64Format::Standard),
            "Urlsafe" => Ok(Base64Format::Urlsafe),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "Standard",
            Base64Format::Urlsafe => "Urlsafe",
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Base64Format::Standard => write!(f, "Standard"),
            Base64Format::Urlsafe => write!(f, "Urlsafe"),
        }
    }
}
