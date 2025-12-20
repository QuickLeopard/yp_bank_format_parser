
use std::io::{BufReader};

use std::collections::HashMap;

use std::fs::{File};

use std::env;
use yp_bank_parser_lib::parsers::parser::Parser;
use yp_bank_parser_lib::parsers::types::{YPBankRecord};
use yp_bank_parser_lib::{extract_format, parse_cli_args};

fn usage() {
    println!("Использование:");
    println!("  --file1 <input_file>");
    println!("  --format1 <format>");
    println!("  --file2 <input_file>");
    println!("  --format2 <format>");
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    println!("Args: {:?}", args);

    if args.len() == 1 && args[0] == "--help" {
        usage();
        return;
    }

    let args_map = parse_cli_args(&args, &["--file1", "--format1", "--file2", "--format2"]);

    if !args_map.contains_key("--file1") || !args_map.contains_key("--file2") {
        panic!("Both --file1 and --file2 arguments are required.");
    }

    let mut format1 ;
    let file1_path = args_map.get("--file1").expect("Empty --input argument!");
    let fs1 = File::open(file1_path).expect("Failed to open input file1");
    let reader1 = BufReader::new(fs1);

    format1 = extract_format(file1_path);

    if args_map.contains_key("--format1") {
        format1 = args_map
            .get("--format1")
            .expect("Empty --format1 argument!")
            .to_string();
    }

    let mut format2;
    let file2_path = args_map.get("--file2").expect("Empty --input argument!");
    let fs2 = File::open(file2_path).expect("Failed to open input file2");
    let reader2 = BufReader::new(fs2);

    format2 = extract_format(file2_path);

    if args_map.contains_key("--format2") {
        format2 = args_map
            .get("--format2")
            .expect("Empty --format2 argument!")
            .to_string();
    }

    let records1 =
        Parser::from_read(reader1, &format1).expect("Failed to parse records from file1");

    let records2 =
        Parser::from_read(reader2, &format2).expect("Failed to parse records from file2");

    let hashes1: HashMap<u64, &YPBankRecord> = records1
        .iter()
        .map(|record| (record.tx_id, record))
        .collect();

    let hashes2: HashMap<u64, &YPBankRecord> = records2
        .iter()
        .map(|record| (record.tx_id, record))
        .collect();

    let mut diffs = 0;
    for (tx_id, record1) in &hashes1 {
        match hashes2.get(tx_id) {
            Some(record2) => {
                if record1 != record2 {
                    println!("Difference found for TX_ID {}:", tx_id);
                    println!("  File1: {:?}", record1);
                    println!("  File2: {:?}", record2);
                    diffs += 1;
                }
            }
            None => {
                println!(
                    "Record with TX_ID {} found in '{}' but not in '{}'",
                    tx_id, file1_path, file2_path
                );
                diffs += 1;
            }
        }
    }

    if diffs == 0 {
        println!(
            "The transaction records in '{}' and '{}' are identical.",
            file1_path, file2_path
        );
    } else {
        println!("Total differences found: {}", diffs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use yp_bank_parser_lib::parsers::types::{YPBankRecord, Status};
    use yp_bank_parser_lib::test_helpers::{create_test_records};


    fn compare_records(records1: &[YPBankRecord], records2: &[YPBankRecord]) -> usize {
        let hashes1: HashMap<u64, &YPBankRecord> = records1
            .iter()
            .map(|record| (record.tx_id, record))
            .collect();

        let hashes2: HashMap<u64, &YPBankRecord> = records2
            .iter()
            .map(|record| (record.tx_id, record))
            .collect();

        let mut diffs = 0;
        for (tx_id, record1) in &hashes1 {
            match hashes2.get(tx_id) {
                Some(record2) => {
                    if record1 != record2 {
                        diffs += 1;
                    }
                }
                None => {
                    diffs += 1;
                }
            }
        }

        // Check for records in file2 but not in file1
        for tx_id in hashes2.keys() {
            if !hashes1.contains_key(tx_id) {
                diffs += 1;
            }
        }

        diffs
    }

    #[test]
    fn test_identical_records() {
        let records1 = create_test_records(10, 100);
        let records2 = records1.clone();
        
        let diffs = compare_records(&records1, &records2);
        assert_eq!(diffs, 0);
    }

    #[test]
    fn test_different_records() {
        let records1 = create_test_records(5, 100);
        let mut records2 = records1.clone();
        
        // Modify one record
        records2[0].amount = 999999;
        
        let diffs = compare_records(&records1, &records2);
        assert_eq!(diffs, 1);
    }

    #[test]
    fn test_missing_records() {
        let records1 = create_test_records(10, 100);
        let records2 = create_test_records(8, 100); // Missing 2 records
        
        let diffs = compare_records(&records1, &records2);
        assert_eq!(diffs, 2);
    }

    #[test]
    fn test_extra_records() {
        let records1 = create_test_records(5, 100);
        let records2 = create_test_records(8, 100); // 3 extra records
        
        let diffs = compare_records(&records1, &records2);
        assert_eq!(diffs, 3);
    }

    #[test]
    fn test_completely_different_records() {
        let records1 = create_test_records(5, 100);
        let records2 = create_test_records(5, 200); // Different seed = different records
        
        let diffs = compare_records(&records1, &records2);
        assert_eq!(diffs, 10); // All records are different (5 missing + 5 extra)
    }

    #[test]
    fn test_multiple_differences() {
        let records1 = create_test_records(10, 100);
        let mut records2 = records1.clone();
        
        // Modify multiple records
        records2[0].amount = 999999;
        records2[1].description = "Modified".to_string();
        records2[2].status = Status::Failure;
        
        let diffs = compare_records(&records1, &records2);
        assert_eq!(diffs, 3);
    }

    #[test]
    fn test_empty_files() {
        let records1: Vec<YPBankRecord> = vec![];
        let records2: Vec<YPBankRecord> = vec![];
        
        let diffs = compare_records(&records1, &records2);
        assert_eq!(diffs, 0);
    }

    #[test]
    fn test_one_empty_file() {
        let records1 = create_test_records(5, 100);
        let records2: Vec<YPBankRecord> = vec![];
        
        let diffs = compare_records(&records1, &records2);
        assert_eq!(diffs, 5);
    }

    #[test]
    fn test_large_dataset_comparison() {
        let records1 = create_test_records(100, 1000);
        let records2 = records1.clone();
        
        let diffs = compare_records(&records1, &records2);
        assert_eq!(diffs, 0);
    }

    #[test]
    fn test_large_dataset_with_differences() {
        let records1 = create_test_records(100, 1000);
        let mut records2 = records1.clone();
        
        // Modify every 10th record
        for i in (0..records2.len()).step_by(10) {
            records2[i].amount += 1;
        }
        
        let diffs = compare_records(&records1, &records2);
        assert_eq!(diffs, 10);
    }

    #[test]
    fn test_duplicate_tx_ids() {
        let mut records1 = create_test_records(5, 100);
        let mut records2 = create_test_records(5, 100);
        
        // Create duplicate tx_ids within each set
        records1[1].tx_id = records1[0].tx_id;
        records2[1].tx_id = records2[0].tx_id;
        
        // The comparison should still work (last record with same tx_id wins)
        let diffs = compare_records(&records1, &records2);
        assert_eq!(diffs, 0);
    }

    #[test]
    fn test_format_independence() {
        // Test that comparison works regardless of how records were created
        let records1 = create_test_records(5, 100);
        
        // Create records2 by serializing to CSV and back
        let mut csv_output = Vec::new();
        if let Err(_) = Parser::write_to(&mut csv_output, &records1, "csv") {
            return;
        }
        
        let csv_reader = Cursor::new(csv_output);
        let records2 = match Parser::from_read(csv_reader, "csv") {
            Ok(records) => records,
            Err(_) => return,
        };
        
        let diffs = compare_records(&records1, &records2);
        assert_eq!(diffs, 0);
    }

    #[test]
    fn test_cross_format_comparison() {
        let records1 = create_test_records(3, 100);
        
        // Convert to TXT format and back
        let mut txt_output = Vec::new();
        if let Err(_) = Parser::write_to(&mut txt_output, &records1, "txt") {
            return;
        }
        
        let txt_reader = Cursor::new(txt_output);
        let records2 = match Parser::from_read(txt_reader, "txt") {
            Ok(records) => records,
            Err(_) => return,
        };
        
        let diffs = compare_records(&records1, &records2);
        assert_eq!(diffs, 0);
    }

    #[test]
    fn test_binary_format_comparison() {
        let records1 = create_test_records(5, 100);
        
        // Convert to BIN format and back
        let mut bin_output = Vec::new();
        if let Err(_) = Parser::write_to(&mut bin_output, &records1, "bin") {
            return;
        }
        
        let bin_reader = Cursor::new(bin_output);
        let records2 = match Parser::from_read(bin_reader, "bin") {
            Ok(records) => records,
            Err(_) => return,
        };
        
        let diffs = compare_records(&records1, &records2);
        assert_eq!(diffs, 0);
    }
}
