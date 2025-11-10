//! Core data transformation logic.
//!
//! This module handles the "plumbing" of reading from a source format
//! into a universal intermediate representation (`serde_json::Value`)
//! and writing from that representation to a target format.

use crate::cli::Format;
use crate::error::TransforError;
use serde_json::{Value, json};
use std::io::{Read, Write};

/// Reads data from a reader and deserializes it into `serde_json::Value`.
///
/// This function acts as the "read" half of the pipeline.
///
/// # Arguments
/// * `r` - A mutable reference to a type implementing `Read` (e.g., a file or stdin).
/// * `format` - The `Format` enum variant specifying what to decode.
///
/// # Returns
/// A `Result` containing the `serde_json::Value` on success, or a
/// `TransforError` on failure.
pub fn read_data(mut r: impl Read, format: Format) -> Result<Value, TransforError> {
    // Read the entire input into a string.
    let mut content = String::new();
    r.read_to_string(&mut content)?;
    if content.is_empty() {
        return Err(TransforError::EmptyInput);
    }

    // Deserialize from the specified format into `Value`
    let data: Value = match format {
        Format::Json => serde_json::from_str(&content)?,
        Format::Toml => toml::from_str(&content)?,
        Format::Csv => {
            // Special handling for CSV. We convert it into an array of objects.
            let mut rdr = csv::Reader::from_reader(content.as_bytes());
            let headers = rdr.headers()?.clone();
            let mut records = Vec::new();

            for result in rdr.records() {
                let record = result?;
                let mut map = serde_json::Map::new();
                for (header, field) in headers.iter().zip(record.iter()) {
                    // For MVP, we treat all CSV fields as strings.
                    // A more advanced version might try to infer types.
                    map.insert(header.to_string(), Value::String(field.to_string()));
                }
                records.push(Value::Object(map));
            }
            Value::Array(records)
        }
    };
    Ok(data)
}

/// Serializes `serde_json::Value` and writes it to a writer.
///
/// This function acts as the "write" half of the pipeline.
///
/// # Arguments
/// * `w` - A mutrable reference to a type implementing `Write` (e.g., a file or stdout).
/// * `data` - A reference to the `serde_json::Value` to serialize.
/// * `format` - The `Format` enum variant specifying what to encode.
///
/// # Returns
/// A `Result` indicating success or containing a `TransforError` on failure.
pub fn write_data(mut w: impl Write, data: &Value, format: Format) -> Result<(), TransforError> {
    match format {
        Format::Json => {
            let s = serde_json::to_string_pretty(data)?;
            w.write_all(s.as_bytes())?;
        }
        Format::Toml => {
            let s = if data.is_array() {
                let wrapped_data = json!({"data":data});
                toml::to_string(&wrapped_data)?
            } else {
                toml::to_string(data)?
            };
            w.write_all(s.as_bytes())?;
        }
        Format::Csv => {
            // Special handling for CSV. We expect an array of objects.
            if let Value::Array(records) = data {
                let mut wtr = csv::Writer::from_writer(w);

                if records.is_empty() {
                    // Handle empty array: write nothing, not even headers.
                    return Ok(());
                }

                // Get headers from the first object
                let headers: Vec<String> = if let Some(Value::Object(first)) = records.get(0) {
                    let mut h: Vec<String> = first.keys().cloned().collect();
                    h.sort(); // Ensure consistent column order
                    h
                } else {
                    // The array contains non-objects
                    return Err(TransforError::NonTabularForCsv);
                };

                // Write header record
                wtr.write_record(&headers)?;

                let mut rows: Vec<Vec<String>> = Vec::new();

                // Convert all records to rows
                for record in records {
                    if let Value::Object(map) = record {
                        let mut row: Vec<String> = Vec::with_capacity(headers.len());
                        for header in &headers {
                            // Get the value for this header.
                            // Convert JSON values (Null, Number, Bool, etc.) to strings.
                            let val_str = match map.get(header).unwrap_or(&Value::Null) {
                                Value::String(s) => s.clone(),
                                Value::Number(n) => n.to_string(),
                                Value::Bool(b) => b.to_string(),
                                Value::Null => String::new(),
                                // For complex types, serialize them as JSON strings.
                                v => serde_json::to_string(v).unwrap_or_default(),
                            };
                            row.push(val_str);
                        }
                        rows.push(row);
                    } else {
                        // The array contains a mix of objects and non-objects
                        return Err(TransforError::NonTabularForCsv);
                    }
                }

                // Write all data rows
                for row in rows {
                    wtr.write_record(&row)?;
                }
                wtr.flush()?;
            } else {
                // The root JSON value was not an array
                return Err(TransforError::NonTabularForCsv);
            }
        }
    }
    Ok(())
}

// --- Unit Tests ---

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // --- CSV to Other ---

    #[test]
    fn test_csv_to_json() {
        let input = "id,name\n1,alice\n2,bob";
        let expected = json!([
          {"id": "1", "name": "alice"},
          {"id": "2", "name": "bob"}
        ]);

        let data = read_data(input.as_bytes(), Format::Csv).unwrap();
        assert_eq!(data, expected);

        let mut output = Vec::new();
        write_data(&mut output, &data, Format::Json).unwrap();
        let output_data: Value = serde_json::from_slice(&output).unwrap();
        assert_eq!(output_data, expected);
    }

    #[test]
    fn test_csv_toml() {
        let input = "id,name\n1,alice";
        let data = read_data(input.as_bytes(), Format::Csv).unwrap();

        // TOML can't represent a top-level array without a key.
        // `toml::to_string` will wrap it.
        let mut output = Vec::new();
        write_data(&mut output, &data, Format::Toml).unwrap();
        let output_str = String::from_utf8(output).unwrap();

        assert!(output_str.contains("id = \"1\""));
        assert!(output_str.contains("name = \"alice\""));
    }

    // --- JSON to Other ---

    #[test]
    fn test_json_to_csv() {
        let input = json!([
          {"id":"1","name":"alice","extra":"foo"},
          {"id":"2","name":"bob","extra":"bar"}
        ]);
        let input_bytes = serde_json::to_string(&input).unwrap();
        let expected = "extra,id,name\nfoo,1,alice\nbar,2,bob\n";

        let data = read_data(input_bytes.as_bytes(), Format::Json).unwrap();

        let mut output = Vec::new();
        write_data(&mut output, &data, Format::Csv).unwrap();
        let output_str = String::from_utf8(output).unwrap();

        assert_eq!(output_str, expected);
    }

    #[test]
    fn test_json_to_toml() {
        let input = json!({"user":{"id":1,"name":"alice"}});
        let expected = "[user]\nid = 1\nname = \"alice\"\n";
        let input_bytes = serde_json::to_string(&input).unwrap();

        let data = read_data(input_bytes.as_bytes(), Format::Json).unwrap();

        let mut output = Vec::new();
        write_data(&mut output, &data, Format::Toml).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        assert_eq!(output_str, expected);
    }

    // --- TOML to Other ---

    #[test]
    fn test_toml_to_json() {
        let input = "[user]\nid = 1\nname = \"alice\"\n";
        let expected = json!({"user":{"id":1,"name":"alice"}});

        let data = read_data(input.as_bytes(), Format::Toml).unwrap();
        assert_eq!(data, expected);
    }

    #[test]
    fn test_toml_to_csv() {
        // TOML tables are often represented as `[[table_name]]` for an array
        let input = "[[users]]\nid = 1\nname = \"alice\"\n\n[[users]]\nid = 2\nname = \"bob\"\n";
        let expected = "id,name\n1,alice\n2,bob\n";

        let data = read_data(input.as_bytes(), Format::Toml).unwrap();

        // `read_data` deserializes to `Value`, which would be:
        // {"users": [{"id": 1, "name": "alice"}, {"id": 2, "name": "bob"}]}
        // This is NOT tabular. `write_data` will fail.

        // This test demonstrated the limitation.
        let write_result = write_data(&mut Vec::new(), &data, Format::Csv);
        assert!(matches!(write_result, Err(TransforError::NonTabularForCsv)));

        // A *valid* TOML for CSV output would need to be converted first.
        // But if the TOML represented by a `Value` that *was* tabular...
        let tabular_data = json!([{"id": 1, "name": "alice"}, {"id": 2, "name": "bob"}]);
        let mut output = Vec::new();
        write_data(&mut output, &tabular_data, Format::Csv).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        assert_eq!(output_str, expected);
    }

    // --- Error Cases ---
    #[test]
    fn test_non_tabular_to_csv_fails() {
        let input = json!({"id":1, "name":"alice"}); // An object, not an array
        let input_bytes = serde_json::to_string(&input).unwrap();

        let data = read_data(input_bytes.as_bytes(), Format::Json).unwrap();
        let result = write_data(&mut Vec::new(), &data, Format::Csv);

        assert!(matches!(result, Err(TransforError::NonTabularForCsv)));
    }

    #[test]
    fn test_mixed_array_to_csv_fails() {
        let input = json!([{"id": 1}, "string", {"id": 2}]); // Array, but not all objects
        let input_bytes = serde_json::to_string(&input).unwrap();

        let data = read_data(input_bytes.as_bytes(), Format::Json).unwrap();
        let result = write_data(&mut Vec::new(), &data, Format::Csv);

        assert!(matches!(result, Err(TransforError::NonTabularForCsv)));
    }
}
