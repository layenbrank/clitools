use anyhow::{Ok, Result};
// use base64::{prelude::{BASE64_STANDARD,BASE64_URL_SAFE_NO_PAD}, Engine};
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
use std::{fs::File, io::Read};

use crate::Base64Format;
pub fn process_encode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buffer),
        Base64Format::Urlsafe => URL_SAFE_NO_PAD.encode(&buffer),
        // Base64Format::Base64Mime => BASE64_MIME.encode(buffer),
    };

    println!("{}", encoded);

    Ok(())
}
pub fn process_decode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;

    let mut buffer = String::new();

    reader.read_to_string(&mut buffer)?;

    // 处理掉换行，否则报错，无法正常解码
    // avoid accidental newline
    let buffer = buffer.trim();

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buffer)?,
        Base64Format::Urlsafe => URL_SAFE_NO_PAD.decode(buffer)?,
        // Base64Format::Base64Mime => BASE64_MIME.decode(buffer),
    };

    // TODO decoded data might not be String (buffer for this example, we assume it is)
    let decoded = String::from_utf8(decoded)?;
    println!("decoded: {}", decoded);
    Ok(())
}

fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_encode(input, format).is_ok())
    }

    #[test]

    fn test_process_decode() {
        let input = "fixtures/base64.txt";
        let format = Base64Format::Standard;
        assert!(process_decode(input, format).is_ok());
    }
}
