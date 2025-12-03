
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

    let args_map = parse_cli_args(&args, &["--input", "--input-format", "--output", "--output-format"]);

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
    let mut writer: Box<dyn Write> = if args_map.contains_key("--output") {
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
            eprintln!("Error parsing input: {:?}", err);
            return;
        }
    };

    let write_result = Parser::write_to(writer, &records, &output_format);
    if let Err(ParserError::ParseError(e)) = write_result {
        eprintln!("Write to outpu error: {}", e);
    }
}

#[cfg(test)]
mod tests {

    use std::fs::{self, File};
    use std::io::Write;

    #[test]
    fn write_works() {
        let file_path = "_test.csv";

        let mut f = File::create(file_path).unwrap();
        //writeln! (f, "John;100;[]");

        //assert_eq!(storage.get_balance(&"John".to_string()), Some(100));

        fs::remove_file(file_path).unwrap();
    }
}
