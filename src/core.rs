//! WASM CORE
//! This module contains the core functionality of the WASM module.
//! It is responsible for converting CSV data to JSON.
//! It is written in Rust and compiled to WASM.
//! It is used by the WASM module and the browser module.


// Import
use serde_json::{Value, to_string};
use csv::Reader;
use std::str;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum CsvToJsonError {
    Utf8Error(std::str::Utf8Error),
    CsvError(csv::Error),
    SerdeJsonError(serde_json::Error),
}

impl fmt::Display for CsvToJsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CsvToJsonError::Utf8Error(ref e) => write!(f, "UTF-8 encoding error: {}", e),
            CsvToJsonError::CsvError(ref e) => write!(f, "CSV processing error: {}", e),
            CsvToJsonError::SerdeJsonError(ref e) => write!(f, "JSON serialization error: {}", e),
        }
    }
}

impl Error for CsvToJsonError {}
impl From<std::str::Utf8Error> for CsvToJsonError {
    fn from(err: std::str::Utf8Error) -> CsvToJsonError {
        CsvToJsonError::Utf8Error(err)
    }
}

impl From<csv::Error> for CsvToJsonError {
    fn from(err: csv::Error) -> CsvToJsonError {
        CsvToJsonError::CsvError(err)
    }
}

impl From<serde_json::Error> for CsvToJsonError {
    fn from(err: serde_json::Error) -> CsvToJsonError {
        CsvToJsonError::SerdeJsonError(err)
    }
}

/// Convert CSV to JSON
pub fn csv_to_json(data: Vec<u8>) -> Result<String, CsvToJsonError> {
    let csv_str = str::from_utf8(&data).map_err(CsvToJsonError::Utf8Error)?;
    let mut reader = Reader::from_reader(csv_str.as_bytes());
    let headers = reader.headers().map_err(CsvToJsonError::CsvError)?.clone();

    let mut records = Vec::new();

    for result in reader.records() {
        let record = result.map_err(CsvToJsonError::CsvError)?;
        let mut json_item = serde_json::Map::new();

        for (header, field) in headers.iter().zip(record.iter()) {
            json_item.insert(header.to_string(), Value::String(field.to_owned()));
        }
        records.push(Value::Object(json_item));
    }

    to_string(&records).map_err(CsvToJsonError::SerdeJsonError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_to_json_valid_input() {
        let csv_data = b"Name,Age,Location\nAlice,30,NY\nBob,40,CA";
        let expected_output = r#"[{"Age":"30","Location":"NY","Name":"Alice"},{"Age":"40","Location":"CA","Name":"Bob"}]"#;

        let output = csv_to_json(csv_data.to_vec()).unwrap();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_csv_to_json_invalid_utf8() {
        let csv_data = vec![0, 255, 254];

        match csv_to_json(csv_data) {
            Ok(_) => panic!("Expected an error due to invalid UTF-8 input"),
            Err(e) => assert!(matches!(e, CsvToJsonError::Utf8Error(_))),
        }
    }
}
