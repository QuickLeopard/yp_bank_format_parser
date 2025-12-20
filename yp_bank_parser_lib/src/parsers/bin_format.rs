
use byteorder::{BigEndian, WriteBytesExt};
use std::array::TryFromSliceError;
use std::io::{Read, Write};

use crate::parsers::error::ParserError;
use crate::parsers::types::{Status, TransactionType, YPBankRecord};

use crate::{HEADER_SIZE, MAGIC, MIN_BODY_SIZE};

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
            // Try to read the header
            match reader.read_exact(&mut header_buf) {
                Ok(()) => {}
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                    // End of file reached cleanly
                    break;
                }
                Err(e) => return Err(ParserError::Io(e)),
            }

            // Validate magic bytes
            let magic: [u8; 4] = header_buf[0..4].try_into().map_err(|e: TryFromSliceError| ParserError::ParseError(e.to_string()))?;
            if magic != MAGIC {
                return Err(ParserError::InvalidMagic(magic));
            }

            // Read record size (big-endian u32)
            let record_size = u32::from_be_bytes(header_buf[4..8].try_into().map_err(|e: TryFromSliceError| ParserError::ParseError(e.to_string()))?);

            if (record_size as usize) < MIN_BODY_SIZE {
                return Err(ParserError::RecordTooSmall(record_size, MIN_BODY_SIZE));
            }

            // Read the record body
            let mut body = vec![0u8; record_size as usize];
            reader.read_exact(&mut body)?;

            // Parse the record
            let record = Self::parse_record_body(&body)?;
            records.push(record);
        }

        Ok(records)
    }

    /// Parse a single record body from a byte slice.
    fn parse_record_body(body: &[u8]) -> Result<YPBankRecord, ParserError> {
        if body.len() < MIN_BODY_SIZE {
            return Err(ParserError::UnexpectedEof {
                expected: MIN_BODY_SIZE,
                actual: body.len(),
            });
        }

        let mut offset = 0;

        // TX_ID: 8 bytes, u64 big-endian
        let tx_id = u64::from_be_bytes(
            body[offset..offset + 8]
                .try_into()
                .map_err(|e: TryFromSliceError| ParserError::ParseError(e.to_string()))?,
        );
        offset += 8;

        // TX_TYPE: 1 byte
        let tx_type = TransactionType::from_byte(body[offset])?;
        offset += 1;

        // FROM_USER_ID: 8 bytes, u64 big-endian
        let from_user_id = u64::from_be_bytes(
            body[offset..offset + 8]
                .try_into()
                .map_err(|e: TryFromSliceError| ParserError::ParseError(e.to_string()))?,
        );
        offset += 8;

        // TO_USER_ID: 8 bytes, u64 big-endian
        let to_user_id = u64::from_be_bytes(
            body[offset..offset + 8]
                .try_into()
                .map_err(|e: TryFromSliceError| ParserError::ParseError(e.to_string()))?,
        );
        offset += 8;

        // AMOUNT: 8 bytes, i64 big-endian
        let amount = i64::from_be_bytes(
            body[offset..offset + 8]
                .try_into()
                .map_err(|e: TryFromSliceError| ParserError::ParseError(e.to_string()))?,
        );
        offset += 8;

        // TIMESTAMP: 8 bytes, u64 big-endian
        let timestamp = u64::from_be_bytes(
            body[offset..offset + 8]
                .try_into()
                .map_err(|e: TryFromSliceError| ParserError::ParseError(e.to_string()))?,
        );
        offset += 8;

        // STATUS: 1 byte
        let status = Status::from_byte(body[offset])?;
        offset += 1;

        // DESC_LEN: 4 bytes, u32 big-endian
        let desc_len = u32::from_be_bytes(
            body[offset..offset + 4]
                .try_into()
                .map_err(|e: TryFromSliceError| ParserError::ParseError(e.to_string()))?,
        );
        offset += 4;

        // DESCRIPTION: DESC_LEN bytes, UTF-8
        let remaining = body.len() - offset;
        if (desc_len as usize) > remaining {
            return Err(ParserError::DescriptionOverflow {
                desc_len,
                remaining,
            });
        }

        let description = if desc_len > 0 {
            String::from_utf8(body[offset..offset + desc_len as usize].to_vec())?
        } else {
            String::new()
        };

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
