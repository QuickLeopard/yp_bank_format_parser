
use std::io::{Read, BufRead};

use crate::parsers::types::YPBankCsvRecord;
use crate::parsers::error::ParserError;
use crate::parsers::csv_format::{YPBankCsvParser};

pub struct Parser;

impl Parser {
    pub fn from_read<R: Read + BufRead>(reader: R, format: &str) -> Result<Vec<YPBankCsvRecord>, ParserError> {
        match format.to_lowercase().as_str() {
            "csv" => YPBankCsvParser::from_read(reader),
            // "txt" => YPBankTxtParser::from_read(reader),
            _ => Err(ParserError::ParseError(format! ("Unsupported format: {}", format))),
        }
    }
}