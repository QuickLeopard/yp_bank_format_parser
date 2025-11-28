
use std::io;
use std::io::Read;
use std::io::BufRead;

use yp_bank_format_parser::parsers::csv_format::{YPBankCsvRecord};

fn read_csv<R: Read + BufRead>(reader: R) -> Vec<(String, String)> {

    let mut records = Vec::new();    
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 2 {
            records.push((
                parts[0].to_string(),
                parts[1].to_string(),
            ));
        }
    }

    records
}


fn main () {
    println!("Hello from parser!");

    let data = YPBankCsvRecord::from_read (io::stdin().lock());

    for record in data {
        println!("Record: {:?}", record);
    }
}