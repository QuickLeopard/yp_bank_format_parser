use std::io::{BufRead, Read, Write};

use crate::parsers::bin_format::YPBankBinParser;
use crate::parsers::csv_format::YPBankCsvParser;
use crate::parsers::error::ParserError;
use crate::parsers::txt_format::YPBankTxtParser;
use crate::parsers::types::YPBankRecord;

/// Parser for reading and writing YPBank records in various formats.
pub struct Parser;

impl Parser {
    /// Reads YPBank records from a reader in the specified format.
    ///
    /// # Arguments
    ///
    /// * `reader` - A reader implementing Read + BufRead traits
    /// * `format` - Format string ("csv", "txt", or "bin")
    ///
    /// # Returns
    ///
    /// Returns a Result containing a Vec<YPBankRecord> on success, or ParserError on failure.
    ///
    /// # Errors
    ///
    /// Returns ParserError::UnsupportedFormat if the format is not supported.
    /// Returns other ParserError variants for parsing failures.
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

    /// Writes YPBank records to a writer in the specified format.
    ///
    /// # Arguments
    ///
    /// * `writer` - A writer implementing the Write trait
    /// * `records` - Slice of YPBankRecord to write
    /// * `format` - Format string ("csv", "txt", or "bin")
    ///
    /// # Returns
    ///
    /// Returns a Result with () on success, or ParserError on failure.
    ///
    /// # Errors
    ///
    /// Returns ParserError::UnsupportedFormat if the format is not supported.
    /// Returns other ParserError variants for writing failures.
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
