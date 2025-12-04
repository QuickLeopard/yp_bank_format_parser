use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::array::TryFromSliceError;
use std::io::{self, BufRead, Read, Seek, SeekFrom, Write};

use crate::parsers::error::ParserError;
use crate::parsers::types::{Status, TransactionType, YPBankRecord};

use crate::{HEADER_SIZE, MAGIC, MIN_BODY_SIZE};

const MAGIC_HEADER: u32 = 0x5950424E; // 'YPBN' in ASCII

pub struct YPBankBinParser;

impl YPBankBinParser {
    /*pub fn from_read<R: Read + BufRead>(mut reader: R) -> Result<Vec<YPBankCsvRecord>, ParserError> {
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
                timestamp,
                status,
                description,
            };

            records.push(record.clone ());

            println!("");
            println! ("Parsed: {} records, record: {:?}", records.len(), record);
        }

        Ok (records)
    }*/

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
            let magic: [u8; 4] = header_buf[0..4].try_into().unwrap();
            if magic != MAGIC {
                return Err(ParserError::InvalidMagic(magic));
            }

            // Read record size (big-endian u32)
            let record_size = u32::from_be_bytes(header_buf[4..8].try_into().unwrap());

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
