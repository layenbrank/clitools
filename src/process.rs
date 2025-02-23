use std::fs;

use anyhow::Result;

use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    name: String,

    position: String,

    #[serde(rename = "DOB")]
    dob: String,

    nationality: String,

    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut result = Vec::with_capacity(128);
    for records in reader.deserialize::<Record>() {
        let record = records?;
        result.push(record);
        // println!("{:?}", record)
    }
    let json = serde_json::to_string_pretty(&result)?;
    fs::write(output, json)?;

    // println!("{:?}", records);
    Ok(())
    // let records=reader.deserialize().map(|record|record.unwrap()).collect::<Vec<Record>>();
}
