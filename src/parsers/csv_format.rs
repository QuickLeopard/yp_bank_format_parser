

use std::io;
use std::io::Read;
use std::io::BufRead;

use crate::parsers::types::{TransactionType, Status};

#[derive(Debug)]
pub struct YPBankCsvRecord {
    tx_id: u64,
    tx_type: TransactionType,
    from_user_id: u64,
    to_user_id: u64,
    amount: u64,
    timestamp: String,
    status: Status,
    description: String,
}

impl YPBankCsvRecord {
    pub fn from_read<R: Read + BufRead>(reader: R) -> Vec<YPBankCsvRecord> {
        // Implementation for reading CSV and parsing into YPBankCsvRecord structs

        let mut records = Vec::new();    
        for line in reader.lines() {
            let line = match line {
                Err(_) => {eprintln!("Error reading line"); continue;},
                Ok(l) => l,
            };
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 8 {
                records.push(YPBankCsvRecord {
                    tx_id: parts[0].parse().unwrap_or(0),
                    tx_type: parts[1].parse().unwrap_or(TransactionType::DEPOSIT),
                    from_user_id: parts[2].parse().unwrap_or(0),
                    to_user_id: parts[3].parse().unwrap_or(0),
                    amount: parts[4].parse().unwrap_or(0),
                    timestamp: parts[5].to_string(),
                    status: parts[6].parse ().unwrap_or(Status::FAILURE),
                    description: parts[7].to_string(),
                });
            }
            else {
                eprintln!("Invalid record: {}", line);
            }
        }

        records
    }

    pub fn write_to<W: std::io::Write>(&mut self, writer: &mut W) -> Result<(), String> {
        Ok (())
    }
}