
use core::hash;
use std::io;
use std::io::{Read, BufRead, BufReader, BufWriter, Write};

use std::collections::HashMap;

use std::fs::{self, File};

use std::env;

use yp_bank_parser_lib::parsers::error::ParserError;
use yp_bank_parser_lib::parsers::parser::Parser;
use yp_bank_parser_lib::parsers::types::{Status, TransactionType, YPBankRecord};
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

    let mut format1 = "csv".to_string();
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

    let mut format2 = "csv".to_string();
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

    let records1 = Parser::from_read(reader1, &format1)
        .expect("Failed to parse records from file1");

    let records2 = Parser::from_read(reader2, &format2)
        .expect("Failed to parse records from file2");

    let hashes1: HashMap<u64, &YPBankRecord> = records1.iter()
        .map(|record| (record.tx_id, record))
        .collect();
    
    let hashes2: HashMap<u64, &YPBankRecord> = records2.iter()
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
            },
            None => {
                println!("Record with TX_ID {} found in '{}' but not in '{}'", tx_id, file1_path, file2_path);
                diffs += 1;
            }
        }
    }

    if diffs == 0 {
        println!("The transaction records in '{}' and '{}' are identical.", file1_path, file2_path);
    } else {
        println!("Total differences found: {}", diffs);
    }
}
