
use std::io;
use std::io::BufReader;
use std::io::Read;
use std::io::{BufRead, BufWriter, Write};

use std::fs::{self, File};

use std::env;

use yp_bank_format_parser::parsers::types::{YPBankCsvRecord, TransactionType, Status};
use yp_bank_format_parser::parsers::parser::Parser;

fn usage () {
    println!("Использование:");
    println!("  --input <input_file>");
    println!("  --input-format <format>");
    println!("  --output <output_file>");
    println!("  --output-format <format>");
}

fn main () {
    
    let args: Vec<String> = env::args().collect();

    if /*args.len() < 2 ||*/ (args.len() == 2 && args[1] == "--help") {
        usage ();
        return;
    }    

    println!("Args: {:?}", args);

    let reader: Box<dyn BufRead> = if args.len() >= 3 && args[1] == "--input" {
        println!("Reading from file: {}", &args[2]);
        let fs = File::open(&args[2]).expect ("Failed to open input file");
        Box::new(BufReader::new(fs))
    } else {
        Box::new(io::stdin().lock())
    };

    let mut input_format = "csv";
    if args.len () >= 2 {
        let splitted: Vec<&str> = args[2].split(".").collect();
        if args.len() >= 5 && args[3] == "--input-format" {
            input_format = &args[4];
        }
        else if splitted.len() > 1 {
            if let Some(ext) = splitted.last() {
                match *ext {
                    "csv" => input_format = "csv",
                    "txt" => input_format = "txt",
                    "bin" => input_format = "bin",
                    _ => {}
                }
            }
        }
    }

    let mut writer: Box<dyn Write> = if args.len() >= 7 && args[5] == "--output" {
        println!("Writing to file: {}", &args[6]);
        let fs = File::create(&args[6]).expect ("Failed to open output file");
        Box::new(BufWriter::new(fs))
    } else {
        Box::new(io::stdout().lock())
    };

    let mut output_format = "csv";
    if args.len () >= 6 {
        let splitted: Vec<&str> = args[6].split(".").collect();
        if args.len() >= 8 && args[7] == "--output-format" {
            input_format = &args[8];
        }
        else if splitted.len() > 1 {
                if let Some(ext) = splitted.last() {
                    match *ext {
                        "csv" => output_format = "csv",
                        "txt" => output_format = "txt",
                        "bin" => output_format = "bin",
                        _ => {}
                    }
                }
            }
    }

    println! ("Input format: {}", input_format);
    println! ("Output format: {}", output_format);

    let records = 
    
        match Parser::from_read (reader, input_format) {
            Ok(records) => records,
            Err(err) =>{
                eprintln!("Error parsing input: {:?}", err);
                return;
            }
        };

    Parser::write_to(writer, &records, output_format);
    
    /*for record in data {
        writer.write("Record: {:?}", record);
    }*/
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