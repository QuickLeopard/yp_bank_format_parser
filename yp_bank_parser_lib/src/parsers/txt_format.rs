use std::collections::HashMap;
use std::io::BufRead;
use std::io::{Read, Write};

use crate::parsers::error::ParserError;
use crate::parsers::parser;
use crate::parsers::types::{YPBankRecord, TransactionType, Status};

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

        fn parse_helper<T>(d: &HashMap<String, String>, key: &str, error: ParserError) -> Result<T, ParserError>
        where
            T: std::str::FromStr,
            T::Err: std::fmt::Display,
        {
            let tx_str = d.get(key).ok_or(error)?;
            tx_str.parse::<T>().map_err(|e| {
                ParserError::ParseError(format!(
                    "Failed to parse {}: {} error: {}",
                    key,
                    tx_str,
                    e
                ))
            })
        }

        let records = dict
            .into_iter()
            .map(|d| {
                Ok(YPBankRecord {
                    tx_id: parse_helper::<u64>(&d, "tx_id", ParserError::MissingTxId)?,                     
                                           
                    tx_type: parse_helper::<TransactionType>(&d, "tx_type", ParserError::MissingTransactionType)?,                         
                                           
                    from_user_id: parse_helper::<u64>(&d, "from_user_id", ParserError::MissingFromUserId)?,
                        
                    to_user_id: parse_helper::<u64>(&d, "to_user_id", ParserError::MissingToUserId)?,
                       
                    amount: parse_helper::<i64>(&d, "amount", ParserError::MissingAmount)?,
                                                              
                    timestamp: parse_helper::<u64>(&d, "timestamp", ParserError::MissingTimestamp)?,
                                            
                    status: parse_helper::<Status>(&d, "status", ParserError::MissingStatus)?,
                                            
                    description: parse_helper::<String>(&d, "description", ParserError::MissingDescription)?,                        
                })
            })
            .collect::<Result<Vec<_>, ParserError>>()?;

        Ok(records)
    }

    pub fn write_to<W: Write>(mut writer: W, records: &[YPBankRecord]) -> Result<(), ParserError> {
        for record in records.iter().enumerate() {
            writeln!(writer, "# Record {} ({:?})", record.0, record.1.tx_type)?;
            writeln!(writer, "tx_id: {}", record.1.tx_id)?;
            writeln!(writer, "tx_type: {:?}", record.1.tx_type)?;
            writeln!(writer, "from_user_id: {}", record.1.from_user_id)?;
            writeln!(writer, "to_user_id: {}", record.1.to_user_id)?;
            writeln!(writer, "amount: {}", record.1.amount)?;
            writeln!(writer, "timestamp: {}", record.1.timestamp)?;
            writeln!(writer, "status: {:?}", record.1.status)?;
            writeln!(writer, "description: {}", record.1.description)?;
            if record.0 < records.len() - 1 {
                writeln!(writer)?;
            }
        }
        Ok(())
    }
}
