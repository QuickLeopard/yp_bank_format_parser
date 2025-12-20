
use byteorder::{BigEndian, WriteBytesExt};
use std::io::{Read, Write};

use crate::parsers::error::ParserError;
use crate::parsers::types::{Status, TransactionType, YPBankRecord};

use crate::{HEADER_SIZE, MAGIC, MIN_BODY_SIZE, MAX_RECORD_SIZE};

const MAGIC_HEADER: u32 = 0x5950424E; // 'YPBN' in ASCII

pub struct YPBankBinParser;

impl YPBankBinParser {    

    pub fn parse_bytes(data: &[u8]) -> Result<Vec<YPBankRecord>, ParserError> {
        Self::from_read(std::io::Cursor::new(data))
    }

    pub fn from_read<R: Read>(mut reader: R) -> Result<Vec<YPBankRecord>, ParserError> {
        let mut records = Vec::new();
        let mut header_buf = [0u8; HEADER_SIZE];

        loop {
            match reader.read_exact(&mut header_buf) {
                Ok(()) => {}
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(ParserError::Io(e)),
            }

            let magic: [u8; 4] = header_buf[0..4].try_into()?;
            if magic != MAGIC {
                return Err(ParserError::InvalidMagic(magic));
            }

            let record_size = u32::from_be_bytes(header_buf[4..8].try_into()?);
            
            // Validate size bounds
            if record_size < MIN_BODY_SIZE as u32 {
                return Err(ParserError::RecordTooSmall(record_size, MIN_BODY_SIZE));
            }
            
            if record_size > MAX_RECORD_SIZE as u32 {
                return Err(ParserError::RecordTooLarge(record_size, MAX_RECORD_SIZE));
            }

            // Parse record directly from reader without pre-buffering entire body
            let record = Self::parse_record_from_reader(&mut reader, record_size)?;
            records.push(record);
        }

        Ok(records)
    }

    fn parse_record_from_reader<R: Read>(reader: &mut R, record_size: u32) -> Result<YPBankRecord, ParserError> {
        // Use take() to limit reading to exactly record_size bytes
        let mut limited_reader = reader.take(record_size as u64);
        
        let mut buffer = [0u8; 8];
        
        // TX_ID: 8 bytes, u64 big-endian
        limited_reader.read_exact(&mut buffer)?;
        let tx_id = u64::from_be_bytes(buffer);
        
        // TX_TYPE: 1 byte
        let mut tx_type_buf = [0u8; 1];
        limited_reader.read_exact(&mut tx_type_buf)?;
        let tx_type = TransactionType::from_byte(tx_type_buf[0])?;
        
        // FROM_USER_ID: 8 bytes
        limited_reader.read_exact(&mut buffer)?;
        let from_user_id = u64::from_be_bytes(buffer);
        
        // TO_USER_ID: 8 bytes
        limited_reader.read_exact(&mut buffer)?;
        let to_user_id = u64::from_be_bytes(buffer);
        
        // AMOUNT: 8 bytes
        limited_reader.read_exact(&mut buffer)?;
        let amount = i64::from_be_bytes(buffer);
        
        // TIMESTAMP: 8 bytes
        limited_reader.read_exact(&mut buffer)?;
        let timestamp = u64::from_be_bytes(buffer);
        
        // STATUS: 1 byte
        let mut status_buf = [0u8; 1];
        limited_reader.read_exact(&mut status_buf)?;
        let status = Status::from_byte(status_buf[0])?;
        
        // DESC_LEN: 4 bytes
        let mut desc_len_buf = [0u8; 4];
        limited_reader.read_exact(&mut desc_len_buf)?;
        let desc_len = u32::from_be_bytes(desc_len_buf);
        
        // Read description
        if desc_len > (record_size - 42) { // 42 = sum of all fixed field sizes
            return Err(ParserError::DescriptionOverflow { 
                desc_len, 
                remaining: record_size as usize - 42 
            });
        }
        
        let mut description_bytes = vec![0u8; desc_len as usize];
        limited_reader.read_exact(&mut description_bytes)?;
        let description = String::from_utf8(description_bytes)?;
        
        // Ensure we've consumed exactly record_size bytes
        if limited_reader.limit() != 0 {
            // This should not happen if our calculations are correct
            return Err(ParserError::ParseError(format!("Did not consume all record bytes: {} remaining", limited_reader.limit())));
        }

        Ok(YPBankRecord {
            tx_id,
            tx_type,
            from_user_id,
            to_user_id,
            amount,
            timestamp,
            status,
            description,
        })
    }    

    pub fn write_to<W: Write>(mut writer: W, records: &[YPBankRecord]) -> Result<(), ParserError> {
        if records.is_empty() {
            return Err(ParserError::ParseError("No records to write".to_string()));
        }

        for record in records.iter() {
            //}.take (1) {
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
            writer.write_u64::<BigEndian>(record.timestamp)?;
            writer.write_u8(record.status as u8)?;

            // Write description
            writer.write_u32::<BigEndian>(desc_bytes.len() as u32)?;
            writer.write_all(desc_bytes)?;
        }

        Ok(())
    }
}
