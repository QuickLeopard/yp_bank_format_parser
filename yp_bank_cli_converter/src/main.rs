use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};

use std::fs::File;

use std::env;

use yp_bank_parser_lib::parsers::error::ParserError;
use yp_bank_parser_lib::parsers::parser::Parser;
use yp_bank_parser_lib::{extract_format, parse_cli_args};

fn usage() {
    println!("Использование:");
    println!("  --input <input_file>");
    println!("  --input-format <format>");
    println!("  --output <output_file>");
    println!("  --output-format <format>");
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    println!("Args: {:?}", args);

    if args.len() == 1 && args[0] == "--help" {
        usage();
        return;
    }

    let args_map = parse_cli_args(
        &args,
        &["--input", "--input-format", "--output", "--output-format"],
    );

    let mut input_format = "csv".to_string();
    let reader: Box<dyn BufRead> = if args_map.contains_key("--input") {
        let file_path = args_map.get("--input").expect("Empty --input argument!");

        input_format = extract_format(file_path);

        println!("Reading from file: {}", file_path);
        let fs = File::open(file_path).expect("Failed to open input file");
        Box::new(BufReader::new(fs))
    } else {
        Box::new(io::stdin().lock())
    };

    if args_map.contains_key("--input-format") {
        input_format = args_map
            .get("--input-format")
            .expect("Empty --input-format argument!")
            .to_string();
    }

    println!("Input format: {}", input_format);

    let mut output_format = "csv".to_string();
    let writer: Box<dyn Write> = if args_map.contains_key("--output") {
        let file_path = args_map.get("--output").expect("Empty --output argument!");

        output_format = extract_format(file_path);

        println!("Writing to file: {}", file_path);
        let fs = File::create(file_path).expect("Failed to open output file");
        Box::new(BufWriter::new(fs))
    } else {
        Box::new(io::stdout().lock())
    };

    if args_map.contains_key("--output-format") {
        output_format = args_map
            .get("--output-format")
            .expect("Empty --output-format argument!")
            .to_string();
    }

    println!("Output format: {}", output_format);

    let records = match Parser::from_read(reader, &input_format) {
        Ok(records) => records,
        Err(err) => {
            panic!("Error parsing input: {:?}", err);
        }
    };

    let write_result = Parser::write_to(writer, &records, &output_format);
    if let Err(ParserError::ParseError(e)) = write_result {
        panic!("Write to output error: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::BufReader;
    use yp_bank_parser_lib::test_helpers::{create_test_records};

    fn create_temp_file(suffix: &str) -> String {
        format!("test_temp_{}.{}", std::process::id(), suffix)
    }

    fn cleanup_file(path: &str) {
        let _ = fs::remove_file(path);
    }

    // Positive Tests - File I/O
    #[test]
    fn test_csv_file_read_write() {
        let input_file = create_temp_file("input.csv");
        let output_file = create_temp_file("output.csv");
        
        // Create test CSV file
        let csv_content = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n123,Deposit,456,789,1000,1640995200,Success,Test transaction\n";
        fs::write(&input_file, csv_content).unwrap();
        
        // Read from file
        let file = File::open(&input_file).unwrap();
        let reader = BufReader::new(file);
        let records = Parser::from_read(reader, "csv").unwrap();
        
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].tx_id, 123);
        
        // Write to file
        let file = File::create(&output_file).unwrap();
        let writer = BufWriter::new(file);
        Parser::write_to(writer, &records, "csv").unwrap();
        
        // Verify output file
        let output_content = fs::read_to_string(&output_file).unwrap();
        assert!(output_content.contains("123"));
        assert!(output_content.contains("Deposit"));
        
        cleanup_file(&input_file);
        cleanup_file(&output_file);
    }

    #[test]
    fn test_txt_file_read_write() {
        let input_file = create_temp_file("input.txt");
        let output_file = create_temp_file("output.txt");
        
        let txt_content = "tx_id: 123\ntx_type: Deposit\nfrom_user_id: 456\nto_user_id: 789\namount: 1000\ntimestamp: 1640995200\nstatus: Success\ndescription: Test transaction\n";
        fs::write(&input_file, txt_content).unwrap();
        
        let file = File::open(&input_file).unwrap();
        let reader = BufReader::new(file);
        let records = Parser::from_read(reader, "txt").unwrap();
        
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].tx_id, 123);
        
        let file = File::create(&output_file).unwrap();
        let writer = BufWriter::new(file);
        Parser::write_to(writer, &records, "txt").unwrap();
        
        let output_content = fs::read_to_string(&output_file).unwrap();
        assert!(output_content.contains("tx_id: 123"));
        
        cleanup_file(&input_file);
        cleanup_file(&output_file);
    }

    #[test]
    fn test_bin_file_read_write() {
        let input_file = create_temp_file("input.bin");
        let output_file = create_temp_file("output.bin");
        
        let records = create_test_records(5, 100);
        
        // Write to binary file
        let file = File::create(&input_file).unwrap();
        let writer = BufWriter::new(file);
        Parser::write_to(writer, &records, "bin").unwrap();
        
        // Read from binary file
        let file = File::open(&input_file).unwrap();
        let reader = BufReader::new(file);
        let parsed_records = Parser::from_read(reader, "bin").unwrap();
        
        assert_eq!(parsed_records.len(), 5);
        for (original, parsed) in records.iter().zip(parsed_records.iter()) {
            assert_eq!(original.tx_id, parsed.tx_id);
            assert_eq!(original.amount, parsed.amount);
        }
        
        // Write parsed records to output file
        let file = File::create(&output_file).unwrap();
        let writer = BufWriter::new(file);
        Parser::write_to(writer, &parsed_records, "bin").unwrap();
        
        // Verify output file exists and has content
        let metadata = fs::metadata(&output_file).unwrap();
        assert!(metadata.len() > 0);
        
        cleanup_file(&input_file);
        cleanup_file(&output_file);
    }

    #[test]
    fn test_format_conversion_files() {
        let csv_file = create_temp_file("convert.csv");
        let txt_file = create_temp_file("convert.txt");
        let bin_file = create_temp_file("convert.bin");
        
        let records = create_test_records(3, 200);
        
        // CSV -> TXT -> BIN conversion chain
        let file = File::create(&csv_file).unwrap();
        let writer = BufWriter::new(file);
        Parser::write_to(writer, &records, "csv").unwrap();
        
        let file = File::open(&csv_file).unwrap();
        let reader = BufReader::new(file);
        let csv_records = Parser::from_read(reader, "csv").unwrap();
        
        let file = File::create(&txt_file).unwrap();
        let writer = BufWriter::new(file);
        Parser::write_to(writer, &csv_records, "txt").unwrap();
        
        let file = File::open(&txt_file).unwrap();
        let reader = BufReader::new(file);
        let txt_records = Parser::from_read(reader, "txt").unwrap();
        
        let file = File::create(&bin_file).unwrap();
        let writer = BufWriter::new(file);
        Parser::write_to(writer, &txt_records, "bin").unwrap();
        
        let file = File::open(&bin_file).unwrap();
        let reader = BufReader::new(file);
        let bin_records = Parser::from_read(reader, "bin").unwrap();
        
        assert_eq!(records.len(), bin_records.len());
        for (original, final_record) in records.iter().zip(bin_records.iter()) {
            assert_eq!(original.tx_id, final_record.tx_id);
            assert_eq!(original.amount, final_record.amount);
        }
        
        cleanup_file(&csv_file);
        cleanup_file(&txt_file);
        cleanup_file(&bin_file);
    }

    #[test]
    fn test_large_file_processing() {
        let input_file = create_temp_file("large.csv");
        let output_file = create_temp_file("large_out.txt");
        
        let records = create_test_records(100, 1000);
        
        let file = File::create(&input_file).unwrap();
        let writer = BufWriter::new(file);
        Parser::write_to(writer, &records, "csv").unwrap();
        
        let file = File::open(&input_file).unwrap();
        let reader = BufReader::new(file);
        let parsed_records = Parser::from_read(reader, "csv").unwrap();
        
        assert_eq!(parsed_records.len(), 100);
        
        let file = File::create(&output_file).unwrap();
        let writer = BufWriter::new(file);
        Parser::write_to(writer, &parsed_records, "txt").unwrap();
        
        let output_content = fs::read_to_string(&output_file).unwrap();
        assert!(output_content.len() > 1000); // Should be substantial content
        
        cleanup_file(&input_file);
        cleanup_file(&output_file);
    }

    // Negative Tests - File I/O
    #[test]
    fn test_nonexistent_input_file() {
        let nonexistent_file = "nonexistent_file_12345.csv";
        
        let result = File::open(nonexistent_file);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_csv_file() {
        let invalid_file = create_temp_file("invalid.csv");
        
        let invalid_content = "WRONG,HEADER\ninvalid,data,here";
        fs::write(&invalid_file, invalid_content).unwrap();
        
        let file = File::open(&invalid_file).unwrap();
        let reader = BufReader::new(file);
        let result = Parser::from_read(reader, "csv");
        
        assert!(result.is_err());
        cleanup_file(&invalid_file);
    }

    #[test]
    fn test_corrupted_txt_file() {
        let corrupted_file = create_temp_file("corrupted.txt");
        
        let corrupted_content = "tx_id: 123\ninvalid line without colon\ntx_type: Deposit";
        fs::write(&corrupted_file, corrupted_content).unwrap();
        
        let file = File::open(&corrupted_file).unwrap();
        let reader = BufReader::new(file);
        let result = Parser::from_read(reader, "txt");
        
        assert!(result.is_err());
        cleanup_file(&corrupted_file);
    }

    #[test]
    fn test_empty_file() {
        let empty_file = create_temp_file("empty.csv");
        
        fs::write(&empty_file, "").unwrap();
        
        let file = File::open(&empty_file).unwrap();
        let reader = BufReader::new(file);
        let result = Parser::from_read(reader, "csv");
        
        assert!(result.is_err());
        cleanup_file(&empty_file);
    }

    #[test]
    fn test_write_to_readonly_directory() {
        // This test may not work on all systems, but demonstrates the concept
        let readonly_path = "/readonly/test.csv"; // Unix path that typically doesn't exist
        
        let result = File::create(readonly_path);
        
        // Should fail to create file in non-existent/readonly directory
        assert!(result.is_err());
    }

    #[test]
    fn test_unsupported_format_file() {
        let test_file = create_temp_file("test.xml");
        
        fs::write(&test_file, "<xml>data</xml>").unwrap();
        
        let file = File::open(&test_file).unwrap();
        let reader = BufReader::new(file);
        let result = Parser::from_read(reader, "xml");
        
        assert!(result.is_err());
        cleanup_file(&test_file);
    }

    #[test]
    fn test_file_permissions() {
        let test_file = create_temp_file("permissions.csv");
        let _records = create_test_records(2, 300);
        
        // Create file first
        let file = File::create(&test_file).unwrap();
        let writer = BufWriter::new(file);
        Parser::write_to(writer, &_records, "csv").unwrap();
        
        // Verify file was created and has content
        let metadata = fs::metadata(&test_file).unwrap();
        assert!(metadata.len() > 0);
        
        // Verify we can read it back
        let file = File::open(&test_file).unwrap();
        let reader = BufReader::new(file);
        let parsed_records = Parser::from_read(reader, "csv").unwrap();
        
        assert_eq!(parsed_records.len(), 2);
        cleanup_file(&test_file);
    }
}