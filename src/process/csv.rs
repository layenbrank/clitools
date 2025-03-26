use std::fs;

use anyhow::Result;

use csv::Reader;
use serde_json::Value;

use crate::cli::OutputFormat;

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let headers = reader.headers()?.clone();
    let mut result = Vec::with_capacity(128);
    for records in reader.records() {
        let record = records?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        result.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&result)?,
        OutputFormat::Yaml => serde_yaml::to_string(&result)?,
    };
    // let json = serde_json::to_string_pretty(&result)?;
    fs::write(output, content)?;

    // println!("{:?}", records);
    Ok(())
    // let records=reader.deserialize().map(|record|record.unwrap()).collect::<Vec<Record>>();
}
