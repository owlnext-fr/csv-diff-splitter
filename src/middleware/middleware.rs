use std::{path::PathBuf, collections::{hash_map::DefaultHasher, HashMap}, hash::{Hash, Hasher}};

use crate::{config::config_options::ConfigOptions, csv_utils::loader::load_data_from_csv};

/// processes the middleware program.
pub fn process(source_path: PathBuf, target_path: PathBuf, config: ConfigOptions, output_path: PathBuf) {
    // load CSV data with configuration

    let source_content = load_data_from_csv(source_path.as_path(), config.separator.as_str(), config.has_headers);
    let target_content = load_data_from_csv(target_path.as_path(), config.separator.as_str(), config.has_headers);

    debug!("Source lines: {}", source_content.len());
    debug!("Target lines: {}", target_content.len());

    // load keys into vector
    let mut source_identifiers: Vec<u64> = vec![];
    let mut target_identifiers: Vec<u64> = vec![];

    let mut source_hashed_lines: HashMap<u64, u64> = HashMap::new();
    let mut target_hashed_lines: HashMap<u64, u64> = HashMap::new();

    // TODO: add a hash<key, line>

    let mut hasher = DefaultHasher::new();

    for line in source_content.iter() {
        for idx in config.id_index.iter() {
            line.get(*idx as usize).unwrap_or("").hash(&mut hasher);
        }

        let key = hasher.finish();

        source_identifiers.push(key);

        for idx in config.update_markers.iter() {
            line.get(*idx as usize).unwrap_or("").hash(&mut hasher);
        }

        source_hashed_lines.insert(key, hasher.finish());
    }

    for line in target_content.iter() {
        for idx in config.id_index.iter() {
            line.get(*idx as usize).unwrap_or("").hash(&mut hasher);
        }

        let key = hasher.finish();

        target_identifiers.push(key);

        for idx in config.update_markers.iter() {
            line.get(*idx as usize).unwrap_or("").hash(&mut hasher);
        }

        target_hashed_lines.insert(key, hasher.finish());
    }

    // process new lines
    // process deleted lines
    // process updated lines
    // make treatments async
    // generate outpout CSVs
}