use std::io::{BufRead, Read, Write};

use crate::parsers::bin_format::YPBankBinParser;
use crate::parsers::csv_format::YPBankCsvParser;
use crate::parsers::error::ParserError;
use crate::parsers::txt_format::YPBankTxtParser;
use crate::parsers::types::YPBankRecord;

pub struct Parser;

impl Parser {
    pub fn from_read<R: Read + BufRead>(
        reader: R,
        format: &str,
    ) -> Result<Vec<YPBankRecord>, ParserError> {
        match format.to_lowercase().as_str() {
            "csv" => YPBankCsvParser::from_read(reader),
            "txt" => YPBankTxtParser::from_read(reader),
            "bin" => YPBankBinParser::from_read(reader),
            _ => Err(ParserError::UnsupportedFormat(format.to_string())),
        }
    }

    pub fn write_to<W: Write>(
        writer: W,
        records: &[YPBankRecord],
        format: &str,
    ) -> Result<(), ParserError> {
        match format.to_lowercase().as_str() {
            "csv" => YPBankCsvParser::write_to(writer, records),
            "txt" => YPBankTxtParser::write_to(writer, records),
            "bin" => YPBankBinParser::write_to(writer, records),
            _ => Err(ParserError::UnsupportedFormat(format.to_string())),
        }
    }
}
