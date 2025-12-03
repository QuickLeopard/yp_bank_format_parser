
use std::io::{Read, BufRead, Write};
use std::str::FromStr;

use crate::parsers::error::ParserError;
use crate::parsers::types::{Status, TransactionType, YPBankRecord};

const PROPER_HEADER: &str =
    "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION";

pub struct YPBankCsvParser;

impl YPBankCsvParser {
    fn check_header(header: &str) -> bool {
        let expected_header = PROPER_HEADER;
        header.trim().to_uppercase() == expected_header
    }

    pub fn from_read<R: Read + BufRead>(reader: R) -> Result<Vec<YPBankRecord>, ParserError> {
        let mut lines = reader.lines();

        let header = match lines.next() {
            Some(Ok(h)) => h,
            Some(Err(e)) => return Err(ParserError::ParseError(e.to_string())),
            None => return Err(ParserError::ParseError("Empty file".to_string())),
        };

        if !Self::check_header(&header) {
            return Err(ParserError::ParseError("Wrong header".to_string()));
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

    pub fn write_to<W: Write>(
        mut writer: W,
        records: &[YPBankRecord],
    ) -> Result<(), ParserError> {
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
                .map_err(|e| ParserError::ParseError(e))?;
        }

        Ok(())
    }
}

impl YPBankRecord {
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

    pub fn from_read2<R: Read + BufRead>(mut reader: R) -> Result<Self, ParserError> {
        // Implementation for reading CSV and parsing into YPBankCsvRecord structs

        let mut line = String::new();
        let _ = reader.read_line(&mut line);

        Self::from_string(&line)
    }

    pub fn from_string(s: &str) -> Result<Self, ParserError> {
        let parts: Vec<&str> = s.split(',').collect();
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
                timestamp: parts[5].parse ().map_err(|e| {
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

    pub fn write_to<W: Write>(&self, writer: &mut W) -> Result<(), String> {
        let record = format!(
            "{},{:?},{},{},{},{},{:?},{}\n",
            self.tx_id,
            self.tx_type,
            self.from_user_id,
            self.to_user_id,
            self.amount,
            self.timestamp,
            self.status,
            self.description
        );
        writer
            .write_all(record.as_bytes())
            .map_err(|e| format!("Failed to write record: {}", e))?;
        Ok(())
    }
}
