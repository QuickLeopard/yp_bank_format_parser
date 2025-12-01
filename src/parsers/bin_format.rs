
use byteorder::{BigEndian, ReadBytesExt};
use std::io::{self, Read, BufRead, Write, Seek, SeekFrom};

use crate::parsers::error::ParserError;
use crate::parsers::types::{Status, TransactionType, YPBankCsvRecord};

pub struct YPBankBinParser;

impl YPBankBinParser {


    pub fn from_read<R: Read + BufRead>(mut reader: R) -> Result<Vec<YPBankCsvRecord>, ParserError> {
        let mut records = Vec::new();

        let mut buffer = [0u8; 8]; // For reading MAGIC and RECORD_SIZE
    
        loop {
            // Read MAGIC header
            match reader.read(&mut buffer[0..4]) {
                Ok(0) => break, // EOF
                Ok(4) => {
                    let magic = u32::from_be_bytes(buffer[0..4].try_into().unwrap());
                    if magic != 0x5950424E { // 'YPBN'
                        eprintln!("Invalid magic number: {:X}", magic);
                        break;
                    }
                }
                Ok(_) => return Err(ParserError::ParseError("Incomplete magic header".to_string())),
                Err(e) => return Err(ParserError::ParseError (e.to_string())),
            }
            
            // Read RECORD_SIZE
            let record_size = reader.read_u32::<BigEndian>()?;
            
            // Parse record body
            let tx_id = reader.read_u64::<BigEndian>()?;
            let tx_type_byte = reader.read_u8()?;
            let tx_type = TransactionType::try_from(tx_type_byte)?;
            
            let from_user_id = reader.read_u64::<BigEndian>()?;
            let to_user_id = reader.read_u64::<BigEndian>()?;
            let amount = reader.read_i64::<BigEndian>()?;
            let timestamp = reader.read_u64::<BigEndian>()?;
            
            let status_byte = reader.read_u8()?;
            let status = Status::try_from(status_byte)?;
            
            let desc_len = reader.read_u32::<BigEndian>()?;
            
            // Safety check: prevent unreasonable allocation sizes
            if desc_len > 10_000_000 { // 10MB limit
                return Err(ParserError::ParseError(format! ("Too long description: {}", desc_len)));
            }
            
            // Read description bytes
            let mut desc_bytes = vec![0u8; desc_len as usize];
            reader.read_exact(&mut desc_bytes)?;
            
            // Validate UTF-8
            let description = String::from_utf8(desc_bytes)?;
            
            records.push(YPBankCsvRecord {
                tx_id,
                tx_type,
                from_user_id,
                to_user_id,
                amount,
                timestamp: format! ("{}", timestamp),
                status,
                description,
            });
        }

        Ok (records)
    }

}