
use std::collections::HashMap;

pub mod parsers;

/// Magic bytes identifying a YPBankBin record header: "YPBN"
pub const MAGIC: [u8; 4] = [0x59, 0x50, 0x42, 0x4E];

/// Header size in bytes (MAGIC + RECORD_SIZE)
pub const HEADER_SIZE: usize = 8;

/// Minimum body size in bytes (fixed fields without description)
pub const MIN_BODY_SIZE: usize = 46;

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

pub fn parse_cli_args(args: &[String], valid_args: &[&str]) -> HashMap<String, String> {
    let mut dict = HashMap::new();

    for chunk in args.chunks_exact(2) {
        dict.insert(
            chunk[0].trim().to_string(),
            chunk[1].trim().to_string(),
        );

        if !valid_args.contains(&chunk[0].as_str())
        {
            panic!("Unknown CLI arguments: {} {}", chunk[0], chunk[1]);
        }
    }

    dict
}