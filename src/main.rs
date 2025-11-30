
use std::io;
use std::io::Read;
use std::io::BufRead;

use yp_bank_format_parser::parsers::csv_format::YPBankCsvParser;
use yp_bank_format_parser::parsers::types::{YPBankCsvRecord, TransactionType, Status};

fn main () {
    
    let data = YPBankCsvParser::from_read (io::stdin().lock()).unwrap();

    for record in data {
        println!("Record: {:?}", record);
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