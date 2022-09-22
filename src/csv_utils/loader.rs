use std::{path::Path, fs::File};

use csv::StringRecord;

use crate::cli_utils::error_codes;

/// loads data from a CSV file given by `file_path` with given separator and header options.
/// 
/// It will panic if the file cannot be opened.
pub fn load_data_from_csv(file_path: &Path, separator: &str, has_headers: bool) -> Vec<StringRecord> {
    let input: File;

    match File::open(file_path) {
        Ok(i) => input = i.try_clone().unwrap(),
        Err(_) => {
            crate::crash!(format!("Canonot read content of {}.", file_path.as_os_str().to_string_lossy()), error_codes::ERROR_FILE_UNREADABLE);
        },
    };

    let mut rdr = csv::ReaderBuilder::new()
    .delimiter(*separator.as_bytes().get(0).unwrap())
    .has_headers(has_headers)
    .from_reader(input);

    let mut content: Vec<StringRecord> = vec![];

    for result in rdr.records() {
        content.push(result.unwrap());
    }

    content
}