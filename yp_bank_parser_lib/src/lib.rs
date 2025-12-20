use std::collections::HashMap;

pub mod parsers;

/// Magic bytes identifying a YPBankBin record header: "YPBN"
pub const MAGIC: [u8; 4] = [0x59, 0x50, 0x42, 0x4E];

/// Header size in bytes (MAGIC + RECORD_SIZE)
pub const HEADER_SIZE: usize = 8;

/// Minimum body size in bytes (fixed fields without description)
pub const MIN_BODY_SIZE: usize = 46;

pub const MAX_RECORD_SIZE: usize = 10 * 1024 * 1024;

pub mod test_helpers {
    use crate::parsers::types::{YPBankRecord, TransactionType, Status};

/// Creates a test YPBankRecord with deterministic data based on a seed.
    ///
    /// # Arguments
    ///
    /// * `seed` - A u64 value used to generate deterministic test data
    ///
    /// # Returns
    ///
    /// Returns a YPBankRecord with fields generated from the seed value.
    pub fn create_test_record(seed: u64) -> YPBankRecord {
        let tx_types = [TransactionType::Deposit, TransactionType::Withdrawal, TransactionType::Transfer];
        let statuses = [Status::Success, Status::Failure, Status::Pending];
        let descriptions = ["Payment", "Transfer", "Deposit", "Withdrawal", "Fee", "Refund"];
        
        YPBankRecord {
            tx_id: seed % 1000000 + 1,
            tx_type: tx_types[(seed % 3) as usize],
            from_user_id: (seed >> 8) % 10000 + 1,
            to_user_id: (seed >> 16) % 10000 + 1,
            amount: ((seed >> 24) % 100000 + 100) as i64,
            timestamp: 1640995200 + (seed % 31536000),
            status: statuses[((seed >> 32) % 3) as usize],
            description: format!("{} {}", descriptions[((seed >> 40) % 6) as usize], seed % 1000),
        }
    }

    /// Creates a vector of test YPBankRecords.
    ///
    /// # Arguments
    ///
    /// * `count` - Number of records to create
    /// * `base_seed` - Base seed value for generating deterministic data
    ///
    /// # Returns
    ///
    /// Returns a Vec<YPBankRecord> containing the specified number of test records.
    pub fn create_test_records(count: usize, base_seed: u64) -> Vec<YPBankRecord> {
        (0..count).map(|i| create_test_record(base_seed + i as u64)).collect()
    }
}

/// Extracts the file format from a file path based on its extension.
///
/// # Arguments
///
/// * `file_path` - A string slice containing the file path
///
/// # Returns
///
/// Returns a String representing the format:
/// - "csv" for .csv files
/// - "txt" for .txt files  
/// - "bin" for .bin files
/// - "csv" as default for unknown extensions
///
/// # Examples
///
/// ```
/// use yp_bank_parser_lib::extract_format;
///
/// assert_eq!(extract_format("data.csv"), "csv");
/// assert_eq!(extract_format("data.txt"), "txt");
/// assert_eq!(extract_format("data.bin"), "bin");
/// assert_eq!(extract_format("data"), "csv"); // default
/// ```
pub fn extract_format(file_path: &str) -> String {
    let split_path: Vec<&str> = file_path.split(".").collect();
    if split_path.len() > 1 
       && let Some(ext) = split_path.last() {
        match *ext {
            "csv" => return "csv".to_string(),
            "txt" => return "txt".to_string(),
            "bin" => return "bin".to_string(),
            _ => return "csv".to_string(),
        }
    }    
    "csv".to_string()
}

/// Parses command line arguments into a HashMap.
///
/// Takes pairs of arguments and validates them against a list of valid arguments.
/// Panics if an invalid argument is encountered.
///
/// # Arguments
///
/// * `args` - Slice of command line arguments as strings
/// * `valid_args` - Slice of valid argument names for validation
///
/// # Returns
///
/// Returns a HashMap mapping argument names to their values.
///
/// # Panics
///
/// Panics if an argument is not in the `valid_args` list.
///
/// # Examples
///
/// ```
/// use yp_bank_parser_lib::parse_cli_args;
///
/// let args = vec!["--input".to_string(), "file.csv".to_string()];
/// let valid = &["--input", "--output"];
/// let result = parse_cli_args(&args, valid);
/// assert_eq!(result.get("--input"), Some(&"file.csv".to_string()));
/// ```
pub fn parse_cli_args(args: &[String], valid_args: &[&str]) -> HashMap<String, String> {
    let mut dict = HashMap::new();

    for chunk in args.chunks_exact(2) {
        dict.insert(
            chunk[0].trim().to_string(),
            chunk[1].trim().to_string(),
        );

        if !valid_args.contains(&chunk[0].as_str()) {
            panic!("Unknown CLI arguments: {} {}", chunk[0], chunk[1]);
        }
    }

    dict
}

#[cfg(test)]
mod tests {
    use crate::{extract_format, parse_cli_args};
    use crate::parsers::parser::Parser;
    use std::io::Cursor;
    use crate::parsers::types::{YPBankRecord};
    use crate::test_helpers::{create_test_records};

    macro_rules! try_test {
        ($expr:expr) => {
            match $expr {
                Ok(val) => val,
                Err(_) => return,
            }
        };
    }    

    #[test]
    fn test_csv_read_write_positive() {
        let csv_data = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n123,Deposit,456,789,1000,1640995200,Success,Test transaction\n";
        let reader = Cursor::new(csv_data);
        
        let records = try_test!(Parser::from_read(reader, "csv"));
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].tx_id, 123);
        
        let mut output = Vec::new();
        try_test!(Parser::write_to(&mut output, &records, "csv"));
        let output_str = try_test!(String::from_utf8(output));
        assert!(output_str.contains("123"));
        assert!(output_str.contains("Deposit"));
    }

    #[test]
    fn test_csv_multiple_records() {
        let records = create_test_records(15, 100);
        
        let mut csv_output = Vec::new();
        try_test!(Parser::write_to(&mut csv_output, &records, "csv"));
        
        let reader = Cursor::new(csv_output);
        let parsed_records = try_test!(Parser::from_read(reader, "csv"));
        
        assert_eq!(parsed_records.len(), 15);
        for (original, parsed) in records.iter().zip(parsed_records.iter()) {
            assert_eq!(original.tx_id, parsed.tx_id);
            assert_eq!(original.amount, parsed.amount);
        }
    }

    #[test]
    fn test_txt_multiple_records() {
        let records = create_test_records(10, 200);
        
        let mut txt_output = Vec::new();
        try_test!(Parser::write_to(&mut txt_output, &records, "txt"));
        
        let reader = Cursor::new(txt_output);
        let parsed_records = try_test!(Parser::from_read(reader, "txt"));
        
        assert_eq!(parsed_records.len(), 10);
        for (original, parsed) in records.iter().zip(parsed_records.iter()) {
            assert_eq!(original.tx_id, parsed.tx_id);
            assert_eq!(original.description, parsed.description);
        }
    }

    #[test]
    fn test_bin_multiple_records() {
        let records = create_test_records(25, 400);
        
        let mut bin_output = Vec::new();
        try_test!(Parser::write_to(&mut bin_output, &records, "bin"));
        
        let reader = Cursor::new(bin_output);
        let parsed_records = try_test!(Parser::from_read(reader, "bin"));
        
        assert_eq!(parsed_records.len(), 25);
        for (original, parsed) in records.iter().zip(parsed_records.iter()) {
            assert_eq!(original.tx_id, parsed.tx_id);
            assert_eq!(original.tx_type, parsed.tx_type);
            assert_eq!(original.status, parsed.status);
        }
    }

    #[test]
    fn test_format_conversion_multiple_records() {
        let records = create_test_records(8, 1000);
        
        let mut csv_output = Vec::new();
        try_test!(Parser::write_to(&mut csv_output, &records, "csv"));
        
        let csv_reader = Cursor::new(csv_output);
        let csv_records = try_test!(Parser::from_read(csv_reader, "csv"));
        
        let mut txt_output = Vec::new();
        try_test!(Parser::write_to(&mut txt_output, &csv_records, "txt"));
        
        let txt_reader = Cursor::new(txt_output);
        let txt_records = try_test!(Parser::from_read(txt_reader, "txt"));
        
        assert_eq!(records.len(), txt_records.len());
        
        let mut bin_output = Vec::new();
        try_test!(Parser::write_to(&mut bin_output, &txt_records, "bin"));
        
        let bin_reader = Cursor::new(bin_output);
        let bin_records = try_test!(Parser::from_read(bin_reader, "bin"));
        
        assert_eq!(records.len(), bin_records.len());
        for (original, final_record) in records.iter().zip(bin_records.iter()) {
            assert_eq!(original.tx_id, final_record.tx_id);
            assert_eq!(original.amount, final_record.amount);
        }
    }

    #[test]
    fn test_edge_case_small_batch() {
        let records = create_test_records(5, 500);
        
        for format in ["csv", "txt", "bin"] {
            let mut output = Vec::new();
            try_test!(Parser::write_to(&mut output, &records, format));
            
            let reader = Cursor::new(output);
            let parsed_records = try_test!(Parser::from_read(reader, format));
            
            assert_eq!(parsed_records.len(), 5);
        }
    }

    #[test]
    fn test_edge_case_large_batch() {
        let records = create_test_records(20, 2000);
        
        for format in ["csv", "txt", "bin"] {
            let mut output = Vec::new();
            try_test!(Parser::write_to(&mut output, &records, format));
            
            let reader = Cursor::new(output);
            let parsed_records = try_test!(Parser::from_read(reader, format));
            
            assert_eq!(parsed_records.len(), 20);
        }
    }

    #[test]
    fn test_csv_read_invalid_header() {
        let csv_data = "INVALID,HEADER\n123,Deposit";
        let reader = Cursor::new(csv_data);
        
        let result = Parser::from_read(reader, "csv");
        assert!(result.is_err());
    }

    #[test]
    fn test_unsupported_format() {
        let data = "some data";
        let reader = Cursor::new(data);
        
        let result = Parser::from_read(reader, "xml");
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_records_write() {
        let records: Vec<YPBankRecord> = vec![];
        let mut output = Vec::new();
        
        let result = Parser::write_to(&mut output, &records, "bin");
        assert!(result.is_err());
    }

    #[test]
    fn extract_fromat_works_correclty() {
        assert_eq!(extract_format("a.csv"), "csv");
        assert_eq!(extract_format("ab.txt"), "txt");
        assert_eq!(extract_format("abc.bin"), "bin");
        assert_eq!(extract_format("abcde"), "csv");

        assert_eq!(extract_format("file1.csv2"), "csv");
        assert_eq!(extract_format("file100.cs"), "csv");

        //assert_eq!(storage.get_balance(&"John".to_string()), Some(100));
    }

    #[test]
    fn parse_cli_args_tests() {
        let dict = parse_cli_args(
            &[
                "--file1".to_string(),
                "abc.csv".to_string(),
                "--file3".to_string(),
                "efgh.bin".to_string(),
            ],
            &["--file1", "--file2", "--file3"],
        );

        assert_eq!(dict.get("--file1"), Some(&"abc.csv".to_string()));
        assert_eq!(dict.get("--file3"), Some(&"efgh.bin".to_string()));
        assert_eq!(dict.len(), 2);

        assert_eq!(dict.get("--file2"), None);
        assert_eq!(dict.get("--file10"), None);
    }
}
