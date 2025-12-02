
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, BufRead, Write, Seek, SeekFrom};

use crate::parsers::error::ParserError;
use crate::parsers::types::{Status, TransactionType, YPBankCsvRecord};

const MAGIC_HEADER: u32 = 0x5950424E; // 'YPBN' in ASCII

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
                Ok(x) => {
                        return Err(ParserError::ParseError(format! ("Incomplete magic header: {} bytes, value: {}", x, &buffer[0])));
                        //eprintln! ("Incomplete magic header");
                    },
                Err(e) => return Err(ParserError::ParseError (e.to_string())),
            }
            
            // Read RECORD_SIZE
            let record_size = reader.read_u32::<BigEndian>()?;
            
            // Parse record body
            let tx_id = reader.read_u64::<BigEndian>()?;

            println! ("tx_id: {}", tx_id);

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

            let record = YPBankCsvRecord {
                tx_id,
                tx_type,
                from_user_id,
                to_user_id,
                amount,
                timestamp: format! ("{}", timestamp),
                status,
                description,
            };
            
            records.push(record.clone ());

            println!("");
            println! ("Parsed: {} records, record: {:?}", records.len(), record);
        }       

        Ok (records)
    }
    pub fn write_to<W: Write>(
        mut writer: W,
        records: &[YPBankCsvRecord],
    ) -> Result<(), ParserError> {
        if records.is_empty() {
            return Err(ParserError::ParseError("No records to write".to_string()));
        }

        for record in records.iter () { //}.take (1) {
            // Write magic header for each record
            writer.write_all(&MAGIC_HEADER.to_be_bytes())?;
            
            // Calculate and write record size
            let desc_bytes = record.description.as_bytes();
            let record_size = 8 + 1 + 8 + 8 + 8 + 8 + 1 + 4 + desc_bytes.len() as u32;
            writer.write_u32::<BigEndian>(record_size)?;

            // Write record fields
            writer.write_u64::<BigEndian>(record.tx_id)?;
            writer.write_u8(record.tx_type as u8)?;
            writer.write_u64::<BigEndian>(record.from_user_id)?;
            writer.write_u64::<BigEndian>(record.to_user_id)?;
            writer.write_i64::<BigEndian>(record.amount)?;
            writer.write_u64::<BigEndian>(record.timestamp.parse::<u64>().unwrap_or(0))?;
            writer.write_u8(record.status as u8)?;

            // Write description
            writer.write_u32::<BigEndian>(desc_bytes.len() as u32)?;
            writer.write_all(desc_bytes)?;

        

        }
     
        Ok(())
    
    }
     

}