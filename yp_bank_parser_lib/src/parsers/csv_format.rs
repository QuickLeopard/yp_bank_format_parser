use std::io::{BufRead, Read, Write};
//use std::str::FromStr;

use crate::parsers::error::ParserError;
use crate::parsers::types::YPBankRecord;

const PROPER_HEADER: &str =
    "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION";

/// Parser for YPBank CSV format files.
pub struct YPBankCsvParser;

impl YPBankCsvParser {
    /// Validates that the CSV header matches the expected format.
    ///
    /// # Arguments
    ///
    /// * `header` - The header line to validate
    ///
    /// # Returns
    ///
    /// Returns true if the header matches the expected CSV format, false otherwise.
    fn check_header(header: &str) -> bool {
        let expected_header = PROPER_HEADER;
        header.trim().to_uppercase() == expected_header
    }

    /// Reads YPBank records from a CSV format reader.
    ///
    /// Validates the CSV header and parses each line as a YPBankRecord.
    ///
    /// # Arguments
    ///
    /// * `reader` - A reader implementing Read + BufRead traits
    ///
    /// # Returns
    ///
    /// Returns a Result containing a Vec<YPBankRecord> on success, or ParserError on failure.
    ///
    /// # Errors
    ///
    /// Returns ParserError::WrongCsvHeader if the header is invalid,
    /// ParserError::ParseError for empty files or parsing failures.
    pub fn from_read<R: Read + BufRead>(reader: R) -> Result<Vec<YPBankRecord>, ParserError> {
        let mut lines = reader.lines();

        let header = match lines.next() {
            Some(Ok(h)) => h,
            Some(Err(e)) => return Err(ParserError::ParseError(e.to_string())),
            None => return Err(ParserError::ParseError("Empty file".to_string())),
        };

        if !Self::check_header(&header) {
            return Err(ParserError::WrongCsvHeader(header.to_string()));
        }

        let mut records = Vec::new();
        for line in lines {
            let line = match line {
                Err(_) => {
                    eprintln!("Error reading line");
                    continue;
                }
                Ok(l) => l,
            };
            let record = YPBankRecord::from_string(&line)?;
            records.push(record);
        }
        Ok(records)
    }

    /// Writes YPBank records to a writer in CSV format.
    ///
    /// Writes the CSV header followed by each record as a CSV line.
    ///
    /// # Arguments
    ///
    /// * `writer` - A writer implementing the Write trait
    /// * `records` - Slice of YPBankRecord to write
    ///
    /// # Returns
    ///
    /// Returns a Result with () on success, or ParserError on failure.
    ///
    /// # Errors
    ///
    /// Returns ParserError::ParseError if no records are provided or writing fails.
    pub fn write_to<W: Write>(mut writer: W, records: &[YPBankRecord]) -> Result<(), ParserError> {
        if records.is_empty() {
            return Err(ParserError::ParseError("No records to write".to_string()));
        }

        let header = format!("{}\n", PROPER_HEADER);
        writer
            .write_all(header.as_bytes())
            .map_err(|e| ParserError::ParseError(format!("Failed to write header: {}", e)))?;

        for record in records {
            record
                .write_to(&mut writer)
                .map_err(ParserError::ParseError)?;
        }

        Ok(())
    }
}

impl YPBankRecord {
    /// Reads YPBank records from a CSV reader (alternative method).
    ///
    /// # Arguments
    ///
    /// * `reader` - A reader implementing Read + BufRead traits
    ///
    /// # Returns
    ///
    /// Returns a Result containing a Vec<YPBankRecord> on success, or ParserError on failure.
    pub fn from_read<R: Read + BufRead>(reader: R) -> Result<Vec<YPBankRecord>, ParserError> {
        // Implementation for reading CSV and parsing into YPBankCsvRecord structs

        let mut records = Vec::new();
        for line in reader.lines() {
            let line = match line {
                Err(_) => {
                    eprintln!("Error reading line");
                    continue;
                }
                Ok(l) => l,
            };
            let record = Self::from_string(&line)?;
            records.push(record);
        }

        Ok(records)
    }

    /// Parses a YPBankRecord from a CSV string.
    ///
    /// # Arguments
    ///
    /// * `s` - CSV string containing record data
    ///
    /// # Returns
    ///
    /// Returns a Result containing a YPBankRecord on success, or ParserError on failure.
    ///
    /// # Errors
    ///
    /// Returns ParserError::ParseError if the string format is invalid or parsing fails.
    pub fn from_string(s: &str) -> Result<Self, ParserError> {
        let mut parts = Vec::new();
        let mut current_field = String::new();
        let mut in_quotes = false;
        let mut chars = s.chars().peekable();
        
        while let Some(ch) = chars.next() {
            match ch {
                '"' => {
                    in_quotes = !in_quotes;
                }
                ',' if !in_quotes => {
                    parts.push(current_field.clone());
                    current_field.clear();
                }
                _ => {
                    current_field.push(ch);
                }
            }
        }
        parts.push(current_field);
        
        if parts.len() == 8 {
            Ok(YPBankRecord {
                tx_id: parts[0].parse().map_err(|e| {
                    ParserError::ParseError(format!(
                        "Failed to parse tx_id: {} error: {}",
                        parts[0], e
                    ))
                })?,
                tx_type: parts[1].parse().map_err(|e| {
                    ParserError::ParseError(format!(
                        "Failed to parse tx_type: {} error: {}",
                        parts[1], e
                    ))
                })?,
                from_user_id: parts[2].parse().map_err(|e| {
                    ParserError::ParseError(format!(
                        "Failed to parse from_user_id: {} error: {}",
                        parts[2], e
                    ))
                })?,
                to_user_id: parts[3].parse().map_err(|e| {
                    ParserError::ParseError(format!(
                        "Failed to parse to_user_id: {} error: {}",
                        parts[3], e
                    ))
                })?,
                amount: parts[4].parse().map_err(|e| {
                    ParserError::ParseError(format!(
                        "Failed to parse amount: {} error: {}",
                        parts[4], e
                    ))
                })?,
                timestamp: parts[5].parse().map_err(|e| {
                    ParserError::ParseError(format!(
                        "Failed to parse timestamp: {} error: {}",
                        parts[5], e
                    ))
                })?,
                status: parts[6].parse().map_err(|e| {
                    ParserError::ParseError(format!(
                        "Failed to parse status: {} error: {}",
                        parts[6], e
                    ))
                })?,
                description: parts[7].to_string(),
            })
        } else {
            eprintln!("Invalid record, expect 8 fields, got: {}", s);
            Err(ParserError::ParseError(format!(
                "Invalid record, expect 8 fields, got: {}",
                s
            )))
        }
    }

    /// Writes this YPBankRecord to a writer in CSV format.
    ///
    /// # Arguments
    ///
    /// * `writer` - A mutable reference to a writer implementing Write trait
    ///
    /// # Returns
    ///
    /// Returns a Result with () on success, or String error message on failure.
    pub fn write_to<W: Write>(&self, writer: &mut W) -> Result<(), String> {
        let escaped_description = if self.description.contains(',') || self.description.contains(':') {
            format!("\"{}\"", self.description)
        } else {
            self.description.clone()
        };
        
        let record = format!(
            "{},{:?},{},{},{},{},{:?},{}\n",
            self.tx_id,
            self.tx_type,
            self.from_user_id,
            self.to_user_id,
            self.amount,
            self.timestamp,
            self.status,
            escaped_description
        );
        writer
            .write_all(record.as_bytes())
            .map_err(|e| format!("Failed to write record: {}", e))?;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::types::{TransactionType, Status};
    use std::io::Cursor;

    #[test]
    fn test_csv_parse_description_with_comma() {
        let csv_line = "123,Deposit,456,789,1000,1640995200,Success,\"Payment, with comma\"";
        let record = YPBankRecord::from_string(csv_line).unwrap();
        assert_eq!(record.description, "Payment, with comma");
    }

    #[test]
    fn test_csv_parse_description_with_colon() {
        let csv_line = "123,Deposit,456,789,1000,1640995200,Success,\"Payment: with colon\"";
        let record = YPBankRecord::from_string(csv_line).unwrap();
        assert_eq!(record.description, "Payment: with colon");
    }

    #[test]
    fn test_csv_parse_description_with_comma_and_colon() {
        let csv_line = "123,Deposit,456,789,1000,1640995200,Success,\"Payment, with: both\"";
        let record = YPBankRecord::from_string(csv_line).unwrap();
        assert_eq!(record.description, "Payment, with: both");
    }

    #[test]
    fn test_csv_write_description_with_comma() {
        let record = YPBankRecord {
            tx_id: 123,
            tx_type: TransactionType::Deposit,
            from_user_id: 456,
            to_user_id: 789,
            amount: 1000,
            timestamp: 1640995200,
            status: Status::Success,
            description: "Payment, with comma".to_string(),
        };
        
        let mut output = Vec::new();
        record.write_to(&mut output).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("\"Payment, with comma\""));
    }

    #[test]
    fn test_csv_write_description_with_colon() {
        let record = YPBankRecord {
            tx_id: 123,
            tx_type: TransactionType::Deposit,
            from_user_id: 456,
            to_user_id: 789,
            amount: 1000,
            timestamp: 1640995200,
            status: Status::Success,
            description: "Payment: with colon".to_string(),
        };
        
        let mut output = Vec::new();
        record.write_to(&mut output).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("\"Payment: with colon\""));
    }

    #[test]
    fn test_csv_roundtrip_with_special_chars() {
        let original_record = YPBankRecord {
            tx_id: 123,
            tx_type: TransactionType::Transfer,
            from_user_id: 456,
            to_user_id: 789,
            amount: 1000,
            timestamp: 1640995200,
            status: Status::Success,
            description: "Transfer, from: account A".to_string(),
        };
        
        let mut output = Vec::new();
        original_record.write_to(&mut output).unwrap();
        let csv_line = String::from_utf8(output).unwrap();
        
        let parsed_record = YPBankRecord::from_string(csv_line.trim()).unwrap();
        assert_eq!(original_record.description, parsed_record.description);
    }
}