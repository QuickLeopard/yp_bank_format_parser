use std::collections::HashMap;
use std::io::BufRead;
use std::io::{Read, Write};
//use std::str::FromStr;

use crate::parsers::error::ParserError;
use crate::parsers::types::YPBankRecord;

pub struct YPBankTxtParser;

impl YPBankTxtParser {
    fn read_sections<R: Read + BufRead>(reader: R) -> Result<Vec<Vec<String>>, ParserError> {
        let mut sections = Vec::new();
        let mut current_section = Vec::new();

        for line_result in reader.lines() {
            let line = line_result.map_err(|e| ParserError::ParseError(e.to_string()))?;

            // Check if this line starts with "#"
            if line.starts_with('#') {
                // If we have accumulated lines in the current section, save it
                if !current_section.is_empty() {
                    sections.push(current_section);
                    current_section = Vec::new();
                }
                // Skip the delimiter line itself - don't add it to any section
                continue;
            }

            // Skip empty lines if desired, or keep them
            if !line.trim().is_empty() {
                current_section.push(line);
            }
        }

        // Don't forget to add the last section if it has content
        if !current_section.is_empty() {
            sections.push(current_section);
        }

        Ok(sections)
    }

    fn parse_sections(
        sections: Vec<Vec<String>>,
    ) -> Result<Vec<HashMap<String, String>>, ParserError> {
        sections
            .into_iter()
            .map(|section| {
                let mut dict = HashMap::new();
                for line in section {
                    let parts: Vec<&str> = line.splitn(2, ':').collect();
                    if parts.len() == 2 {
                        dict.insert(
                            parts[0].trim().to_lowercase().to_string(),
                            parts[1].trim().to_string(),
                        );
                    } else {
                        return Err(ParserError::ParseError(format!(
                            "Invalid line format: {}",
                            line
                        )));
                    }
                }
                Ok(dict)
            })
            .collect()
    }

    pub fn from_read<R: Read + BufRead>(reader: R) -> Result<Vec<YPBankRecord>, ParserError> {
        let sections = Self::read_sections(reader)?;

        let dict = Self::parse_sections(sections)?;

        //let mut lines = reader.lines();

        //let splitted = lines.sp

        let records = dict
            .into_iter()
            .map(|d| {
                Ok(YPBankRecord {
                    tx_id: d
                        .get("tx_id")
                        .ok_or_else(|| ParserError::MissingTxId)?
                        .parse()
                        .map_err(|e| {
                            ParserError::ParseError(format!(
                                "Failed to parse tx_id: {} error: {}",
                                d.get("tx_id").unwrap(),
                                e
                            ))
                        })?,
                    tx_type: d
                        .get("tx_type")
                        .ok_or_else(|| ParserError::MissingTransactionType)?
                        .parse()
                        .map_err(|e| {
                            ParserError::ParseError(format!(
                                "Failed to parse tx_type: {} error: {}",
                                d.get("tx_type").unwrap(),
                                e
                            ))
                        })?,
                    from_user_id: d
                        .get("from_user_id")
                        .ok_or_else(|| ParserError::MissingFromUserId)?
                        .parse()
                        .map_err(|e| {
                            ParserError::ParseError(format!(
                                "Failed to parse from_user_id: {} error: {}",
                                d.get("from_user_id").unwrap(),
                                e
                            ))
                        })?,
                    to_user_id: d
                        .get("to_user_id")
                        .ok_or_else(|| ParserError::MissingToUserId)?
                        .parse()
                        .map_err(|e| {
                            ParserError::ParseError(format!(
                                "Failed to parse to_user_id: {} error: {}",
                                d.get("to_user_id").unwrap(),
                                e
                            ))
                        })?,
                    amount: d
                        .get("amount")
                        .ok_or_else(|| ParserError::MissingAmount)?
                        .parse()
                        .map_err(|e| {
                            ParserError::ParseError(format!(
                                "Failed to parse amount: {} error: {}",
                                d.get("amount").unwrap(),
                                e
                            ))
                        })?,
                    timestamp: d
                        .get("timestamp")
                        .ok_or_else(|| ParserError::MissingTimestamp)?
                        .parse()
                        .map_err(|e| {
                            ParserError::ParseError(format!(
                                "Failed to parse timestamp: {} error: {}",
                                d.get("timestamp").unwrap(),
                                e
                            ))
                        })?,
                    status: d
                        .get("status")
                        .ok_or_else(|| ParserError::MissingStatus)?
                        .parse()
                        .map_err(|e| {
                            ParserError::WrongStatusType(d.get("status").unwrap().parse().unwrap()) /*(format!(
                            "Failed to parse status: {} error: {}",
                            d.get("status").unwrap(),
                            e
                            ))*/
                        })?,
                    description: d
                        .get("description")
                        .ok_or_else(|| ParserError::MissingDescription)?
                        .to_string(),
                })
            })
            .collect::<Result<Vec<_>, ParserError>>()?;

        Ok(records)
    }

    pub fn write_to<W: Write>(mut writer: W, records: &[YPBankRecord]) -> Result<(), ParserError> {
        for record in records.iter().enumerate() {
            writeln!(writer, "# Record {} ({:?})", record.0, record.1.tx_type);
            writeln!(writer, "tx_id: {}", record.1.tx_id);
            writeln!(writer, "tx_type: {:?}", record.1.tx_type);
            writeln!(writer, "from_user_id: {}", record.1.from_user_id);
            writeln!(writer, "to_user_id: {}", record.1.to_user_id);
            writeln!(writer, "amount: {}", record.1.amount);
            writeln!(writer, "timestamp: {}", record.1.timestamp);
            writeln!(writer, "status: {:?}", record.1.status);
            writeln!(writer, "description: {}", record.1.description);
            if record.0 < records.len() - 1 {
                writeln!(writer);
            }
        }
        Ok(())
    }
}
