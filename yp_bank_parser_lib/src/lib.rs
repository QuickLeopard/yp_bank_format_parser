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

        if !valid_args.contains(&chunk[0].as_str()) {
            panic!("Unknown CLI arguments: {} {}", chunk[0], chunk[1]);
        }
    }

    dict
}

#[cfg(test)]
mod tests {

    use crate::{extract_format, parse_cli_args};

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
